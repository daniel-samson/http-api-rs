# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

http-api-rs is a REST API template built in Rust using Actix Web. It demonstrates best practices for structuring a production-ready API with database integration, OpenAPI documentation, and health checking.

## Essential Commands

### Development
- `make dev` - Run the dev server with an in-memory SQLite database (listens on port 9090)
- `cargo watch -x 'run'` - Watch for file changes and auto-restart the server (requires `cargo install cargo-watch`)
- `make test` - Run all tests
- `make test -- --test-threads=1` - Run tests sequentially (useful for debugging)

### Linting & Formatting
- `make lint` - Run cargo clippy to check code quality
- `make fix` - Auto-fix clippy warnings

### Database Management
- `make migrate` - Create SQLite database and apply all migrations
- `make cleandb` - Delete the SQLite database file
- `make entities` - Regenerate SeaORM entity files from the database schema

### Building & Deployment
- `cargo build --release` - Build optimized release binary
- `make start` - Run the release binary
- `make health` - Health check the running server

### Environment Variables
- `PORT` - Server port (default: 9090)
- `DATABASE_URL` - Database connection string (default: `sqlite::memory:`)
- `RUST_LOG` - Log level (see [env_logger docs](https://docs.rs/env_logger/latest/env_logger/))

## Architecture

### Core Structure

**Main Application** (`src/main.rs`)
- Sets up Actix Web server with Logger middleware
- Mounts API routes under `/api` scope
- Configures OpenAPI/Swagger UI at `/swagger-ui/`
- Root route `/` returns "Hello World!"

**Modules**
- `src/env.rs` - Environment variable parsing and defaults
- `src/health.rs` - Health check endpoint that validates API and database connectivity
- `src/entities/` - SeaORM entity models (auto-generated from migrations)

### Database Layer

**ORM**: SeaORM (`sea_orm` v1.1.19)
- Supports SQLite, MySQL, PostgreSQL via feature flags
- Entities are auto-generated from migrations
- **Version 1.x**: Major upgrade from 0.12.x with enhanced query capabilities, refined insertion workflows, and improved partial model functionality

**Migrations**: Located in `migration/` workspace member
- Uses SeaORM migration system
- Currently has one migration: `m20220101_000001_create_audit_table`
- To add migrations:
  1. Run `cargo run -- migrate generate MIGRATION_NAME` in the `migration/` directory
  2. Edit the generated file to define schema changes
  3. Run `make migrate` to apply

**Health Check Implementation** (`src/health.rs`)
- Verifies API connectivity (always Operational)
- Checks database connection and queries (can be Critical or Deminished)
- The health endpoint performs an actual database write to the audit table to validate full connectivity

### API Documentation

**Swagger/OpenAPI**
- Generated via Utoipa decorators on handler functions
- Accessible at `http://localhost:9090/swagger-ui/`
- Update by adding/modifying `#[utoipa::path(...)]` attributes on route handlers
- Add new schemas to the `ApiDocV1` struct's `components(schemas(...))`

## Key Implementation Patterns

### Adding a New Route
1. Create a handler function in a new module (or existing module)
2. Add `#[utoipa::path(...)]` decorator for OpenAPI documentation
3. Register the handler in route configuration (e.g., in a `config()` function)
4. Mount the config in `main.rs` under the `/api` scope

### Adding a New Database Entity
1. Create a migration in `migration/src/` with schema definition
2. Run `make migrate` to apply the migration
3. Run `make entities` to auto-generate Rust entity files
4. Import entities from `crate::entities::{prelude::*, *}` and use them in handlers

### Error Handling
- Database errors use `sea_orm::DbErr`
- HTTP responses use Actix Web's `HttpResponse` builder
- Logging uses the standard `log` crate (initialized via `env_logger::init()`)

## Testing

- Tests are co-located with source code using `#[cfg(test)]` modules
- Use `#[actix_web::test]` attribute for async test functions
- Import test utilities from `actix_web::test`
- Run specific tests: `cargo test --lib health` or `cargo test test_name`

## Dependencies

**Key Crates**
- `actix-web` (4.4.0) - Web framework
- `sea-orm` (1.1.19) - ORM for database operations
- `utoipa` (5.4.0) - OpenAPI/Swagger code generation with OpenAPI 3.1 support
- `utoipa-swagger-ui` (9.0.2) - Swagger UI integration
- `serde` (1.0.228) - Serialization/deserialization for JSON
- `log` / `env_logger` (0.11.0) - Logging infrastructure

See `Cargo.toml` for complete dependency list and versions.

## Common Workflows

### Running Development Server with Database
```bash
make migrate  # First time setup
make dev      # Runs server with DATABASE_URL=sqlite:./sqlite.db
```

### Inspecting the Database
SQLite file is stored at `./sqlite.db` in the project root when using `make dev`.

### Debugging a Single Test
```bash
RUST_LOG=debug cargo test test_name -- --nocapture --test-threads=1
```

### Starting Fresh
```bash
make cleandb
make migrate
make dev
```
