# Proposal: Add auth middleware

## Motivation

The API currently has no authentication layer. This change adds token-based middleware.

## Scope

- Add verify_token call to every protected endpoint
- Update tests to include auth headers

## Out of scope

- OAuth integration
- Password reset flow
