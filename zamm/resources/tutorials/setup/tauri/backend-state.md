# Tauri backend state management

To store state or resources in the Tauri backend, we first set up a singleton graph database using Tauri's state management system. Import Mutex:

```rust
use std::sync::Mutex;
use oxigraph::store::StorageError;
```

Then define a GraphDB struct to store the new store in. Connect to "zamm.db" by default, but put it in a constant.

```rust
const DB_NAME: &str = "zamm.db";

struct GraphDB(Mutex<Store>);
```

Now add this new store to the state, from:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
```

to

```rust
fn main() -> Result<(), StorageError> {
    let store = Store::open(DB_NAME)?;

    tauri::Builder::default()
        .manage(GraphDB(Mutex::new(store)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
```