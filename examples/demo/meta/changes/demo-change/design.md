# Design: Auth middleware

## Approach

Extract auth logic into middleware that runs before route handlers.

## Changes

ADDED:
- `src/auth/middleware.rs` - Token validation middleware

MODIFIED:
- `src/api/lib.rs` - Apply middleware to protected endpoints
