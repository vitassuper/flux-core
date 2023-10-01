# Using Diesel CLI with `--database-url`

## Install

  ```sh
  cargo install diesel_cli --no-default-features --features "postgres"  # For PostgreSQL
  ```

## Setup

  ```sh
  diesel setup --database-url=postgres://username:password@localhost/database  # For PostgreSQL
  ```

## Usage

### Running Migrations

To apply pending migrations to your database, use the following command:

```sh
diesel migration run --database-url=postgres://username:password@localhost/database
```

### Rolling Back Migrations

To rollback the last applied migration, use the following command:

```sh
diesel migration redo --database-url=postgres://username:password@localhost/database
```

### Generating Models and Schema

Diesel can generate Rust code for your database schema and models. To generate the code, use:

```sh
diesel print-schema --database-url=postgres://username:password@localhost/database > src/schema.rs
```
