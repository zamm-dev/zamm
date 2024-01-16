# Using Diesel for database access

We will be following [these instructions](https://diesel.rs/guides/getting-started) for setup.

First, edit `Cargo.toml` to add the following dependencies:

```toml
[dependencies]
diesel = { version = "2.1.0", features = ["sqlite"] }
```

To connect to a DB, import

```rust
use diesel::Connection;
use diesel::sqlite::SqliteConnection;
```

Then

```rust
    match SqliteConnection::establish(db_path_str) {
        Ok(conn) => {
            println!("Connected to DB at {}", db_path_str);
            Some(conn)
        },
        Err(e) => {
            eprintln!("Failed to connect to DB: {}", e);
            None
        }
    }
```

## Migrations

First, install the CLI:

```bash
$ sudo apt install -y libpq-dev libmysqlclient-dev libsqlite3-dev
$ cargo install diesel_cli
```

If you are using `asdf`, you will also need to run

```bash
$ asdf reshim rust
```

You can alternately only install the client libraries for the database you are actually using, as described on the page.

We can start off by running

```bash
$ diesel setup
Creating migrations directory at: /root/zamm/src-tauri/migrations
The --database-url argument must be passed, or the DATABASE_URL environment variable must be set.
```

This gets us `diesel.toml`:

```toml
# For documentation on how to configure this file,
# see https://diesel.rs/guides/configuring-diesel-cli

[print_schema]
file = "src/schema.rs"
custom_type_derives = ["diesel::query_builder::QueryId"]

[migrations_directory]
dir = "migrations"
```

and an empty migrations directory. Now we populate it, let's say with a table for terminal command executions:

```toml
$ diesel migration generate create_executions
Creating migrations/2023-08-17-054802_create_executions/up.sql
Creating migrations/2023-08-17-054802_create_executions/down.sql
```

Edit `migrations/2023-08-17-054802_create_executions/up.sql`:

```sql
CREATE TABLE executions (
  id VARCHAR PRIMARY KEY,
  command TEXT,
  output TEXT
)
```

Edit `migrations/2023-08-17-054802_create_executions/down.sql`:

```sql
DROP TABLE executions
```

If our database is a local file that needs to be created on app startup, then it doesn't exist yet and we can't just create it on the user's local machine with `diesel migration run`. Instead, we'll create it in code.

Next, we add `diesel_migrations` as a dependency. Edit `Cargo.toml` to add:

```
diesel_migrations = { version = "2.1.0", features = ["sqlite"] }
```

Now, where appropriate, embed your migrations by pointing it to a path relative to your `Cargo.toml`:

```rust
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations/postgresql");
```

and run those migrations. For example:

```rust
fn run_migrations(connection: &mut impl MigrationHarness<DB>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
```

For another example, if you already have a function such as:

```rust
pub fn get_db() -> Option<SqliteConnection> {
  get_data_dir_db().or_else(|| {
      eprintln!("Unable to create DB in user data dir, defaulting to current dir instead.");
      connect_to(env::current_dir().expect("Failed to get current directory").join(DB_NAME))
  })
}
```

then change it to this:

```rust
pub fn get_db() -> Option<SqliteConnection> {
  let mut possible_connection = get_data_dir_db().or_else(|| {
      eprintln!("Unable to create DB in user data dir, defaulting to current dir instead.");
      connect_to(env::current_dir().expect("Failed to get current directory").join(DB_NAME))
  });
  if let Some(connection) = possible_connection.as_mut() {
    match connection.run_pending_migrations(MIGRATIONS) {
      Ok(_) => (),
      Err(e) => {
        eprintln!("Failed to run migrations: {}", e);
        return None;
      }
    }
  }
  possible_connection
}
```

Edit build.rs to add:

```rust
   println!("cargo:rerun-if-changed=path/to/your/migration/dir/relative/to/your/Cargo.toml");
```

We'll create a sample database and then run Diesel on it just to get it to autogenerate the schema file:

```bash
$ diesel migration run --database-url /root/.local/share/zamm/zamm.sqlite3
```

Now `src/schema.rs` should look like this:

```rust
// @generated automatically by Diesel CLI.

diesel::table! {
    executions (id) {
        id -> Text,
        raw_io -> Text,
        command -> Text,
        output -> Text,
    }
}

```

If you want to change something about your `up.sql`, do this:

```bash
$ diesel migration redo --database-url /root/.local/share/zamm/zamm.sqlite3
```

To just undo the last migration, do:

```bash
$ diesel migration revert -n 1 --database-url /root/.local/share/zamm/zamm.sqlite3
```

Add the `uuid` package to enable using UUIDs for IDs:

```bash
$ cargo add uuid --features v4,fast-rng,macro-diagnostics
```

and add the feature to `diesel_migrations` in `Cargo.toml`:

```toml
diesel = { version = "2.1.0", features = ["sqlite", "uuid"] }
```

We can start off by trying

```rust
use crate::schema;
use diesel::prelude::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::executions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Execution {
    pub id: Uuid,
    pub raw_io: String,
    pub command: String,
    pub output: String,
}
```

Because we get this error:

```
 --> src/models.rs:9:13
  |
9 |     pub id: Uuid,
  |             ^^^^ the trait `Queryable<diesel::sql_types::Text, Sqlite>` is not implemented for `Uuid`
  |
  = help: the following other types implement trait `Queryable<ST, DB>`:
            <(T0, T1) as Queryable<(ST0, ST1), __DB>>
            <(T0, T1, T2) as Queryable<(ST0, ST1, ST2), __DB>>
            <(T0, T1, T2, T3) as Queryable<(ST0, ST1, ST2, ST3), __DB>>
            <(T0, T1, T2, T3, T4) as Queryable<(ST0, ST1, ST2, ST3, ST4), __DB>>
            <(T0, T1, T2, T3, T4, T5) as Queryable<(ST0, ST1, ST2, ST3, ST4, ST5), __DB>>
            <(T0, T1, T2, T3, T4, T5, T6) as Queryable<(ST0, ST1, ST2, ST3, ST4, ST5, ST6), __DB>>
            <(T0, T1, T2, T3, T4, T5, T6, T7) as Queryable<(ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7), __DB>>
            <(T0, T1, T2, T3, T4, T5, T6, T7, T8) as Queryable<(ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7, ST8), __DB>>
          and 76 others
  = note: required for `Uuid` to implement `FromSqlRow<diesel::sql_types::Text, Sqlite>`
  = help: see issue #48214
```

we can edit it based on [this answer](https://stackoverflow.com/a/62756507), and the updated documentation for [ToSql](https://docs.diesel.rs/2.1.x/diesel/serialize/trait.ToSql.html) and [FromSql](https://docs.diesel.rs/2.1.x/diesel/deserialize/trait.FromSql.html), and the documentation for [Uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#formatting) itself:

```rust
use crate::schema::executions;
use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use uuid::Uuid;

#[derive(AsExpression, FromSqlRow, Debug, Clone)]
#[diesel(sql_type = Text)]
pub struct EntityId {
    pub uuid: Uuid,
}

#[derive(Queryable, Selectable, Debug)]
pub struct Execution {
    pub id: EntityId,
    pub raw_io: String,
    pub command: String,
    pub output: String,
}

#[derive(Insertable)]
#[diesel(table_name = executions)]
pub struct NewExecution<'a> {
    pub id: EntityId,
    pub raw_io: &'a str,
    pub command: &'a str,
    pub output: &'a str,
}

impl ToSql<Text, Sqlite> for EntityId
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let uuid_str = self.uuid.to_string();
        out.set_value(uuid_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for EntityId
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let uuid_str = String::from_sql(bytes)?;
        let parsed_uuid = Uuid::parse_str(&uuid_str)?;
        Ok(EntityId { uuid: parsed_uuid })
    }
}
```

Note that we were forced to declare a wrapper type `EntityId`, because otherwise we would be unable to implement the traits `ToSql` and `FromSql` for `Uuid` because we don't own any of those types, and in Rust you can only implement a trait for a type if you own either the trait or the type.

Declare these modules in `main.rs`:

```rust
mod models;
mod schema;
```

If you are building a library, declare them instead in a `lib.rs`.

### Inserting and querying data

To test it out, we can set up an in-memory database with all migrations applied:

```rust
    use super::*;
    use crate::setup::MIGRATIONS;

    use diesel_migrations::MigrationHarness;

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }
```

and then

```rust
    #[test]
    fn test_uuid_serialization_and_deserialization() {
        let mut conn = setup_database();

        let new_execution = NewExecution {
            id: EntityId {
                uuid: Uuid::new_v4(),
            },
            raw_io: "Test IO",
            command: "Test Command",
            output: "Test Output",
        };

        // Insert
        diesel::insert_into(executions::table)
            .values(&new_execution)
            .execute(&mut conn)
            .unwrap();

        // Query
        let results: Vec<Execution> = executions::table.load(&mut conn).unwrap();
        assert_eq!(results.len(), 1);

        let retrieved_execution = &results[0];
        assert_eq!(retrieved_execution.id.uuid, new_execution.id.uuid);
        assert_eq!(retrieved_execution.raw_io, new_execution.raw_io);
        assert_eq!(retrieved_execution.command, new_execution.command);
        assert_eq!(retrieved_execution.output, new_execution.output);
    }
```

Note that you shouldn't follow the example in the README for SQLite because the Diesel bindings for SQLite expect [`execute`](https://stackoverflow.com/a/70843733) rather than `get_results`.

## Warnings for clippy pre-commit hook

Diesel warnings don't trigger a non-zero exit code from clippy:

```
    Checking zamm v0.0.0 (/root/zamm/src-tauri)
warning: #[sql_type] attribute form is deprecated
  = help: use `#[diesel(sql_type = Text)]` instead

warning: #[sql_type] attribute form is deprecated
  = help: use `#[diesel(sql_type = Text)]` instead

warning: #[table_name] attribute form is deprecated
  = help: use `#[diesel(table_name = executions)]` instead

warning: #[sql_type] attribute form is deprecated
  = help: use `#[diesel(sql_type = Text)]` instead

warning: #[sql_type] attribute form is deprecated
  = help: use `#[diesel(sql_type = Text)]` instead

warning: #[table_name] attribute form is deprecated
  = help: use `#[diesel(table_name = executions)]` instead

    Finished dev [unoptimized + debuginfo] target(s) in 1.74s
```

Therefore, you should follow the instructions at [`cargo.md`](/zamm/resources/tutorials/setup/repo/pre-commit/cargo.md) to manually error out when "warning" is written to stdout.

To trigger the above warning for a test, do

```rust
#[derive(AsExpression, FromSqlRow, Debug, Clone)]
#[sql_type = "Text"]
pub struct EntityId {
    pub uuid: Uuid,
}
```

The fix is

```rust
#[derive(AsExpression, FromSqlRow, Debug, Clone)]
#[diesel(sql_type = Text)]
pub struct EntityId {
    pub uuid: Uuid,
}
```
