# Todos REST API Example

A REST API demonstrating structural typing with a SQLite database backend. Shows how structural types enable flexible, context-specific field requirements in a real-world application.

## Prerequisites

- Rust toolchain
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) with SQLite support:
  ```bash
  cargo install sqlx-cli --no-default-features --features sqlite
  ```

## Setup

All commands should be run from the **workspace root** (`structural-typing-rs/`), not from the example directory.

### 1. Create `.env` file

Create a `.env` file in the workspace root:

```bash
echo "DATABASE_URL=sqlite:examples/todos-api/todos.db" > .env
```

### 2. Create and migrate database

```bash
sqlx database create
sqlx migrate run --source examples/todos-api/migrations
```

## Running

From the workspace root:

```bash
cargo run -p todos-api
```

The server will start on `http://localhost:3000`.

## Verify

Test the API:

```bash
curl http://localhost:3000/todos
```

## Building

The example uses sqlx's compile-time query verification. Offline query metadata is committed in `.sqlx/`, allowing builds without a live database connection.

To rebuild query metadata after changing SQL queries:

```bash
cargo sqlx prepare --workspace
```
