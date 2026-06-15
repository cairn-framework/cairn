//! Minimal SSE (Server-Sent Events) consumer for Gas City integration spikes.
//!
//! Parses `text/event-stream` into typed events. No external HTTP client
//! dependency: the caller provides a `BufRead` over the response body.

use std::io::{BufRead, Error as IoError};

/// A single SSE event parsed from the stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseEvent {
    /// Event type (e.g. `request.result`, `bead.created`).
    pub event: String,
    /// Event payload, usually JSON.
    pub data: String,
    /// Optional event ID.
    pub id: Option<String>,
}

/// Error parsing an SSE stream.
#[derive(Debug)]
pub enum SseError {
    /// Underlying I/O failure.
    Io(IoError),
    /// UTF-8 decode failure in a line.
    Utf8(std::str::Utf8Error),
}

impl std::fmt::Display for SseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "sse io error: {e}"),
            Self::Utf8(e) => write!(f, "sse utf-8 error: {e}"),
        }
    }
}

impl std::error::Error for SseError {}

impl From<IoError> for SseError {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

/// Parse SSE events from a buffered reader.
///
/// # Errors
///
/// Returns `SseError::Io` on read failure or `SseError::Utf8` on invalid UTF-8.
pub fn parse_events<R: BufRead>(reader: &mut R) -> Result<Vec<SseEvent>, SseError> {
    let mut events = Vec::new();
    let mut current_event = String::new();
    let mut current_data = String::new();
    let mut current_id = None;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            // Blank line terminates the event.
            if !current_event.is_empty() || !current_data.is_empty() {
                events.push(SseEvent {
                    event: current_event,
                    data: current_data,
                    id: current_id,
                });
                current_event = String::new();
                current_data = String::new();
                current_id = None;
            }
            continue;
        }

        if let Some(value) = line.strip_prefix("event:") {
            // Reason: converting &str to String; clone_into not applicable on &str.
            #[allow(clippy::assigning_clones)]
            {
                current_event = value.trim_start().to_owned();
            }
        } else if let Some(value) = line.strip_prefix("data:") {
            if !current_data.is_empty() {
                current_data.push('\n');
            }
            current_data.push_str(value.trim_start());
        } else if let Some(value) = line.strip_prefix("id:") {
            // Reason: converting &str to String; clone_into not applicable on &str.
            #[allow(clippy::assigning_clones)]
            {
                current_id = Some(value.trim_start().to_owned());
            }
        }
        // Ignore unknown fields (retry:, comments, etc.).
    }

    // trailing event without terminating blank line
    if !current_event.is_empty() || !current_data.is_empty() {
        events.push(SseEvent {
            event: current_event,
            data: current_data,
            id: current_id,
        });
    }

    Ok(events)
}

/// Open a plain HTTP connection to an SSE endpoint and return a buffered
/// reader positioned at the start of the response body.
///
/// Sends `GET <path> HTTP/1.0` with `Accept: text/event-stream`.  HTTP/1.0 is
/// used deliberately so servers that would otherwise use chunked
/// transfer-encoding fall back to a plain body stream, which [`parse_events`]
/// can consume directly.
///
/// Only `http://` URLs are supported; HTTPS requires a TLS layer not provided
/// here.
///
/// # Errors
///
/// Returns `SseError::Io` when the URL is invalid, the TCP connection fails,
/// or the server returns a non-200 HTTP status.
pub fn connect(url: &str) -> Result<std::io::BufReader<std::net::TcpStream>, SseError> {
    use std::io::{BufRead as _, Write as _};

    let (host, port, path) = parse_http_url(url)?;
    let stream = std::net::TcpStream::connect((host.as_str(), port)).map_err(SseError::Io)?;

    // Write the GET request via a reference; ownership of `stream` stays here.
    (&stream)
        .write_all(
            format!("GET {path} HTTP/1.0\r\nHost: {host}\r\nAccept: text/event-stream\r\n\r\n")
                .as_bytes(),
        )
        .map_err(SseError::Io)?;

    let mut reader = std::io::BufReader::new(stream);

    // Validate the status line.
    let mut status_line = String::new();
    reader.read_line(&mut status_line).map_err(SseError::Io)?;
    if !status_line.contains("200") {
        return Err(SseError::Io(std::io::Error::other(format!(
            "unexpected HTTP status: {}",
            status_line.trim()
        ))));
    }

    // Consume remaining headers until the blank separator line.
    let mut header = String::new();
    loop {
        header.clear();
        reader.read_line(&mut header).map_err(SseError::Io)?;
        if header == "\r\n" || header == "\n" || header.is_empty() {
            break;
        }
    }

    Ok(reader)
}

/// Parse an `http://` URL into `(host, port, path)`.
///
/// # Errors
///
/// Returns `SseError::Io` when the URL is not a valid `http://` URL or the
/// port is not a valid `u16`.
fn parse_http_url(url: &str) -> Result<(String, u16, String), SseError> {
    let rest = url.strip_prefix("http://").ok_or_else(|| {
        SseError::Io(IoError::new(
            std::io::ErrorKind::InvalidInput,
            "URL must start with http://",
        ))
    })?;

    let (authority, path) = rest.find('/').map_or_else(
        || (rest, "/".to_owned()),
        |slash| (&rest[..slash], rest[slash..].to_owned()),
    );

    let (host, port) = if let Some(colon) = authority.rfind(':') {
        let port: u16 = authority[colon + 1..].parse().map_err(|_| {
            SseError::Io(IoError::new(
                std::io::ErrorKind::InvalidInput,
                "invalid port in URL",
            ))
        })?;
        (&authority[..colon], port)
    } else {
        (authority, 80_u16)
    };

    Ok((host.to_owned(), port, path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_parse_single_event() {
        let input = "event: request.result\ndata: {\"status\":\"ok\"}\n\n";
        let mut reader = BufReader::new(input.as_bytes());
        let events = parse_events(&mut reader).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "request.result");
        assert_eq!(events[0].data, "{\"status\":\"ok\"}");
        assert_eq!(events[0].id, None);
    }

    #[test]
    fn test_parse_multiple_events() {
        let input = concat!(
            "event: bead.created\n",
            "id: 1\n",
            "data: {\"id\":\"bd-001\"}\n",
            "\n",
            "event: bead.updated\n",
            "id: 2\n",
            "data: {\"id\":\"bd-001\"}\n",
            "\n",
        );
        let mut reader = BufReader::new(input.as_bytes());
        let events = parse_events(&mut reader).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event, "bead.created");
        assert_eq!(events[0].id, Some("1".to_owned()));
        assert_eq!(events[1].event, "bead.updated");
        assert_eq!(events[1].id, Some("2".to_owned()));
    }

    #[test]
    fn test_parse_multiline_data() {
        let input = concat!(
            "event: message\n",
            "data: line one\n",
            "data: line two\n",
            "\n",
        );
        let mut reader = BufReader::new(input.as_bytes());
        let events = parse_events(&mut reader).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data, "line one\nline two");
    }

    #[test]
    fn test_parse_ignores_comments_and_retry() {
        let input = concat!(
            ":heartbeat\n",
            "retry: 5000\n",
            "event: ping\n",
            "data: pong\n",
            "\n",
        );
        let mut reader = BufReader::new(input.as_bytes());
        let events = parse_events(&mut reader).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "ping");
        assert_eq!(events[0].data, "pong");
    }

    #[test]
    fn test_parse_empty_stream() {
        let input = "";
        let mut reader = BufReader::new(input.as_bytes());
        let events = parse_events(&mut reader).unwrap();
        assert!(events.is_empty());
    }

    // --- connect / parse_http_url tests ---

    #[test]
    fn test_parse_http_url_host_and_path() {
        let (host, port, path) = parse_http_url("http://example.com/events").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 80);
        assert_eq!(path, "/events");
    }

    #[test]
    fn test_parse_http_url_custom_port() {
        let (host, port, path) = parse_http_url("http://127.0.0.1:9000/stream").unwrap();
        assert_eq!(host, "127.0.0.1");
        assert_eq!(port, 9000);
        assert_eq!(path, "/stream");
    }

    #[test]
    fn test_parse_http_url_no_path_defaults_to_slash() {
        let (host, port, path) = parse_http_url("http://example.com").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 80);
        assert_eq!(path, "/");
    }

    #[test]
    fn test_parse_http_url_rejects_non_http() {
        assert!(parse_http_url("https://example.com/events").is_err());
        assert!(parse_http_url("ws://example.com/events").is_err());
    }

    #[test]
    fn test_connect_returns_events_from_mock_server() {
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                // Drain request headers so the OS can close with FIN, not RST.
                let mut req = std::io::BufReader::new(&stream);
                let mut line = String::new();
                loop {
                    line.clear();
                    req.read_line(&mut line).unwrap_or(0);
                    if line == "\r\n" || line == "\n" || line.is_empty() {
                        break;
                    }
                }
                let response = concat!(
                    "HTTP/1.0 200 OK\r\n",
                    "Content-Type: text/event-stream\r\n",
                    "\r\n",
                    "event: ping\r\n",
                    "data: pong\r\n",
                    "\r\n",
                );
                let _ = std::io::Write::write_all(&mut (&stream), response.as_bytes());
            }
        });

        let url = format!("http://127.0.0.1:{port}/events");
        let mut reader = connect(&url).unwrap();
        let events = parse_events(&mut reader).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "ping");
        assert_eq!(events[0].data, "pong");
    }

    #[test]
    fn test_connect_errors_on_non_200_status() {
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                // Drain request so OS closes with FIN.
                let mut req = std::io::BufReader::new(&stream);
                let mut line = String::new();
                loop {
                    line.clear();
                    req.read_line(&mut line).unwrap_or(0);
                    if line == "\r\n" || line == "\n" || line.is_empty() {
                        break;
                    }
                }
                let _ =
                    std::io::Write::write_all(&mut (&stream), b"HTTP/1.0 404 Not Found\r\n\r\n");
            }
        });

        let url = format!("http://127.0.0.1:{port}/missing");
        let err = connect(&url).unwrap_err();
        assert!(
            err.to_string().contains("404"),
            "error should mention status code, got: {err}"
        );
    }
}
