# Saving and retrieving references

We first create a `src-tauri/src/commands/preferences.rs` and define the data structures, as usual:

```rs
use serde::{Deserialize, Serialize};
use specta::{specta, Type};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    unceasing_animations: bool,
    sound_on: bool,
}

```

Then we import this from `src-tauri/src/commands/mod.rs`:

```rust
...
mod preferences;

...
```

This gets the file noticed for compilation purposes, so that we can start iterating on it. Now we fill in the rest of it:

```rust
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use specta::{specta, Type};
use serde_yaml;
use std::fs;

use std::path::PathBuf;

use crate::commands::errors::ZammResult;

static PREFERENCES_FILENAME: &str = "preferences.yaml";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    #[serde(default)]
    unceasing_animations: bool,
    #[serde(default)]
    sound_on: bool,
}

impl Default for Preferences {
  fn default() -> Self {
      Preferences {
        unceasing_animations: false,
        sound_on: true,
      }
  }
}

fn get_preferences_happy_path(maybe_preferences_dir: Option<&PathBuf>) -> ZammResult<Preferences> {
  let preferences_dir = maybe_preferences_dir.ok_or(anyhow!("No preferences dir found"))?;
  let relative_preferences_path = preferences_dir.join(PREFERENCES_FILENAME);
  let preferences_path = fs::canonicalize(relative_preferences_path)?;
  let display_filename = preferences_path.display();
  if preferences_path.exists() {
    println!("Reading preferences from {display_filename}");
    let contents = fs::read_to_string(preferences_path)?;
    let preferences: Preferences = serde_yaml::from_str(&contents)?;
    Ok(preferences)
  } else {
    println!("No preferences found at {display_filename}");
    Ok(Preferences::default())
  }
}

fn get_preferences_helper(preferences_path: &Option<PathBuf>) -> Preferences {
  match get_preferences_happy_path(preferences_path.as_ref()) {
    Ok(preferences) => preferences,
    Err(e) => {
      eprintln!("Error getting preferences: {e}");
      Preferences::default()
    }
  }
}

#[tauri::command]
#[specta]
pub fn get_preferences(app_handle: tauri::AppHandle) -> Preferences {
  let app_dir = app_handle.path_resolver().app_config_dir();
  get_preferences_helper(&app_dir)
}
```

Requirements:

- It should be clear to the user whether the file read succeeded or not
- It should be clear to the user the absolute file path that was expected

We add the corresponding errors at `src-tauri/src/commands/errors.rs`:

```rust
...

#[derive(thiserror::Error, Debug)]
pub enum SerdeError {
    #[error(transparent)]
    Json {
        #[from]
        source: serde_json::Error,
    },
    #[error(transparent)]
    Yaml {
        #[from]
        source: serde_yaml::Error,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    ...
    #[error(transparent)]
    Serde {
        #[from]
        source: SerdeError,
    },
    ...
    #[error(transparent)]
    Io {
        #[from]
        source: std::io::Error,
    },
    ...
}

...

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        let serde_err: SerdeError = err.into();
        serde_err.into()
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        let serde_err: SerdeError = err.into();
        serde_err.into()
    }
}

...
```

and modify `src-tauri/Cargo.toml` to move `serde_yaml = "0.9.25"` from being a dev dependency to being a regular dependency. We export the new API call in `src-tauri/src/commands/errors.rs`:

```rust
pub use preferences::get_preferences;
```

and use it in `src-tauri/src/main.rs`:

```rust
...
use commands::{get_api_keys, greet, play_sound, get_preferences};

...

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![greet, get_api_keys, play_sound, get_preferences],
        "../src-svelte/src/lib/bindings.ts",
    )
    .unwrap();

    ...

    tauri::Builder::default()
        ...
        .invoke_handler(tauri::generate_handler![greet, get_api_keys, play_sound, get_preferences])
        ...
}

```

As usual, we now create a sample call API file at `src-tauri/api/sample-calls/get_preferences-no-file.yaml`:

```yaml
request: ["get_preferences"]
response: >
  {
    "unceasing_animations": false,
    "sound_on": true
  }

```

and we read this in a test at `src-tauri/src/commands/preferences.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;

    use std::fs;

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_get_preferences_sample(file_prefix: &str) {
        let sample = read_sample(file_prefix);
        assert_eq!(sample.request, vec!["get_preferences"]);

        let actual_result = get_preferences_helper(&Some("./non-existent/path".into()));
        let actual_json = serde_json::to_string_pretty(&actual_result).unwrap();
        let expected_json = sample.response.trim();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_get_preferences_without_file() {
        check_get_preferences_sample("./api/sample-calls/get_preferences-no-file.yaml");
    }
}
```

Note that we check that the serialized JSON are equivalent instead of checking that the parsed JSON is equivalent, because the serialized JSON is lower-level and parsing the JSON might kick in Serde's default field-setting, which if invoked will give us an inaccurate, different picture of the API call than what the frontend is experiencing.

Now that this passes, we create a file at `src-tauri/api/sample-settings/sound-override/preferences.yaml`:

```yaml
sound_on: false

```

and the corresponding expected output at `src-tauri/api/sample-calls/get_preferences-sound-override.yaml`:

```yaml
request: ["get_preferences"]
response: >
  {
    "unceasing_animations": false,
    "sound_on": false
  }
```

and modify the `check_get_preferences_sample` function to take in the directory as an argument:

```rust
    fn check_get_preferences_sample(file_prefix: &str, preferences_dir: &str) {
        ...

        let actual_result = get_preferences_helper(&Some(preferences_dir.into()));
        ...
    }

    #[test]
    fn test_get_preferences_without_file() {
        check_get_preferences_sample(
          "./non-existent/path",
          "./api/sample-calls/get_preferences-no-file.yaml",
        );
    }

    #[test]
    fn test_get_preferences_without_file() {
        check_get_preferences_sample(
          "./api/sample-settings/sound-override",
          "./api/sample-calls/get_preferences-no-file.yaml",
        );
    }
```

It fails, but this is only because we swapped the arguments and forgot to pass in the file for `sound-override` instead of no file.

We realize that when the first test fails, it fails with the message

```
Error getting preferences: No such file or directory (os error 2)
```

This is decidedly not what we want. We reproduce this in a test:

```rust
    #[test]
    fn test_get_preferences_happy_path_without_file() {
      let non_existent_path = PathBuf::from("./non-existent/path");
      let happy_path_result = get_preferences_happy_path(Some(&non_existent_path));
      assert!(happy_path_result.is_ok());
      assert_eq!(happy_path_result.unwrap(), Preferences::default());
    }
```

It fails as expected:

```
---- commands::preferences::tests::test_get_preferences_happy_path_without_file stdout ----
thread 'commands::preferences::tests::test_get_preferences_happy_path_without_file' panicked at 'assertion failed: happy_path_result.is_ok()', src/commands/preferences.rs:98:7
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

It turns out this is because `fs::canonicalize` returns an error if the file doesn't already exist, because it tries to resolve symlinks. We don't care about symlinks, we just want the user to know exactly what file is not being read. We solve this by using the `path_absolutize` crate:

```rust
use path_absolutize::Absolutize;

  let preferences_path = relative_preferences_path.absolutize()?;
```

Just to be safe, we add a new sample test file `src-tauri/api/sample-settings/extra-settings/preferences.yaml`

```yaml
sound_on: false
unknown_key: 123

```

with the corresponding `src-tauri/api/sample-calls/get_preferences-extra-settings.yaml`:

```yaml
request: ["get_preferences"]
response: >
  {
    "unceasing_animations": false,
    "sound_on": false
  }

```

and a corresponding test in `src-tauri/src/commands/preferences.rs`

```rust
    #[test]
    fn test_get_preferences_with_extra_settings() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-extra-settings.yaml",
            "./api/sample-settings/extra-settings",
        );
    }
```

We now realize with the additional requirements:

- Unknown keys should not be overwritten on settings save
- Keys should not be written to disk unless they are explicitly toggled from the default

it's better to let the frontend handle the defaults, and let the API call simply be a record of what keys are explicitly set or not, as well as a way of documenting the expected preference type. We make a small change, getting rid of the manual implementation of `Default`:

```rust
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    unceasing_animations: Option<bool>,
    sound_on: Option<bool>,
}
```

As it turns out, `serde_json` serializes `None` to `null`, so our `src-tauri/api/sample-calls/get_preferences-extra-settings.yaml` looks like

```yaml
request: ["get_preferences"]
response: >
  {
    "unceasing_animations": null,
    "sound_on": false
  }

```

Our other files look similar. We do another module refactor, with `src-tauri/src/commands/preferences/models.rs` looking like:

```rust
use serde::{Deserialize, Serialize};

use specta::Type;

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    unceasing_animations: Option<bool>,
    sound_on: Option<bool>,
}

```

and `src-tauri/src/commands/preferences/read.rs` looking like:

```rust
use anyhow::anyhow;
use path_absolutize::Absolutize;

use specta::specta;
use std::fs;

use std::path::PathBuf;

use crate::commands::errors::ZammResult;
use crate::commands::preferences::models::Preferences;

static PREFERENCES_FILENAME: &str = "preferences.yaml";

fn get_preferences_happy_path(
    maybe_preferences_dir: Option<&PathBuf>,
) -> ZammResult<Preferences> {
    let preferences_dir =
        maybe_preferences_dir.ok_or(anyhow!("No preferences dir found"))?;
    let relative_preferences_path = preferences_dir.join(PREFERENCES_FILENAME);
    let preferences_path = relative_preferences_path.absolutize()?;
    let display_filename = preferences_path.display();
    if preferences_path.exists() {
        println!("Reading preferences from {display_filename}");
        let contents = fs::read_to_string(preferences_path)?;
        let preferences: Preferences = serde_yaml::from_str(&contents)?;
        Ok(preferences)
    } else {
        println!("No preferences found at {display_filename}");
        Ok(Preferences::default())
    }
}

fn get_preferences_helper(preferences_path: &Option<PathBuf>) -> Preferences {
    match get_preferences_happy_path(preferences_path.as_ref()) {
        Ok(preferences) => preferences,
        Err(e) => {
            eprintln!("Error getting preferences: {e}");
            Preferences::default()
        }
    }
}

#[tauri::command]
#[specta]
pub fn get_preferences(app_handle: tauri::AppHandle) -> Preferences {
    let app_dir = app_handle.path_resolver().app_config_dir();
    get_preferences_helper(&app_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;

    use std::fs;

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_get_preferences_sample(file_prefix: &str, preferences_dir: &str) {
        let sample = read_sample(file_prefix);
        assert_eq!(sample.request, vec!["get_preferences"]);

        let actual_result = get_preferences_helper(&Some(preferences_dir.into()));
        let actual_json = serde_json::to_string_pretty(&actual_result).unwrap();
        let expected_json = sample.response.trim();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_get_preferences_without_file() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-no-file.yaml",
            "./non-existent/path",
        );
    }

    #[test]
    fn test_get_preferences_happy_path_without_file() {
        let non_existent_path = PathBuf::from("./non-existent/path");
        let happy_path_result = get_preferences_happy_path(Some(&non_existent_path));
        assert!(happy_path_result.is_ok());
        assert_eq!(happy_path_result.unwrap(), Preferences::default());
    }

    #[test]
    fn test_get_preferences_with_sound_override() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-sound-override.yaml",
            "./api/sample-settings/sound-override",
        );
    }

    #[test]
    fn test_get_preferences_with_extra_settings() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-extra-settings.yaml",
            "./api/sample-settings/extra-settings",
        );
    }
}

```

with `src-tauri/src/commands/preferences/mod.rs` tying it all together:

```rust
mod models;
mod read;

pub use models::Preferences;
pub use read::get_preferences;

```

We now start creating a write module, and in the process of doing so realize that we need to refactor some more functionality out of `read.rs`. We therefore edit `src-tauri/src/commands/preferences/models.rs`:

```rust
use crate::commands::errors::ZammResult;
use anyhow::anyhow;
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;

pub static PREFERENCES_FILENAME: &str = "preferences.yaml";

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    unceasing_animations: Option<bool>,
    sound_on: Option<bool>,
}

pub fn get_preferences_file(
    maybe_preferences_dir: Option<&PathBuf>,
) -> ZammResult<PathBuf> {
    let preferences_dir =
        maybe_preferences_dir.ok_or(anyhow!("No preferences dir found"))?;
    let relative_preferences_path = preferences_dir.join(PREFERENCES_FILENAME);
    let absolute_preferences_path = relative_preferences_path.absolutize()?;
    Ok(absolute_preferences_path.into_owned())
}

```

and update `src-tauri/src/commands/preferences/read.rs` accordingly:

```rust
use crate::commands::preferences::models::{get_preferences_file, Preferences};

fn get_preferences_happy_path(
    maybe_preferences_dir: &Option<PathBuf>,
) -> ZammResult<Preferences> {
    let preferences_path = get_preferences_file(maybe_preferences_dir.as_ref())?;
    let display_filename = preferences_path.display();
    ...
}

fn get_preferences_helper(preferences_path: &Option<PathBuf>) -> Preferences {
    match get_preferences_happy_path(preferences_path) {
        ...
    }
}

...

#[cfg(test)]
mod tests {
    ...

    #[test]
    fn test_get_preferences_happy_path_without_file() {
        ...
        let happy_path_result = get_preferences_happy_path(&Some(non_existent_path));
        ...
    }

    ...
}
```

Now we implement `src-tauri/src/commands/preferences/write.rs`:

```rust
use anyhow::anyhow;
use serde_yaml::mapping::Entry;
use serde_yaml::Value;
use specta::specta;
use std::fs;
use std::path::PathBuf;

use crate::commands::errors::ZammResult;
use crate::commands::preferences::models::{get_preferences_file, Preferences};

fn deep_merge(base: &mut Value, other: &Value) {
    match (base, other) {
        (&mut Value::Mapping(ref mut base_map), Value::Mapping(other_map)) => {
            for (k, v) in other_map {
                if !v.is_null() {
                    match base_map.entry(k.clone()) {
                        Entry::Vacant(entry) => {
                            entry.insert(v.clone());
                        }
                        Entry::Occupied(mut entry) => {
                            deep_merge(entry.get_mut(), v);
                        }
                    }
                }
            }
        }
        (base, other) => {
            if !other.is_null() {
                *base = other.clone();
            }
        }
    }
}

fn set_preferences_helper(
    maybe_preferences_dir: &Option<PathBuf>,
    preferences: &Preferences,
) -> ZammResult<()> {
    let preferences_dir = maybe_preferences_dir
        .as_ref()
        .ok_or(anyhow!("No preferences dir found"))?;
    let preferences_path = get_preferences_file(Some(preferences_dir))?;
    let mut existing_yaml = if preferences_path.exists() {
        let file_contents = fs::read_to_string(&preferences_path)?;
        serde_yaml::from_str::<Value>(&file_contents)?
    } else {
        serde_yaml::Mapping::new().into()
    };

    let override_yaml = serde_yaml::to_value(preferences)?;
    deep_merge(&mut existing_yaml, &override_yaml);

    let merged_prefs_str = serde_yaml::to_string(&existing_yaml)?;
    fs::create_dir_all(preferences_dir)?;
    fs::write(preferences_path, merged_prefs_str)?;
    Ok(())
}

#[tauri::command]
#[specta]
pub fn set_preferences(
    app_handle: tauri::AppHandle,
    preferences: Preferences,
) -> ZammResult<()> {
    let app_dir = app_handle.path_resolver().app_config_dir();
    match set_preferences_helper(&app_dir, &preferences) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error writing preferences: {e}");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use serde::{Deserialize, Serialize};

    use std::env;
    use std::fs;

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    struct SetPreferencesRequest {
        preferences: Preferences,
    }

    fn parse_request(request_str: &str) -> SetPreferencesRequest {
        serde_json::from_str(request_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_set_preferences_sample(
        file_prefix: &str,
        existing_preferences_file: Option<&str>,
        expected_preferences_file: &str,
    ) {
        let sample = read_sample(file_prefix);
        assert_eq!(sample.request.len(), 2);
        assert_eq!(sample.request[0], "set_preferences");

        let mut test_preferences_dir = env::temp_dir();
        test_preferences_dir.push("zamm/tests");
        test_preferences_dir.push(PathBuf::from(file_prefix).file_stem().unwrap());
        if test_preferences_dir.exists() {
            fs::remove_dir_all(&test_preferences_dir)
                .expect("Can't reset test preferences dir");
        }
        let test_preferences_file: PathBuf =
            get_preferences_file(Some(&test_preferences_dir)).unwrap();
        println!(
            "Test will use preference file at {}",
            test_preferences_file.display()
        );

        if let Some(existing_preferences) = existing_preferences_file {
            fs::create_dir_all(test_preferences_dir.as_path()).unwrap();
            let test_preferences_path = test_preferences_file.as_path();
            fs::copy(existing_preferences, test_preferences_path).unwrap_or_else(|e| {
                panic!(
                    "Can't copy existing preferences file from {} to {}: {}",
                    existing_preferences,
                    test_preferences_path.display(),
                    e
                )
            });
        }

        let actual_request = parse_request(&sample.request[1]);
        let actual_result = set_preferences_helper(
            &Some(test_preferences_dir),
            &actual_request.preferences,
        );
        assert!(actual_result.is_ok());
        let actual_json =
            serde_json::to_string_pretty(&actual_result.unwrap()).unwrap();
        let expected_json = sample.response.trim();
        assert_eq!(actual_json, expected_json);

        let resulting_contents = fs::read_to_string(test_preferences_file)
            .expect("Test preferences file doesn't exist");
        let expected_contents = fs::read_to_string(expected_preferences_file)
            .unwrap_or_else(|_| {
                panic!("No file found at {}", expected_preferences_file)
            });
        assert_eq!(resulting_contents.trim(), expected_contents.trim());
    }

    #[test]
    fn test_set_preferences_sound_off_without_file() {
        check_set_preferences_sample(
            "./api/sample-calls/set_preferences-sound-off.yaml",
            None,
            "./api/sample-settings/sound-override/preferences.yaml",
        );
    }

    #[test]
    fn test_set_preferences_sound_on_with_extra_settings() {
        check_set_preferences_sample(
            "./api/sample-calls/set_preferences-sound-on.yaml",
            Some("./api/sample-settings/extra-settings/preferences.yaml"),
            "./api/sample-settings/extra-settings/sound-on.yaml",
        );
    }
}

```

with the corresponding sample files at `src-tauri/api/sample-calls/set_preferences-sound-on.yaml`:

```yaml
request:
  - set_preferences
  - >
    {
      "preferences": {
        "sound_on": true
      }
    }
response: "null"

```

and `src-tauri/api/sample-calls/set_preferences-sound-off.yaml`:

```yaml
request:
  - set_preferences
  - >
    {
      "preferences": {
        "sound_on": false
      }
    }
response: "null"

```

which combines with `src-tauri/api/sample-settings/extra-settings/preferences.yaml` to produce `src-tauri/api/sample-settings/extra-settings/sound-on.yaml`:

```yaml
sound_on: true
unknown_key: 123

```

As usual, we add the new command to `src-tauri/src/commands/preferences/mod.rs`:

```rust
mod write;

pub use write::set_preferences;

```

and to `src-tauri/src/commands/mod.rs`:

```rust
pub use preferences::{get_preferences, set_preferences};

```

and to `src-tauri/src/main.rs`:

```rust
use commands::{get_api_keys, get_preferences, greet, play_sound, set_preferences};

...

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            ...
            set_preferences
        ],
        "../src-svelte/src/lib/bindings.ts",
    )
    .unwrap();

    ...

    tauri::Builder::default()
        ...
        .invoke_handler(tauri::generate_handler![
            ...
            set_preferences
        ])
        ...
}
```

We are finally done with the Tauri backend portion of this feature.
