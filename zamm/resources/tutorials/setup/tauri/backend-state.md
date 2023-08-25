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

To actually use the state -- for example, if you have

```rust
struct ZammApiKeys(Mutex<ApiKeys>);
```

then do something like

```rust
use crate::setup::api_keys::ApiKeys;
use crate::ZammApiKeys;
use specta::specta;
use std::clone::Clone;
use tauri::State;

#[tauri::command]
#[specta]
pub fn get_api_keys(api_keys: State<ZammApiKeys>) -> ApiKeys {
    api_keys.0.lock().unwrap().clone()
}

```

If you get an error such as

```
error[E0599]: no method named `clone` found for struct `std::sync::MutexGuard<'_, ApiKeys>` in the current scope
 --> src/commands/keys.rs:7:32
  |
7 |     api_keys.0.lock().unwrap().clone()
  |                                ^^^^^ method not found in `MutexGuard<'_, ApiKeys>`
  |
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `clone`, perhaps you need to implement it:
          candidate #1: `Clone`
```

it actually means that `Clone` is not implemented for `ApiKeys`, because otherwise MutexGuard would pass the `.clone()` call down to its inner value. To fix this, add `#[derive(Clone)]` to the struct definition.
