# Using directories

First add the dependency:

```bash
$ cargo add directories
    Updating crates.io index
      Adding directories v5.0.1 to dependencies.
    Updating crates.io index
```

Then, supposing your org is `zamm.dev` and the app is `ZAMM`, you can use the following code to get the project directory:

```rust
let db_path: PathBuf =
        if let Some(zamm_dirs) = ProjectDirs::from("dev", "zamm", "ZAMM") {
            zamm_dirs.data_dir().join(DB_NAME)
        } else {
            eprintln!("Cannot find user home directory.");
        };
```

This requires

```rust
use directories::ProjectDirs;
use std::path::PathBuf;
```
