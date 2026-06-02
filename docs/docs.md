# 🦔 HyraxQL Usage Guide

> **HyraxQL** is a lightning-fast, lightweight database explorer for the modern terminal — a simple, universal interface bridging heavy GUI clients and restrictive raw SQL shells.

---

## Global Options

| Flag | Description |
|------|-------------|
| `--verbose` / `-v` | Enable verbose output. Works with any command for detailed execution info. |

---

## Commands

### `connect`

Establishes a connection to a specified database.

#### Options

| Flag | Shorthand | Required | Default | Description |
|------|-----------|----------|---------|-------------|
| `--type <DB_TYPE>` | `-t` | ✅ Yes | — | Database type: `postgres`, `mysql`, `mariadb`, `sqlite` |
| `--db <DATABASE_NAME>` | `-d` | ⚠️ Non-SQLite | — | Database name (or file path / `:memory:` for SQLite) |
| `--user <USERNAME>` | `-u` | ❌ No | `sqlite` | Username for authentication |
| `--host <HOST>` | `-h` | ❌ No | `localhost` | Database server host |
| `--port <PORT>` | `-p` | ❌ No | `5432` | Connection port |
| `--pw <PASSWORD>` | `-w` | ❌ No | — | Password for authentication |

#### Connection String Format

```
# SQLite
sqlite://<DATABASE_NAME>

# With password
<DB_TYPE>://<USER>:<PASSWORD>@<HOST>:<PORT>/<DATABASE_NAME>

# Without password
<DB_TYPE>://<USER>@<HOST>:<PORT>/<DATABASE_NAME>
```

#### Examples

```bash
# PostgreSQL
connect -t postgres -u myuser -d mydb -h localhost -p 5432 -w mypassword

# In-memory SQLite
connect -t sqlite -d :memory:

# MySQL (no password)
connect -t mysql -u root -d app_db -h 127.0.0.1 -p 3306
```

---

### `explore`

Browse the connected database — list tables or inspect data and schemas.

#### Options

| Flag | Shorthand | Description |
|------|-----------|-------------|
| `--table <TABLE_NAME>` | `-t` | Table to explore. Omit to list all tables. |
| ┣━ `--columns` | `-c` | *(requires `-t`)* Show column names and data types instead of row data. |
| ┗━ `--size <ROWS>` | `-s` | *(requires `-t`)* Number of rows to display. Default: `25`. |

> ⚠️ **`--columns` and `--size` cannot be used together.** Use one or the other.

#### Quick Reference

| Goal | Command |
|------|---------|
| List all tables | `explore` |
| View table data (25 rows) | `explore -t <table>` |
| View table data (custom rows) | `explore -t <table> -s <n>` |
| View table schema | `explore -t <table> -c` |

#### Examples

```bash
# List all tables
explore

# First 25 rows of `users`
explore -t users

# First 10 rows of `products`
explore -t products -s 10

# Schema of `orders`
explore -t orders -c
```

---

### `clear`

Clears the terminal screen.

```bash
clear
```

---

### `disconnect`

Closes the current database connection.

```bash
disconnect
```

---

### `exit`

Exits the HyraxQL session.

> ⚠️ If still connected to a database, you will be prompted to disconnect first.

```bash
exit
```