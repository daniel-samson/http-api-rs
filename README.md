http-api-rs
====

Template project for creating REST API's in rust

## Features

- Actix Web Server
- OpenAPI (SwaggerUI) Documentation
- SeaQL ORM
- [Log](https://docs.rs/log/latest/log/index.html) with [env_logger](https://docs.rs/env_logger/0.9.1/env_logger/)

## Project Status

[![CI](https://github.com/daniel-samson/http-api-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/daniel-samson/http-api-rs/actions/workflows/ci.yml)

## Development

### Pre-requisites
Please install the following packages:

```bash
cargo install sea-orm-cli
cargo install cargo-watch
```

## Watch for changes

it is recommended that you use [cargo-watch](https://github.com/passcod/cargo-watch):

```bash
cargo watch -x 'run'
```

## Environment variables
Please check [src/env.rs](src/env.rs) for a list of environment variables.

### Set up database
To create database run:

```bash
# make migrate
```

## Dev server
Run:

```bash
# make dev
```

You should be able to access the in the browser [http://localhost:9090](http://localhost:9090)

### Swagger Docs
You should be able to access the in the browser [http://localhost:9090/swagger-ui/](http://localhost:9090/swagger-ui/)

### Healthcheck
To check the servers health, run:

```bash
# make health
```

you should see successfull response eg
```json
{
    "rest_api":"Operational",
    "database":"Operational"
}
```

To troubleshoot any issues. Please see the stdout from the dev server.

## Lint

Before making pull requests, you must use cargo clippy to check that your code meets the coding standards. You can run:

```bash
# make lint
```

or to automatically fix style issues run:

```bash
# make fix
```


## Documentation

- [SeaQL ORM Tutorial](https://www.sea-ql.org/sea-orm-tutorial/ch00-00-introduction.html)
- [SeaQL ORM Docs](https://www.sea-ql.org/SeaORM/docs/index/)
- [Utoipa Docs - swagger generation](https://docs.rs/utoipa/latest/utoipa/)
