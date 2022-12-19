## Minimail

A simple mailing list service.

### Database

To begin, set `DATABASE_URL` env to `sqlite:minimail.db`.

The database can be setup by running:
```
cargo sqlx create database
cargo sqlx migrate run
```

New migrations can be created with:
```
cargo sql migrate add <name_of_migration>
```
