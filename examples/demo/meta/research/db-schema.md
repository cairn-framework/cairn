---
node: tasks.db
---

# Database Schema Research

Two tables: tasks and users.

```sql
CREATE TABLE tasks (id INTEGER PRIMARY KEY, title TEXT);
CREATE TABLE users (id INTEGER PRIMARY KEY, email TEXT);
```
