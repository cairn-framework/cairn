---
node: tasks.auth
---

# Auth Contract

The Auth module validates tokens and hashes passwords.

## Operations

- `verify_token(token)` - Return true if token is valid
- `hash_password(password)` - Return hashed password string
