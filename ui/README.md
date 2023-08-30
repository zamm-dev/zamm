# ZAMM

## Dev setup

Follow the instructions in [`tauri.md`](/zamm/resources/tutorials/setup/dev/tauri.md) to set up Tauri.

Then, to avoid the issue mentioned in [`indradb.md`](/zamm/resources/tutorials/libraries/indradb.md), install this:

```bash
$ sudo apt install libclang-dev
```

And to avoid the issue mentioned in [new Tauri project setup](/zamm/resources/tutorials/setup/tauri/new-tauri-project.md), install this:

```bash
$ sudo apt install fuse
```

### For new repos

Follow these instructions to set the project up:

- [Setting up a new Tauri project](/zamm/resources/tutorials/setup/tauri/new-tauri-project.md)
- [Setting up a Python sidecar](/zamm/resources/tutorials/setup/tauri/python-sidecar.md)

### For existing repos

If you already have a version of this project built, then enter into its directories and start building:

```bash
$ cd src-python
$ poetry shell
$ poetry install
$ make sidecar
```

and

```bash
$ pre-commit install
```

## Feature engineering

### Singleton Graph DB

We define it as such

```rust
const DB_NAME: &str = "zamm.sqlite3";

struct ZammDatabase(Mutex<Option<SqliteConnection>>);

...

fn main() {
    let possible_db = get_db();

    tauri::Builder::default()
        .manage(ZammDatabase(Mutex::new(possible_db)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

where `get_db` is defined below.

Check that you have gotten this to compile.

### Saving the DB to the user's data dir

Instead of leaving a mess in the arbitrary current directory where the command is run, or preventing the user from accessing the same database again, use [`directories`](/zamm/resources/tutorials/libraries/directories.md) to pick the user's data folder for app data storage.

Requirements:

- Try to create the database in the user's data directory.
- If that fails for any reason, print out an error message to that effect, and default to creating the database in the current directory instead.
- Make sure to print out the eventual graph database path

Implementation details:

- Make sure to use a constant for the database name.

TODO: make database path a configurable commandline argument instead

This is a sample implementation of these requirements:

```rust
fn connect_to(db_path: PathBuf) -> Option<SqliteConnection> {
    let db_path_str = db_path.to_str().expect("Cannot convert DB path to str");
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
}

/** Try to start SQLite database in user data dir. */
fn get_data_dir_db() -> Option<SqliteConnection> {
    if let Some(user_dirs) = ProjectDirs::from("dev", "zamm", "ZAMM") {
        let data_dir = user_dirs.data_dir();

        if !data_dir.exists() {
            match fs::create_dir_all(&data_dir) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Failed to create data directory: {}", e);
                    return None;
                }
            }
        }

        connect_to(data_dir.join(DB_NAME))
    } else {
        eprintln!("Cannot find user home directory.");
        None
    }
}

fn get_db() -> Option<SqliteConnection> {
    get_data_dir_db().or_else(|| {
        eprintln!("Unable to create DB in user data dir, defaulting to current dir instead.");
        connect_to(env::current_dir().expect("Failed to get current directory").join(DB_NAME))
    })
}
```

### Frontend styling

Install fonts as described [here](/zamm/resources/tutorials/coding/frameworks/sveltekit.md). Then add CSS for the fonts, editing `src-svelte/src/routes/styles.css`:

```css
:root {
  --font-body: Saira, Arial, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  ...
}

...

h1,
h2,
p {
  font-weight: 400;
  font-size: 18px;
}

...
```

"Saira" is the preferred font choice, althoguh "Changa" is a good option too.

If you do this, make sure to edit the font-family for `src-svelte/src/routes/Header.svelte` as well:

```css
  header {
    display: flex;
    justify-content: space-between;
    font-family: Arial, Helvetica, sans-serif;
  }
```


### Exposing API keys to the frontend

First [refactor](/zamm/resources/tutorials/coding/refactoring_rust_module.md), then instructions at [`environment_variables.md`](/zamm/resources/tutorials/systems/environment_variables.md) to read them from the environment.

Then add a command for exposing API keys, as described below. Pipe that data through to the frontend. The keys should be displayed in a table with a single header spanning all columns, named "API Keys". The service name should be displayed on the left column of the table, and the key should be displayed on the right. The key should be marked as "not set" if it's undefined.

Entire page:

```css
<script lang="ts">
  import { getApiKeys } from "$lib/bindings";

  let api_keys = getApiKeys();
</script>

<section>
  <table>
    <tr>
      <th colspan="2">API keys</th>
    </tr>
    <tr>
      <td>OpenAI</td>
      <td class="key">
        {#await api_keys}
          ...loading
        {:then keys}
          {#if keys.openai !== undefined && keys.openai !== null}
            {keys.openai.value}
          {:else}
            <span class="unset">not set</span>
          {/if}
        {:catch error}
          <span style="color: red">{error.message}</span>
        {/await}
      </td>
    </tr>
  </table>
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    flex: 0.6;
  }

  table {
    width:0.1%;
    white-space: nowrap;
  }

  th, td {
    padding: 0 0.5rem;
    text-align: left;
  }

  td {
    color: #000;
  }

  .key {
    font-weight: bold;
    text-transform: lowercase;
  }

  .unset {
    color: #888;
  }
</style>

```

## Actions

### Adding a new command

#### Example: command for exposing API keys

Create `src-tauri/src/commands/keys.rs`:

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

Then edit `src-tauri/src/commands/mod.rs` to include the new command:

```rust
mod api;
mod errors;
mod greet;
mod keys;

pub use greet::greet;
pub use keys::get_api_keys;

```

Now, in `src-tauri/src/main.rs`, check if the name for the new command already exists for a different function or variable. If it does, choose to either:

1. Rename the command to something else
2. Rename the existing function to something else
3. Qualify each use of the existing function or new command with the module name

In this case, `get_api_keys` was already defined for initializing the API keys at app startup. We decide to rename it to `setup_api_keys` instead. Now:

1. Add the new command to Specta types export

```rust
use commands::{get_api_keys, greet};

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![greet, get_api_keys],
        "../src-svelte/src/lib/bindings.ts",
    )
    .unwrap();
```

2. Add the new command to the Tauri invoke handler

```rust
    tauri::Builder::default()
        ...
        .invoke_handler(tauri::generate_handler![greet, get_api_keys])
        ...
```

### Using the new command on the frontend

#### Displaying the API keys

Continuing the example from above, in the Svelte component you want to edit, get the promise:

```ts
  import { getApiKeys } from "$lib/bindings";

  let api_keys = getApiKeys();
```

then edit the HTML for Svelte:

```svelte
<section>
  <p>
    Your OpenAI API key:
    <span class="key">
    {#await api_keys}
      ...loading
    {:then keys}
      {#if keys.openai !== undefined && keys.openai !== null}
        {keys.openai.value}
      {:else}
        <span class="unset">not set</span>
      {/if}
    {:catch error}
      <span style="color: red">{error.message}</span>
    {/await}
    </span>
  </p>
</section>
```

A better way would be to wrap a larger part in "loading...":

```svelte
<section>
  <table>
    <tr>
      <th class="header-text" colspan="2">API Keys</th>
    </tr>
    {#await api_keys}
      <tr><td colspan="2">...loading</td></tr>
    {:then keys}
      <tr>
        <td>OpenAI</td>
        <td class="key">
          {#if keys.openai !== undefined && keys.openai !== null}
            {keys.openai.value}
          {:else}
            <span class="unset">not set</span>
          {/if}
        </td>
      </tr>
    {:catch error}
      <tr><td colspan="2">{error.message}</td></tr>
    {/await}
  </table>
</section>
```

Use [this trick](https://doc.rust-lang.org/std/thread/fn.sleep.html) to make the API call slower so that we can actually see the wait in action:

```rust
use crate::setup::api_keys::ApiKeys;
use crate::ZammApiKeys;
use specta::specta;
use std::clone::Clone;
use std::{thread, time};
use tauri::State;

#[tauri::command]
#[specta]
pub fn get_api_keys(api_keys: State<ZammApiKeys>) -> ApiKeys {
    let ten_seconds = time::Duration::from_secs(10);
    thread::sleep(ten_seconds);
    api_keys.0.lock().unwrap().clone()
}

```

From this we can observe that the screen is not rendering at all until the API call finishes, unlike what SvelteKit should be doing with `await`. At first it is unclear if this is not working as intended due to Tauri or SvelteKit behavior, but with more testing we realize it is definitely on the Tauri side, as not a single way to implement async function on Svelte works.

From further searching, we find [this discussion](https://github.com/tauri-apps/tauri/discussions/4191) where we realize that we need to say `#[tauri::command(async)]`. After this change, the wait works as expected. Make sure to undo the wait before committing.

## Logo

### Creating the SVG

To create the logo SVG, open Gimp up and type in "ZAMM" in 24 point Ropa Sans font. Then, follow [these instructions](https://www.techwalla.com/articles/text-to-path-in-gimp) to convert it to a path. Create a long line that runs parallel to the sloped middle edges of the Z. It may be easier if you press `Edit Mode > Edit (Ctrl)` in the `Paths` palette. Now use that line to extend the top and bottom of the Z to the line, so that the Z becomes a zig-zag. Delete the two intermediate points that are now redundant.

Next, extend the horizontal top and bottom of the Z to cover the whole word. To select multiple points to move at once, select `Edit Mode > Design` and shift-click on each node of the path.

If you see a dotted yellow border, that's the layer boundary. If it doesn't cover the whole word, you can [resize](https://docs.gimp.org/en/gimp-layer-resize.html) the layer as mentioned in the link, or resize it by first doing "Fill Path" in the Paths Tool palette, and then `Layer > Crop to Content` and `Image > Crop to Content` from the menu bar.

You can export the path as an SVG by right-clicking the path on the Paths Tool palette and selecting "Export Path...".

