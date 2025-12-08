# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **BREAKING**: Upgraded utoipa from 4.x to 5.4.0 with OpenAPI 3.1 support
- **BREAKING**: Upgraded utoipa-swagger-ui from 6.x to 9.0.2
- **BREAKING**: Upgraded sea-orm from 0.12.x to 1.1.19
- **BREAKING**: Upgraded sea-orm-migration from 0.12.x to 1.1.19
- Updated various transitive dependencies for security and compatibility

### Added
- `UPGRADE_PLAN.md` - Comprehensive documentation of dependency upgrade strategy
- `CHANGELOG.md` - This file, for tracking changes and releases

### Notes
- No source code changes required - all existing API patterns remain compatible
- Full backward compatibility maintained for existing database operations
- OpenAPI 3.1 support now available for future API documentation enhancements

## [0.3.0] - 2024-12-08

### Added
- Initial release of http-api-rs REST API template
- Actix Web framework integration
- SeaORM database layer with SQLite support
- Utoipa/Swagger UI for OpenAPI documentation
- Health check endpoint with database connectivity validation
- Database migrations using SeaORM migration system
- Comprehensive CLAUDE.md development guide
- Example entity (Audit table) with database operations

### Features
- REST API server listening on configurable port (default: 9090)
- OpenAPI 3.0 documentation via Swagger UI at `/swagger-ui/`
- Health check endpoint at `/api/health` with API and database status
- Environment variable configuration for port and database URL
- Structured logging with env_logger
- Unit tests with actix-web test utilities
- Linting support with cargo clippy

### Dependencies
- actix-web 4.x - Web framework
- sea-orm 0.12.x - Async ORM
- utoipa 4.x - OpenAPI code generation
- serde - JSON serialization
- log/env_logger - Logging

[Unreleased]: https://github.com/daniel-samson/http-api-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/daniel-samson/http-api-rs/releases/tag/v0.3.0
