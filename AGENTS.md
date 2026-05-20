# AGENTS.md — Guidance for AI coding agents

Purpose: Give concise, actionable guidance so an AI agent can quickly work on this Rust web service.

Quick commands
- Build: `cargo build`
- Run: `cargo run` (server uses `server_port` from env)
- Test: `cargo test`
- Format (optional): `cargo fmt`

Environment
- Uses `dotenvy` + `envy` to load configuration. Provide a `.env` with the following vars:
  - `server_port`
  - `db_host`, `db_port`, `db_username`, `db_password`, `db_name`
  - `db_max_connections`, `db_min_connections`, `db_acquire_timeout_sec`, `db_idle_timeout_sec`

Key files & patterns
- Router / app entry: [src/lib.rs](src/lib.rs) and [src/main.rs](src/main.rs)
- Configuration & DB pool: [src/config.rs](src/config.rs)
- Central error type (maps to HTTP responses): [src/app_error.rs](src/app_error.rs)
- API surface: [src/api/mod.rs](src/api/mod.rs)
- Example endpoint: [src/api/helloworld/helloworld.rs](src/api/helloworld/helloworld.rs)

Conventions & notes
- Error handling: use `AppError` (implements `IntoResponse`) for HTTP errors; wrap internal errors with `anyhow` and convert to `AppError::Other` when appropriate.
- DB: uses `sqlx` with Postgres. `init_pool` is in `src/config.rs` and expects the env vars above.
- New endpoints: add a module under `src/api/…` and register the route in `src/api/mod.rs` (see `/api/hello`).
- Keep changes minimal and run `cargo build` and `cargo test` after edits.

When to ask for help
- If a change depends on a database migration or a live DB, request DB credentials or a test DB.

Further customization
- If you'd like, I can also create a `.github/copilot-instructions.md` with narrower guidance for PRs and reviews.
