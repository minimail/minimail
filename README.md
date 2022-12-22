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

### Authenticated Requests

The endpoint for retrieving subscribers is locked behind an admin token that is set through the env. Set the env variable `ADMIN_TOKEN` to whatever value you want and just ensure you pass it whenever you are accessing the endpoint inside of the `Authorization` header, using `Bearer <ADMIN_TOKEN>`.
