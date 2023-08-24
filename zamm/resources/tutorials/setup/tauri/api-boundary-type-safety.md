# Ensuring type-safety across API boundaries

## TypeScript-Rust

Follow the instructions at [`specta.md`](/zamm/resources/tutorials/libraries/specta.md).

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
