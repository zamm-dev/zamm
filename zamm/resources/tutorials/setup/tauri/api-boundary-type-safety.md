# Ensuring type-safety across API boundaries

## TypeScript-Rust

Follow the instructions at [`specta.md`](/zamm/resources/tutorials/libraries/specta.md).

Note, however, that this does not ensure equivalent, synced API calls as the Rust-Python sample calls do. To implement those sample calls for TypeScript as well, add a TypeScript target for `quicktype` in the `Makefile`:

```Makefile
quicktype:
    ...
	yarn quicktype src-python/api/sample-calls/schema.json -s schema -o src-svelte/src/lib/sample-call.ts
```

Then add the generated file to `src-svelte/.eslintrc.yaml`:

```yaml
...
ignorePatterns:
  ...
  - src/lib/sample-call.ts
```

Now if we try to commit, we get

```
/root/zamm/src-svelte/src/lib/sample-call.ts
  0:0  warning  File ignored because of a matching ignore pattern. Use "--no-ignore" to override

✖ 1 problem (0 errors, 1 warning)

ESLint found too many warnings (maximum: 0).
```

We see that this is because [we're asking it to lint something we've told it to ignore](https://github.com/eslint/eslint/issues/5623#issuecomment-198962552), where "we" in this case is us telling `pre-commit` to tell `eslint` to lint this file. Therefore, we add this same exact ignore to `.pre-commit-config.yaml`:

```yaml
  - repo: https://github.com/pre-commit/mirrors-eslint
    rev: v8.46.0
    hooks:
      - id: eslint
        ...
        exclude: src-svelte/src/lib/sample-call.ts
        ...
```

Looking at the generated file, we see that the `Convert` class takes in strings, so we avoid the YAML trick from before. Instead, we go with JSON and edit `src-tauri/api/sample-calls/get_api_keys-empty.json` to be:

```json
{
  "request": [],
  "response": "{}"
}
```

Now say there's a function you'd like to test, such as:

```rust
#[tauri::command(async)]
#[specta]
pub fn get_api_keys(api_keys: State<ZammApiKeys>) -> ApiKeys {
    api_keys.0.lock().unwrap().clone()
}
```

Refactor it to

```rust
fn get_api_keys_helper(zamm_api_keys: &ZammApiKeys) -> ApiKeys {
    zamm_api_keys.0.lock().unwrap().clone()
}

#[tauri::command(async)]
#[specta]
pub fn get_api_keys(api_keys: State<ZammApiKeys>) -> ApiKeys {
    get_api_keys_helper(&*api_keys)
}
```

so that we actually have a function we can test, because there appears to be no way to easily instantiate a `tauri::State` or `tauri::StateManager` for testing, at least not outside of the `tauri` crate.

Then modify the test from before to now read in the new JSON file and assert that we're returning the same thing as expected:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use std::sync::Mutex;

    use std::fs;

    fn parse_api_keys(response_str: &str) -> ApiKeys {
        serde_json::from_str(response_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_json::from_str(&sample_str).unwrap()
    }

    fn check_get_api_keys_sample(file_prefix: &str, rust_input: &ZammApiKeys) {
        let greet_sample = read_sample(file_prefix);
        // zero frontend arguments to get_api_keys
        assert_eq!(greet_sample.request.len(), 0);

        let actual_keys = get_api_keys_helper(rust_input);
        let expected_keys = parse_api_keys(&greet_sample.response);
        assert_eq!(actual_keys, expected_keys);
    }

    #[test]
    fn test_greet_name() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));

        check_get_api_keys_sample(
            "./api/sample-calls/get_api_keys-empty.json",
            &api_keys,
        );
    }
}
```

Meanwhile, over in Svelte, edit `src-svelte/src/routes/homepage.test.ts` to change

```ts
test("no API key set", async () => {
  const spy = vi.spyOn(window, "__TAURI_INVOKE__");
  expect(spy).not.toHaveBeenCalled();
  const mockApiKeys: ApiKeys = {
    openai: null,
  };
  tauriInvokeMock.mockResolvedValueOnce(mockApiKeys);

  render(ApiKeysDisplay, {});
  expect(spy).toHaveBeenLastCalledWith("get_api_keys");

  const openAiRow = screen.getByRole("row", { name: /OpenAI/ });
  const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
  await waitFor(() => expect(openAiKeyCell).toHaveTextContent(/^not set$/));
});
```

to

```ts
import fs from "fs";
import { Convert } from "$lib/sample-call";

test("no API key set", async () => {
  const spy = vi.spyOn(window, "__TAURI_INVOKE__");
  expect(spy).not.toHaveBeenCalled();
  const sample_call_json = fs.readFileSync(
    "../src-tauri/api/sample-calls/get_api_keys-empty.json",
    "utf-8",
  );
  const sampleCall = Convert.toSampleCall(sample_call_json);
  const apiKeys: ApiKeys = JSON.parse(sampleCall.response)
  tauriInvokeMock.mockResolvedValueOnce(apiKeys);

  render(ApiKeysDisplay, {});
  expect(sampleCall.request.length).toEqual(0);
  expect(spy).toHaveBeenLastCalledWith("get_api_keys");

  const openAiRow = screen.getByRole("row", { name: /OpenAI/ });
  const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
  await waitFor(() => expect(openAiKeyCell).toHaveTextContent(/^not set$/));
});
```

You can refactor most of that into a separate function:

```ts
async function checkSampleCall(filename: string, expected_display: RegExp) {
  const spy = vi.spyOn(window, "__TAURI_INVOKE__");
  expect(spy).not.toHaveBeenCalled();
  const sample_call_json = fs.readFileSync(filename, "utf-8");
  const sampleCall = Convert.toSampleCall(sample_call_json);
  const apiKeys: ApiKeys = JSON.parse(sampleCall.response);
  tauriInvokeMock.mockResolvedValueOnce(apiKeys);

  render(ApiKeysDisplay, {});
  expect(sampleCall.request.length).toEqual(0);
  expect(spy).toHaveBeenLastCalledWith("get_api_keys");

  const openAiRow = screen.getByRole("row", { name: /OpenAI/ });
  const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
  await waitFor(() =>
    expect(openAiKeyCell).toHaveTextContent(expected_display),
  );
}
```

Now the individual tests can get shorter:

```ts
test("no API key set", async () => {
  await checkSampleCall(
    "../src-tauri/api/sample-calls/get_api_keys-empty.json",
    /^not set$/,
  );
});

test("some API key set", async () => {
  await checkSampleCall(
    "../src-tauri/api/sample-calls/get_api_keys-openai.json",
    /^0p3n41-4p1-k3y$/,
  );
});
```

Note that this second test requries the ugly file `src-tauri/api/sample-calls/get_api_keys-openai.json` to look like this:

```json
{
  "request": [],
  "response": "{ \"openai\": { \"value\": \"0p3n41-4p1-k3y\", \"source\": \"Environment\" } }"
}

```

If you want to use YAML here to avoid all the escapes:

```bash
$ yarn add -D js-yaml @types/js-yaml
```

then

```ts
import yaml from 'js-yaml';

async function checkSampleCall(filename: string, expected_display: RegExp) {
  const spy = vi.spyOn(window, "__TAURI_INVOKE__");
  expect(spy).not.toHaveBeenCalled();
  const sample_call_yaml = fs.readFileSync(filename, "utf-8");
  const sample_call_json = JSON.stringify(yaml.load(sample_call_yaml));
  const sampleCall = Convert.toSampleCall(sample_call_json);
  ...
}
```

We could also define a new `Convert` class that takes in a YAML string instead, and get rid of the need to go from YAML string -> dict -> JSON string -> dict -> desired data structure, but this involves slightly more manual work every time a new function is renamed, and slight inefficiencies during development is acceptable.

Now we rename `src-tauri/api/sample-calls/get_api_keys-openai.json` to `src-tauri/api/sample-calls/get_api_keys-openai.yaml` and get pretty JSON once again:

```yaml
request: []
response: >
  {
    "openai": {
      "value": "0p3n41-4p1-k3y",
      "source": "Environment"
    }
  }

```

## Rust-Python

First follow the instructions at [`quicktype.md`](/zamm/resources/tutorials/setup/dev/quicktype.md) to set up `quicktype`. Now refactor `src-python/zamm/main.py`:

```python
"""Entry-point for ZAMM Python functionality."""

import sys


def greet(name: str) -> None:
    """Say hello-world."""
    print(f"Hello, {name}! You have been greeted from Python")


greet(sys.argv[1] if len(sys.argv) > 1 else "World")

```

to use the new definitions:

```python
"""Entry-point for ZAMM Python functionality."""

import sys
from zamm.api import GreetArgs, GreetResponse
import json


def greet(args: GreetArgs) -> GreetResponse:
    """Say hello-world."""
    return GreetResponse(
        greeting=f"Hello, {args.name}! You have been greeted from Python"
    )


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python -m zamm.main <json-args>")
        sys.exit(1)
    args_dict = json.loads(sys.argv[1])
    args = GreetArgs.from_dict(args_dict)
    response = greet(args)
    print(json.dumps(response.to_dict()))
```

Add tests for it:

```python
"""Test that greetings work."""

from zamm.main import greet
from zamm.api import GreetArgs


def test_regular_greet() -> None:
    """Make sure a regular greeting works."""
    args = GreetArgs(name="World")
    response = greet(args)
    assert response.greeting == "Hello, World! You have been greeted from Python"


def test_empty_greet() -> None:
    """Make sure an empty greeting works."""
    args = GreetArgs(name="")
    response = greet(args)
    assert response.greeting == "Hello, ! You have been greeted from Python"

```

Check that this side of the API boundary works:

```bash
$ make test
poetry run pytest -v
============================ test session starts ============================
platform linux -- Python 3.11.4, pytest-7.4.0, pluggy-1.2.0 -- /root/.cache/pypoetry/virtualenvs/zamm-seoTTefp-py3.11/bin/python
cachedir: .pytest_cache
rootdir: /root/zamm/src-python
collected 2 items                                                           

tests/test_greet.py::test_regular_greet PASSED                        [ 50%]
tests/test_greet.py::test_empty_greet PASSED                          [100%]

============================= 2 passed in 0.01s =============================
```

Now do the same for Rust. First add `mod python_api;` to `main.rs`. Then edit `src-tauri/src/commands.rs` from

```rust
...

fn greet_helper<T: SidecarExecutor>(t: &T, name: &str) -> String {
    let result = t.execute("zamm-python", &[name]).unwrap();
    format!("{result} via Rust")
}

#[tauri::command]
#[specta]
pub fn greet(name: &str) -> String {
    greet_helper(&SidecarExecutorImpl {}, name)
}

#[cfg(test)]
mod tests {
    use super::{greet_helper, MockSidecarExecutor};

    #[test]
    fn test_greet_name() {
        let mut mock = MockSidecarExecutor::new();
        mock.expect_execute()
            .withf(|cmd, args| {
                assert_eq!(cmd, "zamm-python");
                assert_eq!(args, &vec!["Test"]);
                true
            })
            .returning(|_, _| {
                Ok("Hello, Test! You have been greeted from Python".to_string())
            });

        let result = greet_helper(&mock, "Test");
        assert_eq!(
            result,
            "Hello, Test! You have been greeted from Python via Rust"
        );
    }
}
```

to

```rust
use crate::python_api::{GreetArgs, GreetResponse};
use serde::{Serialize, Deserialize};

...

fn process<T: Serialize, U: for<'de> Deserialize<'de>>(
    s: &impl SidecarExecutor,
    command: &str,
    input: &T,
) -> Result<U> {
    let input_json = serde_json::to_string(input)?;
    let result_json = s.execute(command, &[&input_json])?;
    let response: U = serde_json::from_str(&result_json)?;

    Ok(response)
}

fn greet_helper<T: SidecarExecutor>(t: &T, name: &str) -> String {
    let result = process::<GreetArgs, GreetResponse>(
        t,
        "zamm-python",
        &GreetArgs { name: name.into() },
    )
    .unwrap();
    let greeting = result.greeting;
    format!("{greeting} via Rust")
}

#[tauri::command]
#[specta]
pub fn greet(name: &str) -> String {
    greet_helper(&SidecarExecutorImpl {}, name)
}

#[cfg(test)]
mod tests {
    use super::{greet_helper, MockSidecarExecutor};

    #[test]
    fn test_greet_name() {
        let mut mock = MockSidecarExecutor::new();
        mock.expect_execute()
            .withf(|cmd, args| {
                assert_eq!(cmd, "zamm-python");
                assert_eq!(args, &vec!["{\"name\":\"Test\"}"]);
                true
            })
            .returning(|_, _| {
                Ok(
                    "{\"greeting\":\"Hello, Test! You have been greeted from Python\"}"
                        .to_string(),
                )
            });

        let result = greet_helper(&mock, "Test");
        assert_eq!(
            result,
            "Hello, Test! You have been greeted from Python via Rust"
        );
    }
}
```

We are not mocking the `process` response because it involves both generic types and generic lifetimes, which is not supported by `mockall`. This means, however, that we are forced to pass in JSON strings as input and output in the tests. This does provide for stronger guarantees, as it also makes sure that there are no serialization or deserialization errors lurking on this side of the API boundary, but it also means that the tests are clunky to write, and are also inconsistent with how we're testing the Python side of the API at a higher level.

As such, we will solve both problems by making sure that both sides of the API boundary are expecting and producing exactly the same strings:

### Testing exact string responses

#### Python

Let's move both `greet_args.json` and `greet_response.json` into a subdirectory, `sample-calls/canonical`. This will allow us to keep passing in one entire directory to `quicktype` to produce definitions for. Make sure to update our `Makefile` as well, from

```Makefile
quicktype:
	yarn quicktype src-python/sample-calls -o src-python/zamm/api.py
	yarn quicktype src-python/sample-calls -o src-tauri/src/python_api.rs --visibility public --derive-debug --derive-clone --derive-partial-eq
```

to

```Makefile
quicktype:
	yarn quicktype src-python/sample-calls/canonical -o src-python/zamm/api.py
	yarn quicktype src-python/sample-calls/canonical -o src-tauri/src/python_api.rs --visibility public --derive-debug --derive-clone --derive-partial-eq
```

Now create the folder `sample-calls/alt` and put in alternative tests there. For `src-python/sample-calls/alt/greet_empty_args.json`:

```json
{
  "name": ""
}
```

For `src-python/sample-calls/alt/greet_empty_response.json`:

```json
{
  "greeting": "Hi Test! You've been greeted from Python"
}
```

Now edit `src-python/tests/test_greet.py` from:

```python
"""Test that greetings work."""

from zamm.api import GreetArgs
from zamm.main import greet


def test_regular_greet() -> None:
    """Make sure a regular greeting works."""
    args = GreetArgs(name="World")
    response = greet(args)
    assert response.greeting == "Hello, World! You have been greeted from Python"


def test_empty_greet() -> None:
    """Make sure an empty greeting works."""
    args = GreetArgs(name="")
    response = greet(args)
    assert response.greeting == "Hello, ! You have been greeted from Python"

```

to

```python
"""Test that greetings work."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Callable, Protocol, Type, TypeVar

from zamm.api import GreetArgs
from zamm.main import greet

T = TypeVar("T", bound="ApiArgs", covariant=True)
U = TypeVar("U", bound="ApiResponse", covariant=True)


class ApiArgs(Protocol[T]):
    """Stub for API arguments generated by quicktype."""

    @staticmethod
    def from_dict(obj: Any) -> T:
        """Create an instance of this class from a dict."""
        ...

    def to_dict(self) -> dict:
        """Return a JSON-serializable dict representation of this class."""
        ...


class ApiResponse(Protocol[U]):
    """Stub for API responses generated by quicktype."""

    @staticmethod
    def from_dict(obj: Any) -> U:
        """Create an instance of this class from a dict."""
        ...

    def to_dict(self) -> dict:
        """Return a JSON-serializable dict representation of this class."""
        ...


def read_json(filename: str) -> dict:
    """Read JSON dict from a file."""
    return json.loads(Path(filename).read_text())


def compare_io(
    file_prefix: str, args_type: Type[T], get_response: Callable[[T], U]
) -> None:
    """Compare input and output for generic commands."""
    args_file = f"{file_prefix}_args.json"
    response_file = f"{file_prefix}_response.json"
    args: T = args_type.from_dict(read_json(args_file))  # type: ignore
    response = get_response(args)
    assert response.to_dict() == read_json(response_file)


def compare_greet_io(file_prefix: str) -> None:
    """Compare input and output for the greet command."""
    compare_io(file_prefix, GreetArgs, greet)


def test_regular_greet() -> None:
    """Make sure a regular greeting works."""
    compare_greet_io("sample-calls/canonical/greet")


def test_empty_greet() -> None:
    """Make sure an empty greeting works."""
    compare_greet_io("sample-calls/alt/greet_empty")

```

Note that:

- The duplication between `ApiArgs` and `ApiResponse` is because it is currently unknown how to define the `Callable[[T], U]` type in `compare_io` as two separate classes that both adhere to the same protocol.
- The `# type: ignore` comment is because `mypy` otherwise gives the error

> src-python/tests/test_greet.py:53: error: Incompatible types in assignment (expression has type "T", variable has type "T")  [assignment]

To check that our type-checking works so that we're not accidentally passing in the wrong function for the wrong arguments, change `compare_greet_io` to take in a `GreetResponse` instead:

```python
def compare_greet_io(file_prefix: str) -> None:
    """Compare input and output for the greet command."""
    compare_io(file_prefix, GreetResponse, greet)
```

Now `mypy` correctly complains:

```
src-python/tests/test_greet.py:60: error: Argument 3 to "compare_io" has incompatible type "Callable[[GreetArgs], GreetResponse]"; expected "Callable[[GreetResponse], GreetResponse]"  [arg-type]
```

#### Rust

We start off with implementing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn parse_greet(args_str: &str) -> GreetArgs {
        serde_json::from_str(args_str).unwrap()
    }

    fn read_file(file_prefix: &str, suffix: &str) -> String {
        let expected_path = format!("{file_prefix}{suffix}");
        fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("No file found at {expected_path}"))
    }

    fn check_greet_sample(file_prefix: &str, rust_result: &str) {
        let greet_args_str = read_file(file_prefix, "_args.json");
        let greet_response_str = read_file(file_prefix, "_response.json");

        let mut mock = MockSidecarExecutor::new();
        mock.expect_execute()
            .withf(move |cmd, args| {
                assert_eq!(cmd, "zamm-python");

                let actual_args = parse_greet(args[0]);
                let expected_args = parse_greet(&greet_args_str);
                assert_eq!(actual_args, expected_args);

                true
            })
            .return_once(move |_, _| Ok(greet_response_str));

        let result = greet_helper(&mock, "Test");
        assert_eq!(result, rust_result);
    }

    #[test]
    fn test_greet_name() {
        check_greet_sample(
            "../src-python/sample-calls/canonical/greet",
            "Hello, Test! You have been greeted from Python via Rust",
        );
    }

    #[test]
    fn test_greet_empty_name() {
        check_greet_sample(
            "../src-python/sample-calls/alt/greet_empty",
            "Hello, ! You have been greeted from Python via Rust",
        );
    }
}

```

The additional `test_greet_empty_name` is good for catching errors related to reading from the wrong set of test files.

It fails:

```
---- commands::tests::test_greet_empty_name stdout ----
thread 'commands::tests::test_greet_empty_name' panicked at 'assertion failed: `(left == right)`
  left: `GreetArgs { name: "Test" }`,
 right: `GreetArgs { name: "" }`', src/commands.rs:131:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    commands::tests::test_greet_empty_name
```

It turns out we forgot to parameterize the input:

```rust
    fn check_greet_sample(file_prefix: &str, rust_input: &str, rust_result: &str) {
        let greet_args_str = read_file(file_prefix, "_args.json");
        let greet_response_str = read_file(file_prefix, "_response.json");

        let mut mock = MockSidecarExecutor::new();
        mock.expect_execute()
            .withf(move |cmd, args| {
                assert_eq!(cmd, "zamm-python");

                let actual_args = parse_greet(args[0]);
                let expected_args = parse_greet(&greet_args_str);
                assert_eq!(actual_args, expected_args);

                true
            })
            .return_once(move |_, _| Ok(greet_response_str));

        let result = greet_helper(&mock, rust_input);
        assert_eq!(result, rust_result);
    }

    #[test]
    fn test_greet_name() {
        check_greet_sample(
            "../src-python/sample-calls/canonical/greet",
            "Test",
            "Hello, Test! You have been greeted from Python via Rust",
        );
    }

    #[test]
    fn test_greet_empty_name() {
        check_greet_sample(
            "../src-python/sample-calls/alt/greet_empty",
            "",
            "Hello, ! You have been greeted from Python via Rust",
        );
    }
```

Now it all works:

```
running 3 tests
test commands::tests::test_greet_empty_name ... ok
test commands::tests::test_greet_name ... ok
test models::tests::test_uuid_serialization_and_deserialization ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

and we are confident that the test both fails and succeeds when it is meant to.

Now we have tested both the Python and Rust sides of the API boundary, all the way up to the exact JSON strings being passed back and forth. Both sides are reading from the same set of files, so there is no need to worry about the sample tests going out of sync.

#### Refactoring the API into one command per file

We will want to support multiple commands in the future. So, let's start off by creating a new file `src-python/zamm/api/__init__.py`:

```python
"""API methods for the Python CLI."""
```

and renaming `src-python/zamm/api.py` to `src-python/zamm/api/models.py`. Make sure to once again edit `Makefile`:

```Makefile
quicktype:
	yarn quicktype src-python/sample-calls/canonical -o src-python/zamm/api/models.py
```

and `src-python/pyproject.toml`:

```toml
[tool.ruff]
...
exclude = ["src-python/zamm/api/models.py"]
```

and our imports in `src-python/zamm/main.py`:

```python
from zamm.api.models import GreetArgs, GreetResponse
```

Let's test to make sure we haven't forgotten anything:

```python
$ make test
poetry run pytest -v
================================ test session starts ================================
platform linux -- Python 3.11.4, pytest-7.4.0, pluggy-1.2.0 -- /root/.cache/pypoetry/virtualenvs/zamm-seoTTefp-py3.11/bin/python
cachedir: .pytest_cache
rootdir: /root/zamm/src-python
collected 0 items / 1 error                                                         

====================================== ERRORS =======================================
_______________________ ERROR collecting tests/test_greet.py ________________________
ImportError while importing test module '/root/zamm/src-python/tests/test_greet.py'.
Hint: make sure your test modules/packages have valid Python names.
Traceback:
/root/.asdf/installs/python/3.11.4/lib/python3.11/importlib/__init__.py:126: in import_module
    return _bootstrap._gcd_import(name[level:], package, level)
tests/test_greet.py:9: in <module>
    from zamm.api import GreetArgs
E   ImportError: cannot import name 'GreetArgs' from 'zamm.api' (/root/zamm/src-python/zamm/api/__init__.py)
============================== short test summary info ==============================
ERROR tests/test_greet.py
!!!!!!!!!!!!!!!!!!!!!! Interrupted: 1 error during collection !!!!!!!!!!!!!!!!!!!!!!!
================================= 1 error in 0.08s ==================================
make: *** [Makefile:24: tests] Error 2
```

Right, let's edit `src-python/tests/test_greet.py` as well:

```python
from zamm.api.models import GreetArgs
```

Now it passes:

```
make test
poetry run pytest -v
================================ test session starts ================================
platform linux -- Python 3.11.4, pytest-7.4.0, pluggy-1.2.0 -- /root/.cache/pypoetry/virtualenvs/zamm-seoTTefp-py3.11/bin/python
cachedir: .pytest_cache
rootdir: /root/zamm/src-python
collected 2 items                                                                   

tests/test_greet.py::test_regular_greet PASSED                                [ 50%]
tests/test_greet.py::test_empty_greet PASSED                                  [100%]

================================= 2 passed in 0.02s =================================
```

We now split off `src-python/zamm/main.py`, from:

```python
"""Entry-point for ZAMM Python functionality."""

import json
import sys

from zamm.api.models import GreetArgs, GreetResponse


def greet(args: GreetArgs) -> GreetResponse:
    """Say hello-world."""
    return GreetResponse(
        greeting=f"Hello, {args.name}! You have been greeted from Python"
    )


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python -m zamm.main <json-args>")
        sys.exit(1)
    args_dict = json.loads(sys.argv[1])
    args = GreetArgs.from_dict(args_dict)
    response = greet(args)
    print(json.dumps(response.to_dict()))

```

into

```python
"""Entry-point for ZAMM Python functionality."""

import json
import sys

from zamm.api.greet import greet, greet_method

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python -m zamm.main <json-args>")
        sys.exit(1)
    args_dict = json.loads(sys.argv[1])
    args = greet_method.args_type.from_dict(args_dict)
    response = greet(args)
    print(json.dumps(response.to_dict()))

```

and a `src-python/zamm/api/greet.py` file that looks like

```python
"""API for greeting users."""

from zamm.api.methods import ApiMethod
from zamm.api.models import GreetArgs, GreetResponse


def greet(args: GreetArgs) -> GreetResponse:
    """Say hello-world."""
    return GreetResponse(
        greeting=f"Hello, {args.name}! You have been greeted from Python"
    )


greet_method = ApiMethod(GreetArgs, GreetResponse, greet)

```

and a `src-python/zamm/api/methods.py` file that looks like

```python
"""Enable programmatic access to API metadata."""

from __future__ import annotations

from typing import Any, Callable, Generic, Protocol, TypeVar

T = TypeVar("T", bound="ApiArgs", covariant=True)
U = TypeVar("U")


class ApiArgs(Protocol[T]):
    """Stub for API arguments generated by quicktype."""

    @staticmethod
    def from_dict(obj: Any) -> T:
        """Create an instance of this class from a dict."""
        ...

    def to_dict(self) -> dict:
        """Return a JSON-serializable dict representation of this class."""
        ...


class ApiMethod(Generic[T, U]):
    """Models a single API method."""

    args_type: type[T]
    response_type: type[U]
    invoke: Callable[[T], U]

    def __init__(
        self, args_type: type[T], response_type: type[U], invocation: Callable[[T], U]
    ) -> None:
        """Define a new API method and its expected types."""
        self.args_type = args_type
        self.response_type = response_type
        self.invoke = invocation
```

We also edit `src-python/tests/test_greet.py` to use the new internal API:

```python
"""Test that greetings work."""

from __future__ import annotations

import json
from pathlib import Path

from zamm.api.methods import ApiMethod
from zamm.main import greet_method


def read_json(filename: str) -> dict:
    """Read JSON dict from a file."""
    return json.loads(Path(filename).read_text())


def compare_io(file_prefix: str, api_method: ApiMethod) -> None:
    """Compare input and output for generic commands."""
    args_file = f"{file_prefix}_args.json"
    response_file = f"{file_prefix}_response.json"
    args = api_method.args_type.from_dict(read_json(args_file))
    response = api_method.invoke(args)
    assert response.to_dict() == read_json(response_file)


def compare_greet_io(file_prefix: str) -> None:
    """Compare input and output for the greet command."""
    compare_io(file_prefix, greet_method)


def test_regular_greet() -> None:
    """Make sure a regular greeting works."""
    compare_greet_io("sample-calls/canonical/greet")


def test_empty_greet() -> None:
    """Make sure an empty greeting works."""
    compare_greet_io("sample-calls/alt/greet_empty")

```

Make sure all tests still pass:

```bash
$ make test 
poetry run pytest -v
============================== test session starts ==============================
platform linux -- Python 3.11.4, pytest-7.4.0, pluggy-1.2.0 -- /root/.cache/pypoetry/virtualenvs/zamm-seoTTefp-py3.11/bin/python
cachedir: .pytest_cache
rootdir: /root/zamm/src-python
collected 2 items                                                               

tests/test_greet.py::test_regular_greet PASSED                            [ 50%]
tests/test_greet.py::test_empty_greet PASSED                              [100%]

=============================== 2 passed in 0.02s ===============================
```

Let's refactor the tests as well to make use of the new layout. Create a `src-python/tests/api/__init__.py`:

```python
"""API tests."""

```

and then move `test_greet.py` inside the new directory. Split it off into:

```python
"""Test that greetings work."""

from zamm.main import greet_method

from tests.api.test_helpers import compare_io


def compare_greet_io(file_prefix: str) -> None:
    """Compare input and output for the greet command."""
    compare_io(file_prefix, greet_method)


def test_regular_greet() -> None:
    """Make sure a regular greeting works."""
    compare_greet_io("sample-calls/canonical/greet")


def test_empty_greet() -> None:
    """Make sure an empty greeting works."""
    compare_greet_io("sample-calls/alt/greet_empty")

```

and a `src-python/tests/api/test_helpers.py` that looks like this:

```python
"""Helpers for generic API tests."""

import json
from pathlib import Path

from zamm.api.methods import ApiMethod


def read_json(filename: str) -> dict:
    """Read JSON dict from a file."""
    return json.loads(Path(filename).read_text())


def compare_io(file_prefix: str, api_method: ApiMethod) -> None:
    """Compare input and output for generic commands."""
    args_file = f"{file_prefix}_args.json"
    response_file = f"{file_prefix}_response.json"
    args = api_method.args_type.from_dict(read_json(args_file))
    response = api_method.invoke(args)
    assert response.to_dict() == read_json(response_file)

```

Once again, make sure the tests still work:

```bash
$ make test
poetry run pytest -v
============================== test session starts ==============================
platform linux -- Python 3.11.4, pytest-7.4.0, pluggy-1.2.0 -- /root/.cache/pypoetry/virtualenvs/zamm-seoTTefp-py3.11/bin/python
cachedir: .pytest_cache
rootdir: /root/zamm/src-python
collected 2 items                                                               

tests/api/test_greet.py::test_regular_greet PASSED                        [ 50%]
tests/api/test_greet.py::test_empty_greet PASSED                          [100%]

=============================== 2 passed in 0.02s ===============================
```

Commit as well to make sure that all the other pre-commit checks work as well.

Now we want to make it possible for the Python API to support more than one call. We are currently invoking the Python sidecar like this:

```bash
$ dist/main '{"name": "Test"}'
{"greeting": "Hello, Test! You have been greeted from Python"}
```

One obvious way to support multiple arguments would be to pass in the name of the method as the first argument, and then the arguments as the second argument:

```bash
$ dist/main greet '{"name": "Test"}'
{"greeting": "Hello, Test! You have been greeted from Python"}
```

However, doing this unilaterally on the Python side would break the application *without* the Rust-specific unit tests failing. We want to avoid relying on slow and heavy end-to-end tests to catch such mistakes, so we'll have to store the entire commandline invocation in the sample JSON files. We could store that as a simple array:

```json
["greet", {"name": "Test"}]
```

However, that confuses `quicktype`, which assumes it is a list where every element is a union of a string or an object. As such, we'll have to explicitly tell quicktype about our expected types as JSON schema instead of letting it infer it on its own based on example inputs. Once again, see [`quicktype.md`](/zamm/resources/tutorials/setup/dev/quicktype.md) for instructions on how to set this up.

We move everything from `src-python/sample-calls/alt` and `src-python/sample-calls/canonical` into `src-python/api/sample-calls`:

```bash
$ mkdir -p src-python/api/sample-calls
$ mv src-python/sample-calls/**/*.json src-python/api/sample-calls
$ rm -rf src-python/sample-calls
```

Once again, we update the `Makefile`:

```Makefile
quicktype:
	yarn quicktype src-python/api/schemas/* -s schema -o src-python/zamm/api/models.py
	yarn quicktype src-python/api/schemas/* -s schema -o src-tauri/src/python_api.rs --visibility public --derive-debug --derive-clone --derive-partial-eq
```

Note in particular the `-s schema` argument, to tell it to read the JSON file as a schema file, and the `/*` glob, because it appears that the `-s schema` argument gets ignored otherwise. You want to make sure that your generated class looks like this:

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
...
```

and not this if you lose the `-s schema` argument:

```python
from typing import Any, List, TypeVar, Type, cast, Callable


T = TypeVar("T")


def from_str(x: Any) -> str:
    assert isinstance(x, str)
    return x


def to_class(c: Type[T], x: Any) -> dict:
    assert isinstance(x, c)
    return cast(Any, x).to_dict()


def from_bool(x: Any) -> bool:
    assert isinstance(x, bool)
    return x


def from_list(f: Callable[[Any], T], x: Any) -> List[T]:
    assert isinstance(x, list)
    return [f(y) for y in x]


class Name:
    type: str

    def __init__(self, type: str) -> None:
        self.type = type

    @staticmethod
    def from_dict(obj: Any) -> 'Name':
        assert isinstance(obj, dict)
        type = from_str(obj.get("type"))
        return Name(type)

    def to_dict(self) -> dict:
        result: dict = {}
        result["type"] = from_str(self.type)
        return result


class GreetArgsProperties:
    name: Name

    def __init__(self, name: Name) -> None:
        self.name = name

    @staticmethod
    def from_dict(obj: Any) -> 'GreetArgsProperties':
        assert isinstance(obj, dict)
        name = Name.from_dict(obj.get("name"))
        return GreetArgsProperties(name)

    def to_dict(self) -> dict:
        result: dict = {}
        result["name"] = to_class(Name, self.name)
        return result


class GreetArgsClass:
    type: str
    additional_properties: bool
    properties: GreetArgsProperties
    required: List[str]
    title: str

    def __init__(self, type: str, additional_properties: bool, properties: GreetArgsProperties, required: List[str], title: str) -> None:
        self.type = type
        self.additional_properties = additional_properties
        self.properties = properties
        self.required = required
        self.title = title

    @staticmethod
    def from_dict(obj: Any) -> 'GreetArgsClass':
        assert isinstance(obj, dict)
        type = from_str(obj.get("type"))
        additional_properties = from_bool(obj.get("additionalProperties"))
        properties = GreetArgsProperties.from_dict(obj.get("properties"))
        required = from_list(from_str, obj.get("required"))
        title = from_str(obj.get("title"))
        return GreetArgsClass(type, additional_properties, properties, required, title)

    def to_dict(self) -> dict:
        result: dict = {}
        result["type"] = from_str(self.type)
        result["additionalProperties"] = from_bool(self.additional_properties)
        result["properties"] = to_class(GreetArgsProperties, self.properties)
        result["required"] = from_list(from_str, self.required)
        result["title"] = from_str(self.title)
        return result
...
```

which is what happens when it tries to read the JSON file as a sample file instead of a schema file.

We update the Python and Rust tests as well to point to the new path, and make sure all the tests still pass.

Now that it's time for us to update the sample JSON files, we have the choice to use a JSON array to represent the arguments, as mentioned before:

```json
["greet", {"name": "Test"}]
```

 This means, however, that the second argument will already be parsed as a dict, instead of the string which it will be passed in as when the command is actually invoked. This should be fine, because there is a relatively low chance of a mistake happening during the JSON parsing of commandline arguments. Ideally, though, we get to simulate passing in the actual strings during command invocation on the Python side, and simulate outputting the actual strings during command invocation on the Rust side. We could make the second argument a string containing JSON:

 ```json
 ["greet", "{\"name\": \"Test\"}"]
 ```
 
Unfortunately, this makes it hard to edit the test file as a human. JSON does not have great native support for newlines within strings either. Therefore, we will instead use YAML to store the second JSON string. For example, `src-python/api/sample-calls/greet_args.json` gets renamed to `src-python/api/sample-calls/greet_args.yaml` and looks like this:

```yaml
- greet
- >
  {
    "name": "Test"
  }

```

This is the new `src-python/api/sample-calls/greet_response.yaml`:

```yaml
- greet
- >
  {
    "greeting": "Hello, Test! You have been greeted from Python"
  }

```

In fact, we might as well combine the two into one at `src-python/api/sample-calls/greet_empty_args.yaml`:

```yaml
request:
- greet
- >
  {
    "name": "Test"
  }
response: >
  {
    "greeting": "Hello, Test! You have been greeted from Python"
  }

```

Refactor `src-python/zamm/main.py` yet again into:

```python
"""Entry-point for ZAMM Python functionality."""

import sys

from zamm.execution import handle_commandline_args

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python -m zamm.main <command> <json-args>")
        sys.exit(1)
    print(handle_commandline_args(*sys.argv[1:]))

```

and then create `src-python/zamm/execution.py` to look like:

```python
"""Execution logic for commandline arguments."""

import json

from zamm.api.greet import greet_method

METHODS = {
    "greet": greet_method,
}


def handle_commandline_args(method_name: str, args_dict_str: str) -> str:
    """Handle commandline arguments."""
    method = METHODS[method_name]
    args_dict = json.loads(args_dict_str)
    args = method.args_type.from_dict(args_dict)
    response = method.invoke(args)
    return json.dumps(response.to_dict())

```

Update `src-python/tests/api/test_helpers.py`:

```python
"""Helpers for generic API tests."""

import json
from pathlib import Path

import yaml
from zamm.execution import handle_commandline_args


def compare_io(filename: str) -> None:
    """Compare input and output for generic commands."""
    sample_call = yaml.safe_load(Path(filename).read_text())
    response = handle_commandline_args(*sample_call["request"])
    assert json.loads(response) == json.loads(sample_call["response"])

```

and `src-python/tests/api/test_greet.py`

```python
"""Test that greetings work."""

from tests.api.test_helpers import compare_io


def test_regular_greet() -> None:
    """Make sure a regular greeting works."""
    compare_io("api/sample-calls/greet.yaml")


def test_empty_greet() -> None:
    """Make sure an empty greeting works."""
    compare_io("api/sample-calls/greet_empty.yaml")

```

Now add typing information for YAML for type checking:

```bash
$ poetry add --group dev types-PyYAML
```

and add it to `.pre-commit-config.yaml` too:

```yaml
  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.4.1
    hooks:
      - id: mypy
        args: [--disallow-untyped-defs, --ignore-missing-imports]
        additional_dependencies:
          - types-PyYAML
```

Now all tests and pre-commit checks should work.

Of course, now that we have a new format for the tests that we want to use between both Python and Rust, we can define yet another JSON schema for it at `src-python/api/sample-calls/schema.json`:

```json
{
  "$schema": "http://json-schema.org/draft-06/schema#",
  "$ref": "#/definitions/SampleCall",
  "definitions": {
    "SampleCall": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "request": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "response": {
          "type": "string"
        }
      },
      "required": ["request", "response"],
      "title": "SampleCall"
    }
  }
}

```

and then update the Makefile:

```Makefile
quicktype:
	yarn quicktype src-python/api/schemas/* -s schema -o src-python/zamm/api/models.py
	yarn quicktype src-python/api/schemas/* -s schema -o src-tauri/src/python_api.rs --visibility public --derive-debug --derive-clone --derive-partial-eq
	yarn quicktype src-python/api/sample-calls/schema.json -s schema -o src-python/tests/api/sample_call.py
	yarn quicktype src-python/api/sample-calls/schema.json -s schema -o src-tauri/src/sample_call.rs --visibility public --derive-debug
```

and update the ruff ignore in `src-python/pyproject.toml`:

```toml
[tool.ruff]
select = ["E", "F", "I", "D"]
ignore = ["D203", "D212"]
exclude = [
  "src-python/zamm/api/models.py",
  "src-python/tests/api/sample_call.py"
]
target-version = "py311"
```

and update `src-python/tests/api/test_helpers.py`:

```python
"""Helpers for generic API tests."""

import json
from pathlib import Path

import yaml
from zamm.execution import handle_commandline_args

from tests.api.sample_call import SampleCall


def compare_io(filename: str) -> None:
    """Compare input and output for generic commands."""
    sample_call = SampleCall.from_dict(yaml.safe_load(Path(filename).read_text()))
    response = handle_commandline_args(*sample_call.request)
    assert json.loads(response) == json.loads(sample_call.response)

```

Now add YAML support to the Rust side as well:

```bash
$ cargo add --dev serde_yaml
```

And tell `main.rs` about the newly generated file:

```rust
#[cfg(test)]
mod sample_call;
```

Update the Rust-side consumption of the JSON output at `src-tauri/src/commands.rs`:

```rust
fn process<T: Serialize, U: for<'de> Deserialize<'de>>(
    s: &impl SidecarExecutor,
    binary: &str,
    command: &str,
    input: &T,
) -> Result<U> {
    let input_json = serde_json::to_string(input)?;
    let result_json = s.execute(binary, &[command, &input_json])?;
    let response: U = serde_json::from_str(&result_json)?;

    Ok(response)
}

fn greet_helper<T: SidecarExecutor>(t: &T, name: &str) -> String {
    let result = process::<GreetArgs, GreetResponse>(
        t,
        "zamm-python",
        "greet",
        &GreetArgs { name: name.into() },
    )
    .unwrap();
    let greeting = result.greeting;
    format!("{greeting} via Rust")
}
```

and the corresponding tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    
    use std::fs;

    fn parse_greet(args_str: &str) -> GreetArgs {
        serde_json::from_str(args_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_greet_sample(file_prefix: &str, rust_input: &str, rust_result: &str) {
        let greet_sample = read_sample(file_prefix);

        let mut mock = MockSidecarExecutor::new();
        mock.expect_execute()
            .withf(move |cmd, actual_cmd_args| {
                assert_eq!(cmd, "zamm-python");

                let expected_cmd_args = &greet_sample.request;
                assert_eq!(actual_cmd_args.len(), expected_cmd_args.len());
                assert_eq!(actual_cmd_args[0], expected_cmd_args[0]);

                let actual_greet_args = parse_greet(actual_cmd_args[1]);
                let expected_greet_args = parse_greet(&expected_cmd_args[1]);
                assert_eq!(actual_greet_args, expected_greet_args);

                true
            })
            .return_once(move |_, _| Ok(greet_sample.response));

        let result = greet_helper(&mock, rust_input);
        assert_eq!(result, rust_result);
    }

    #[test]
    fn test_greet_name() {
        check_greet_sample(
            "../src-python/api/sample-calls/greet.yaml",
            "Test",
            "Hello, Test! You have been greeted from Python via Rust",
        );
    }

    #[test]
    fn test_greet_empty_name() {
        check_greet_sample(
            "../src-python/api/sample-calls/greet_empty.yaml",
            "",
            "Hello, ! You have been greeted from Python via Rust",
        );
    }
}
```

Note that this only tests JSON-equivalence in the command strings. If we wanted to go further, we could also test that the strings are exactly the same by capturing the output from the Python and Rust executables, or by getting those languages to serialize the JSON in exactly the manner provided by the coder. For now, without the help of further automation, we'll leave it as-is.

#### Using natural arguments

We can change the API definition to something more Python, from

```python
def greet(args: GreetArgs) -> GreetResponse:
    """Say hello-world."""
    return GreetResponse(
        greeting=f"Hello, {args.name}! You have been greeted from Python"
    )
```

to

```python
def greet(name: str) -> GreetResponse:
    """Say hello-world."""
    return GreetResponse(greeting=f"Hello, {name}! You have been greeted from Python")
```

We'll just have to change `ApiMethod` as well:

```python
class ApiMethod(Generic[T, U]):
    """Models a single API method."""

    args_type: type[T]
    response_type: type[U]
    method: Callable[..., U]

    def __init__(
        self, args_type: type[T], response_type: type[U], method: Callable[..., U]
    ) -> None:
        """Define a new API method and its expected types."""
        self.args_type = args_type
        self.response_type = response_type
        self.method = method

    def invoke(self, args: T) -> U:  # type: ignore
        """Invoke the API method."""
        return self.method(**args.to_dict())
```

However, this means that we'll no longer be able to check that the arguments are correct without running the unit tests. For example, if we had a typo in the argument name:

```python
def greet(namef: str) -> GreetResponse:
    """Say hello-world."""
    return GreetResponse(greeting=f"Hello, {namef}! You have been greeted from Python")

```

mypy will not complain, versus if we did this:

```python
def greet(args: GreetArgs) -> GreetResponse:
    """Say hello-world."""
    return GreetResponse(
        greeting=f"Hello, {args.namef}! You have been greeted from Python"
    )
```

then mypy says:

```
mypy .
zamm/api/greet.py:10: error: "GreetArgs" has no attribute "namef"; maybe "name"?  [attr-defined]
```

There is a tradeoff to be made here between how much assurance we want that the code works, and how readable the code is. For the ZAMM project, we'll choose greater assurance in order to allow LLMs to better be able to self-correct their code.

## Allowing failed responses

While developing across the Tauri-Svelte API boundary, we find that we want these API calls to be able to fail, so that we can test how these failures are handled on the frontend. As such, we edit `src-python/api/sample-calls/schema.json` to turn the response into an object instead of a simple string:

```json
{
  ...
  "definitions": {
    "SampleCall": {
      ...
      "properties": {
        "response": {
          "type": "object",
          "additionalProperties": false,
          "properties": {
            "success": {
              "type": "boolean"
            },
            "message": {
              "type": "string"
            }
          },
          "required": ["message"]
        }
      },
      ...
    }
  }
}
```

We now use the following command defined in our main `Makefile` to update SampleCall definitions across the project:

```bash
$ make quicktype
```

`src-python/tests/api/sample_call.py` got updated automatically as a result of that command. We now edit the Python API sample call at `src-python/api/sample-calls/greet_empty.yaml` to adhere to the new format:

```yaml
request:
  ...
response:
  message: >
    {
      "greeting": "Hello, ! You have been greeted from Python"
    }

```

We do the same for `src-python/api/sample-calls/greet.yaml`, and update `src-python/tests/api/test_helpers.py` to take in the new format:

```py
def compare_io(filename: str) -> None:
    ...
    assert json.loads(response) == json.loads(sample_call.response.message)

```

We confirm that all the Python tests pass.

Next, we look at the Rust side. `src-tauri/src/sample_call.rs` has been automatically updated as well. We update each of the files in `src-tauri/api/sample-calls` in a similar manner, and use the new failure marker in `src-tauri/api/sample-calls/set_api_key-invalid-filename.yaml`:

```yaml
request:
  ...
response:
  success: false
  message: >
    "Is a directory (os error 21)"

```

We start editing the Rust sample call test parsers, such as `src-tauri/src/commands/system.rs`:

```rs
    fn check_get_system_info_sample(...) {
        ...

        let expected_info = parse_system_info(&system_info_sample.response.message);
        ...
    }
```

We apply similar edits to all other tests, replacing `sample.response` with `sample.response.message`. Afterwards, we still have to change the test file that prompted this refactor in the first place, by editing `src-tauri/src/commands/keys/set.rs` to remove the `should_fail` argument, and removing the `check_set_api_key_sample_unit_fails` function because there is no longer anything to differentiate it from the `check_set_api_key_sample_unit` function:

```rs
    pub fn check_set_api_key_sample(
        sample_file: &str,
        existing_zamm_api_keys: &ZammApiKeys,
        test_dir_name: &str,
    ) {
        ...
        if sample.response.success == Some(false) {
            assert!(actual_result.is_err(), "API call should have thrown error");
        } else {
            ...
        }
        ...
        let expected_json = sample.response.message.trim();
        ...
    }

    fn check_set_api_key_sample_unit(
        ...
    ) {
        check_set_api_key_sample(sample_file, existing_zamm_api_keys, "set_api_key");
    }

    ...

    #[test]
    fn test_invalid_filename() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            "api/sample-calls/set_api_key-invalid-filename.yaml",
            &api_keys,
        );
    }
```

We edit the other call to the function in `src-tauri/src/commands/keys/mod.rs`, to use the new signature:

```rs
    #[test]
    fn test_get_after_set() {
        ...

        check_set_api_key_sample(
            "api/sample-calls/set_api_key-existing-no-newline.yaml",
            &api_keys,
            "api_keys_integration_tests",
        );

        ...
    }
```

After checking that all Rust tests pass, we turn our attention to Svelte. Here, we see that `src-svelte/src/lib/sample-call.ts` has been automatically edited. We follow up with changes to `src-svelte/src/lib/sample-call-testing.ts`:

```ts
...

export interface ParsedCall {
  ...
  succeeded: boolean;
}

...

export function parseSampleCall(sampleFile: string): ParsedCall {
  ...
  const parsedSample: ParsedCall = {
    ...,
    response: JSON.parse(rawSample.response.message),
    // if response.success does not exist, then it defaults to true
    succeeded: rawSample.response.success !== false,
  };
  return parsedSample;
}

export class TauriInvokePlayback {
  ...

  mockCall(
    ...
  ): Promise<Record<string, string>> {
    ...
    const matchingCall = this.unmatchedCalls[matchingCallIndex];
    this.unmatchedCalls.splice(matchingCallIndex, 1);

    if (matchingCall.succeeded) {
      return Promise.resolve(matchingCall.response);
    } else {
      return Promise.reject(matchingCall.response);
    }
  }
}
```

There are only a couple of tests that have not yet been refactored to use this new playback scheme, so we update them accordingly, starting with `src-svelte/src/lib/Switch.test.ts`:

```ts
import { ..., type Mock } from "vitest";
...
import { TauriInvokePlayback } from "$lib/sample-call-testing";

...

describe("Switch", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;
  ...

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );
    playback.addSamples(
      "../src-tauri/api/sample-calls/play_sound-switch.yaml",
    );

    ...
  });

  ...

  test("plays clicking sound during toggle", async () => {
    ...
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    ...
    expect(tauriInvokeMock).toBeCalledTimes(1);
  });

  test("does not play clicking sound when sound off", async () => {
    ...
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    ...
    expect(tauriInvokeMock).not.toHaveBeenCalled();
  });

  ...
});
```

We do the same for `src-svelte/src/routes/SidebarUI.test.ts`:

```ts
import { ..., type Mock } from "vitest";
...
import { TauriInvokePlayback } from "$lib/sample-call-testing";
...

describe("Sidebar", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;
  ...

  beforeAll(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );
    playback.addSamples(
      "../src-tauri/api/sample-calls/play_sound-whoosh.yaml",
    );
  });

  beforeEach(() => {
    ...
    expect(tauriInvokeMock).not.toHaveBeenCalled();
  });

  ...

  test("plays whoosh sound with right speed and volume", async () => {
    ...
    expect(tauriInvokeMock).toBeCalledTimes(1);
  });

  test("does not play whoosh sound when sound off", async () => {
    ...
    expect(tauriInvokeMock).not.toHaveBeenCalled();
  });

  test("does not play whoosh sound when path unchanged", async () => {
    ...
    expect(tauriInvokeMock).not.toHaveBeenCalled();
  });
});
```

After checking that all the Svelte tests pass as well, we find that the only thing preventing us from committing is mypy complaining about the generated file:

```
src-python/tests/api/sample_call.py:22: error: Function is missing a type annotation  [no-untyped-def]
Found 1 error in 1 file (checked 2 source files)
```

We see that the function in question is:

```py
def from_union(fs, x):
    for f in fs:
        try:
            return f(x)
        except:
            pass
    assert False
```

We change the function signature to:

```py
def from_union(fs: List[Callable], x: Any) -> Any:
  ...
```
