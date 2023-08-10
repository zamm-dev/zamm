# Using Oxigraph

## Installing

Add the dependency using cargo

```toml
$ cargo add oxigraph
```

## Using the library

Use `Store::open` to create a persistent version of the database:

```rust
use oxigraph::store::Store;

    let store = Store::open("example.db")?;
```

After it gets created, you can do various operations on the database (import `use indradb::Error;` and then have main return `Result<(), Error>` with `Ok(())` if you want to do this inside `main`):

```rust
use oxigraph::store::Store;
use std::error::Error;
use oxigraph::model::*;


fn main() -> Result<(), Box<dyn Error>> {
    let store = Store::open("example.db")?;

    // insertion
    let ex = NamedNode::new("http://example.com")?;
    let quad = Quad::new(ex.clone(), ex.clone(), ex.clone(), GraphName::DefaultGraph);
    store.insert(&quad)?;
    store.flush()?;

    return Ok(());
}
```

When you're ready to persist the changes to disk, call `flush`:

```rust
    store.flush();
```

To read it again:

```rust
    let store = Store::open("example.db")?;
```

We can check that the previous edge can be successfully retrieved:

```rust
use oxigraph::sparql::QueryResults;

    if let QueryResults::Solutions(mut solutions) =  store.query("SELECT ?s WHERE { ?s ?p ?o }").unwrap() {
        let solution = solutions.next().unwrap().unwrap();
        let value = solution.get("s").unwrap();
        if let Term::NamedNode(n) = &value {
            assert_eq!(n.as_str(), "http://example.com");
        } else {
            panic!("Unexpected node");
        }
    }
```
