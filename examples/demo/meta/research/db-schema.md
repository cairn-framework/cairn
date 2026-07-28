---
id: res.db-schema
nodes:
  - tasks.db
date: 2026-05-22
sources: [src.sqlite-when-to-use]
---

# Database Schema Research

Two tables: tasks and users.

```sql
CREATE TABLE tasks (id INTEGER PRIMARY KEY, title TEXT);
CREATE TABLE users (id INTEGER PRIMARY KEY, email TEXT);
```
