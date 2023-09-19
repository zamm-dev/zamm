# Using IndraDB

## Installing

Add this line to your `Cargo.toml` file:

```toml
...

[dependencies]
...
indradb-lib = { version = "4.0.0", features = ["rocksdb-datastore"] }
```

The `rocksdb-datastore` feature is only necessary if you're planning to use RocksDB to store this data.

Try to now build the project. If it fails with

```bash
$ cargo build
...
   Compiling tauri-macros v1.4.0
   Compiling rmp-serde v1.1.2
   Compiling bincode v1.3.3
error: failed to run custom build command for `librocksdb-sys v0.8.3+7.4.4`

Caused by:
  process didn't exit successfully: `/home/amos/Documents/ui/zamm/src-tauri/target/debug/build/librocksdb-sys-cbb702e1334be35f/build-script-build` (exit status: 101)
  --- stderr
  thread 'main' panicked at 'Unable to find libclang: "couldn't find any valid shared libraries matching: ['libclang.so', 'libclang-*.so', 'libclang.so.*', 'libclang-*.so.*'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"', /home/amos/.asdf/installs/rust/1.71.1/registry/src/index.crates.io-6f17d22bba15001f/bindgen-0.64.0/./lib.rs:2393:31
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
```

then install clang libraries first:

```bash
$ sudo apt install libclang-dev
```

## Using the library

Use `MemoryDatastore::create_msgpack_db` to create a persistent version of the database:

```rust
    let db = MemoryDatastore::create_msgpack_db("test_msgpack_db");
```

After it gets created, you can do various operations on the database (import `use indradb::Error;` and then have main return `Result<(), Error>` with `Ok(())` if you want to do this inside `main`):

```rust
    let out_v = indradb::Vertex::new(indradb::Identifier::new("person")?);
    let in_v = indradb::Vertex::new(indradb::Identifier::new("movie")?);
    db.create_vertex(&out_v)?;
    db.create_vertex(&in_v)?;

    // Add an edge between the vertices
    let edge = indradb::Edge::new(out_v.id, indradb::Identifier::new("likes")?, in_v.id);
    db.create_edge(&edge)?;
```

When you're ready to persist the changes to disk, call `sync`:

```rust
    db.sync()?;
```

To read it again:

```rust
    let db = MemoryDatastore::read_msgpack_db("test_msgpack_db").expect("Could not read db");
```

We can check that the previous edge can be successfully retrieved:

```rust
    let output: Vec<indradb::QueryOutputValue> = db.get(indradb::AllEdgeQuery {})?;
    // Convenience function to extract out the edges from the query results
    let e = indradb::util::extract_edges(output).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].t, indradb::Identifier::new("likes")?);
```

Adding IndraDB to a Tauri app results in an increase of about 2 MB.
