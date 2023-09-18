# Setting up quicktype

First install it:

```bash
$ yarn add -W -D quicktype
```

Then use it:

```bash
$ echo '{"name":"me"}' | yarn quicktype -o greet_args.py
```

Look at `greet_args.py`:

```python
from typing import Any, TypeVar, Type, cast


T = TypeVar("T")


def from_str(x: Any) -> str:
    assert isinstance(x, str)
    return x


def to_class(c: Type[T], x: Any) -> dict:
    assert isinstance(x, c)
    return cast(Any, x).to_dict()


class GreetArgs:
    name: str

    def __init__(self, name: str) -> None:
        self.name = name

    @staticmethod
    def from_dict(obj: Any) -> 'GreetArgs':
        assert isinstance(obj, dict)
        name = from_str(obj.get("name"))
        return GreetArgs(name)

    def to_dict(self) -> dict:
        result: dict = {}
        result["name"] = from_str(self.name)
        return result


def greet_args_from_dict(s: Any) -> GreetArgs:
    return GreetArgs.from_dict(s)


def greet_args_to_dict(x: GreetArgs) -> Any:
    return to_class(GreetArgs, x)

```

Now try Rust:

```bash
$ echo '{"name":"me"}' | yarn quicktype -o greet_args.rs
```

Lok at `greet_args.rs`:

```rust
// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::greet_args;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: greet_args = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GreetArgs {
    name: String,
}

```

It just works! Of course, since the Rust code uses `serde`, make sure to add it to your project if you haven't already:

```bash
$ cargo add serde --features derive
$ cargo add serde_json
```

## Directory-based integration

To do an entire directory, create `src-python/sample-calls` and add `src-python/sample-calls/greet_args.json`:

```json
{
  "name": "Test"
}
```

Do something similar for `src-python/sample-calls/greet_response.json`:

```json
{
  "greeting": "Hi Test! You've been greeted from Python"
}
```

Add this target to the top-level `Makefile`:

```Makefile
quicktype:
	yarn quicktype src-python/sample-calls -o src-python/zamm/api.py
	yarn quicktype src-python/sample-calls -o src-tauri/src/python_api.rs --visibility public --derive-debug --derive-clone --derive-partial-eq
```

Now run

```bash
$ make quicktype
yarn quicktype src-python/sample-calls -o src-python/zamm/api.py
yarn run v1.22.19
$ /root/zamm/node_modules/.bin/quicktype src-python/sample-calls -o src-python/zamm/api.py
Done in 0.87s.
yarn quicktype src-python/sample-calls -o src-tauri/src/python_api.rs
yarn run v1.22.19
$ /root/zamm/node_modules/.bin/quicktype src-python/sample-calls -o src-tauri/src/python_api.rs
Done in 0.89s.
```

Edit `pyproject.toml` to exclude the generated file from `ruff` linting, from:

```toml
[tool.ruff]
select = ["E", "F", "I", "D"]
ignore = ["D203", "D212"]
target-version = "py311"
```

to

```toml
[tool.ruff]
select = ["E", "F", "I", "D"]
ignore = ["D203", "D212"]
exclude = ["src-python/zamm/api.py"]
target-version = "py311"
```

Now you can edit your code to use these new structs!

## Using explicit JSON schemas

`quicktype` can help us generate a JSON schema based on our examples, and then use that schema to generate our code. This is more robust in case `quicktype` cannot correctly infer the types based on the examples:

```bash
$ mkdir -p src-python/api/schemas
$ yarn quicktype src-python/sample-calls/canonical/greet_args.json -o src-python/api/schemas/greet_args.json -l schema
...
$ yarn quicktype src-python/sample-calls/canonical/greet_response.json -o src-python/api/schemas/greet_response.json -l schema
```

## Optional dependency

Because this dependency requires Node v18, you may want to mark this as an optional dependency if you want to build the rest of your project on an older version of Node:

```json
{
  ...
  "optionalDependencies": {
    "quicktype": "^23.0.71"
  }
}
```
