# seagull

<div align="center">
  <!-- Github Actions -->
  <img src="https://github.com/Wahuh/seagull/workflows/Continuous%20integration/badge.svg" />
</div>

## Getting Started

The basic workflow is as follows:

1. `seagull init`

```
# seagull.toml
```

## Commands

### seagull init
creates a `seagull.toml` file for storing connection strings and other config values. This file is optional, you can use the other commands without it.

### seagull poop
firstly creates a directory named `migrations` if one does not already exist. Secondly creates an empty `.sql` file in the format `V{1}__{2}.sql` where `{1}` is an auto-incremented version number and `{2}` is a description.

```bash
USAGE
# creates ./migrations/V1__initial.sql
$ seagull poop initial

# creates ./migrations/V1_create_users_table.sql 
$ seagull poop "create users table"

# creates ./migrations/V2_another_migration.sql assuming V1 exists
$ seagull poop another_migration
```

### seagull migrate
firstly creates a database table named `__migration_history` if one does not already exist. Runs all migrations in the `migrations` directory in a single transaction. If one fails, they all fail and the database is rolled back.

```bash
USAGE

# reads config from seagull.toml
$ seagull migrate

# specify your PostgreSQL connection string
$ seagull migrate --database postgresql://postgres:mysecretpassword@localhost/postgres

# looks for migrations in src/migrations
$ seagull migrate --dir src/migrations
```

### seagull remigrate
Same as `seagull migrate` except that it will firstly reset the whole database before running all migrations. Useful for development if you're using a Docker database and changing migrations often. **Would NOT suggest running it on production!** :skull_and_crossbones:

```bash
# reads config from seagull.toml
$ seagull remigrate

# specify your PostgreSQL connection string
$ seagull remigrate --database postgresql://postgres:mysecretpassword@localhost/postgres

# looks for migrations in src/migrations
$ seagull remigrate --dir src/migrations
```