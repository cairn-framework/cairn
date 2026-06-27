---
node: cairn.sse
---

# Contract: cairn.sse

## Purpose

Minimal Server-Sent Events (SSE) consumer for Gas City integration spikes. It
parses a `text/event-stream` body into typed events and can open a plain HTTP
connection to an SSE endpoint. It carries no external HTTP client dependency:
the caller supplies a `BufRead` over the response body, or uses the built-in
HTTP/1.0 `connect` helper.

## Public interface

- `SseEvent`: a parsed event with `event: String`, `data: String`, and
  `id: Option<String>`. Derives `Debug, Clone, PartialEq, Eq`.
- `SseError`: parse error enum with `Io(IoError)` and `Utf8(Utf8Error)` variants;
  implements `Display`, `std::error::Error`, and `From<IoError>`.
- `parse_events<R: BufRead>(reader) -> Result<Vec<SseEvent>, SseError>`: parses
  `event:`, `data:`, and `id:` fields, treating a blank line as an event
  terminator.
- `connect(url) -> Result<BufReader<TcpStream>, SseError>`: opens an `http://`
  endpoint with `GET <path> HTTP/1.0` and `Accept: text/event-stream`, returning
  a reader positioned at the body.

## Invariants

- Multiple `data:` lines accumulate into one payload joined by newlines.
- Unknown fields (`retry:`, comments) are ignored, not errors.
- A trailing event with no terminating blank line is still emitted.
- Empty events (no event and no data) are never pushed.
- HTTP/1.0 is used deliberately so servers avoid chunked transfer-encoding;
  only `http://` is supported (HTTPS needs a TLS layer not provided here).
- A non-200 HTTP status from `connect` yields `SseError::Io`.

## Dependencies

Leaf with no outgoing blueprint edges. Depends only on the standard library
(`std::io::BufRead`, `std::net::TcpStream`). The private `parse_http_url` helper
splits an `http://` URL into `(host, port, path)`.

## Tests

Unit tests in the `#[cfg(test)]` module at the bottom of `src/sse.rs` cover
single and multi-event parsing, multi-line `data:` accumulation, id capture,
unknown-field skipping, trailing events without a blank line, and `parse_http_url`
parsing.
