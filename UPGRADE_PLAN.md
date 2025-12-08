# Comprehensive Upgrade Plan: utoipa & sea-orm

## Current Status
- **Branch**: `upgrade/utoipa-seaorm-latest`
- **Rust**: 1.91.1 (stable)

## Target Versions

| Crate | Current | Target | Status |
|-------|---------|--------|--------|
| utoipa | 4.x | 5.4.0 | Major version upgrade |
| utoipa-swagger-ui | 6.x | 9.0.2 | Major version upgrade |
| sea-orm | ^0.12.12 | 1.1.19 | Major version upgrade |
| sea-orm-migration | ^0.12.12 | 1.1.19 | Major version upgrade |

## Key Changes Overview

### utoipa 4.x → 5.x Breaking Changes

1. **OpenAPI 3.1 Migration**
   - Full support for OpenAPI 3.1 specification
   - Schema definitions may have changed structures
   - No code changes needed for basic usage with #[utoipa::path(...)]

2. **Enum Processing Refactor**
   - Internal changes only - no API changes needed
   - Enums should continue to work as before

3. **Schema Collection Improvements**
   - Automatic schema collection for requests improved
   - Better handling of complex types

4. **Operations Implementation Changes**
   - Internal refactor - existing path macros continue to work

5. **Required Schema References**
   - Schemas are now properly marked as required
   - May improve OpenAPI validation

### utoipa-swagger-ui 6.x → 9.x Breaking Changes

1. **Version 9.0.0 (January 2025)**
   - Re-released as major due to axum upgrade compatibility
   - For actix-web: feature `actix-web` continues to work unchanged

2. **Recent Features (9.0.1, 9.0.2)**
   - Better optimization for API docs caching
   - Updated zip build dependency

### sea-orm 0.12.x → 1.x Breaking Changes

1. **API Restructuring**
   - Entity operation method signatures may have changed
   - Return type structures for database operations
   - Query construction patterns for complex joins

2. **Major Features (1.0+)**
   - Enhanced query capabilities with nested model support
   - Refined insertion workflows for batch operations
   - Expanded partial model functionality
   - Improved JSON serialization for ActiveModel

3. **Code Impact Areas**
   - Database insert/update operations
   - Query construction patterns
   - Error handling for permission scenarios

## Implementation Plan

### Phase 1: Dependency Updates (Cargo.toml)
- [x] Research latest versions
- [ ] Update main Cargo.toml to:
  - utoipa = "5.4"
  - utoipa-swagger-ui = "9"
  - sea-orm = "1.1"
  - sea-orm-migration = "1.1"
- [ ] Update migration/Cargo.toml sea-orm-migration to "1.1"

### Phase 2: Code Migration (if needed)
- [ ] Review src/main.rs for utoipa usage:
  - #[openapi(...)] macro - should work unchanged
  - SwaggerUi::new() - check if API changed
- [ ] Review src/health.rs for sea-orm usage:
  - Database::connect() - check if return type changed
  - Entity operations (insert, query) - verify API
  - ActiveModel usage - verify struct changes
- [ ] Test individual components

### Phase 3: Build & Testing
- [ ] Run `cargo build` to identify compilation errors
- [ ] Fetch documentation for breaking changes
- [ ] Fix errors based on compiler messages and documentation
- [ ] Run `cargo test` to ensure tests pass
- [ ] Run `make lint` to check code quality

### Phase 4: Documentation
- [ ] Update CLAUDE.md with new versions
- [ ] Document any API changes needed for future development
- [ ] Update README.md if needed

## Known Non-Issues

Based on changelog review, the following should NOT require code changes:
- Basic #[utoipa::path(...)] attribute usage
- Basic SwaggerUi::new() configuration with actix-web feature
- Standard sea-orm entity definitions via generated entities
- Health check implementation pattern (select/insert operations)

## Potential Issues to Watch For

1. **sea-orm 1.0 Migration**: Insertion and querying patterns may have changed
   - Current code uses: `Audit::insert(model).exec(&db).await?`
   - Need to verify this pattern still works in 1.x

2. **Async Runtime**: Verify "runtime-actix-rustls" feature is still valid in 1.x

3. **Schema Generation**: sea-orm-cli compatibility with 1.x CLI

## Testing Strategy

1. Build test: `cargo build --release`
2. Unit tests: `cargo test`
3. Linting: `cargo clippy`
4. Database migration: `make migrate`
5. Manual health check: Ensure `/api/health` endpoint works

## Rollback Plan

If critical issues arise:
```bash
git checkout main
git branch -D upgrade/utoipa-seaorm-latest
```

## Resources

- [utoipa 5.4.0 Documentation](https://docs.rs/utoipa/5.4.0/utoipa/)
- [sea-orm 1.1.x Documentation](https://docs.rs/sea-orm/1.1.19/sea_orm/)
- [utoipa GitHub Changelog](https://github.com/juhaku/utoipa/blob/master/utoipa/CHANGELOG.md)
- [sea-orm GitHub Changelog](https://github.com/SeaQL/sea-orm/blob/master/CHANGELOG.md)
