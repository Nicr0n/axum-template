# Axum Template

This is a web application template built with Axum and SeaORM.

## Quick Start

Basically, you can start the application by docker compose or manually.

### Docker compose

1. Setup environment variables
   - `cp .env.example .env.docker`
   - Edit `.env.docker` file

```
docker-compose up
```

### Manual start

1. Setup database and environment variables
   - Create a database

1. Setup environment variables
   - `cp .env.example .env`
   - Edit `.env` file

1. Setup migration

   ```
   cargo run --package migration -- -u
   ```

   you can check about more information using

   ```
   cargo run --package migration -- -h
   ```

1. Start the application
   use

   ```
   cargo start
   ```

   or

   ```
   cargo run --package app -- start
   ```

   to start the application
   you can check about more information using

   ```
   cargo run --package app -- -h
   ```

### Project Structure

- `app`: The axum application
  - `cmd` : The command line interface
  - `server/app.rs`: The axum application entry point
  - `docker`: The docker configuration
- `infra`: The infrastructure config and struct
- `migration`: The database migration
  - `cmd`: The command line interface for migrations
  - `migrations\*`: The database migrations file
