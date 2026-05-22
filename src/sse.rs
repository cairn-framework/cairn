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
}
