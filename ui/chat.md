# Chat feature

## Initial implementation

### Python

First we set up our dependencies by editing `src-python/pyproject.toml`:

```toml
...

[tool.poetry.dependencies]
...
openai = "^1.6.1"
langchain = "^0.1.0"
langchain-openai = "^0.0.2"

[tool.poetry.group.dev.dependencies]
...
vcr-langchain = "^0.0.31"

...
```

Then we create a sample call file at `src-python/api/sample-calls/chat.yaml` to work off of:

```bash
$ request:
  - chat
  - >
    {
      "provider": "OpenAI",
      "api_key": "dummy-key",
      "llm": "gpt-3.5-turbo-1106",
      "prompt": [
        {
          "role": "System",
          "message": "You are the brains of ZAMM, a chat program."
        },
        {
          "role": "Human",
          "message": "Hello, does this work?"
        }
      ]
    }
response:
  message: >
    {
      "response": "LLM says it does work."
    }

```

We don't actually know how the response will look yet, but at least we'll start off with enough information to send the request. We then extract the request JSON part into a `chat.json` and create a schema using:

```bash
$ yarn quicktype src-python/api/sample-calls/chat.json -o src-python/api/schemas/chat_args.json -l schema
```

We review the generated `src-python/api/schemas/chat_args.json` to check that it looks reasonable:

```json
{
    "$schema": "http://json-schema.org/draft-06/schema#",
    "$ref": "#/definitions/ChatArgs",
    "definitions": {
        "ChatArgs": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "provider": {
                    "type": "string"
                },
                "api_key": {
                    "type": "string"
                },
                "llm": {
                    "type": "string"
                },
                "prompt": {
                    "type": "array",
                    "items": {
                        "$ref": "#/definitions/Prompt"
                    }
                }
            },
            "required": [
                "api_key",
                "llm",
                "prompt",
                "provider"
            ],
            "title": "ChatArgs"
        },
        "Prompt": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "role": {
                    "type": "string"
                },
                "message": {
                    "type": "string"
                }
            },
            "required": [
                "message",
                "role"
            ],
            "title": "Prompt"
        }
    }
}

```

We rename `Prompt` to `ChatMessage` because it's a list of chat messages:

```json
{
    ...
    "definitions": {
        "ChatArgs": {
            ...,
            "properties": {
                ...,
                "prompt": {
                    "type": "array",
                    "items": {
                        "$ref": "#/definitions/ChatMessage"
                    }
                }
            },
            ...
        },
        "ChatMessage": {
            ...
            "title": "ChatMessage"
        }
    }
}
```

Next, we generate the Python classes using:

```bash
$ make quicktype
```

To make it easier to experiment with quicktype generation from these JSON files, we can edit the main `Makefile` to only consider JSON files in that directory, instead of a total `*` glob:

```Makefile
quicktype:
	yarn quicktype src-python/api/schemas/*.json ...
	yarn quicktype src-python/api/schemas/*.json ...
	...
```

With a bit more experimentation, we find that we want our chat response to similarly make use of the `ChatMessage` schema, but quicktype will not realize it's the same object and name it a different class in the generated Python model file. As such, we:

- put both chat arg and response types in the same file
- make one single top-level class called `ChatMethod`, because quicktype ignores any definitions that are not part of the top-level class
- add a `temperature` setting to the arguments
- rename `src-python/api/schemas/chat_args.json` to `src-python/api/schemas/chat_method.json`

With these changes, the file looks like:

```json
"$schema": "http://json-schema.org/draft-06/schema#",
    "type": "object",
    "additionalProperties": false,
    "title": "ChatMethod",
    "properties": {
        "args": {
            "$ref": "#/definitions/ChatArgs"
        },
        "response": {
            "$ref": "#/definitions/ChatResponse"
        }
    },
    "required": [
        "args",
        "response"
    ],
    "definitions": {
        "ChatArgs": {
            ...,
            "properties": {
                ...
                "temperature": {
                    "type": "number"
                },
                ...
            },
            ...
        },
        "ChatResponse": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "response": {
                    "$ref": "#/definitions/ChatMessage"
                },
                "tokens": {
                    "$ref": "#/definitions/TokenMetadata"
                }
            },
            "required": [
                "response",
                "tokens"
            ],
            "title": "ChatResponse"
        },
        ...
        "TokenMetadata": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "total": {
                    "type": "integer"
                },
                "prompt": {
                    "type": "integer"
                },
                "completion": {
                    "type": "integer"
                },
                "cost": {
                    "type": "number"
                }
            },
            "required": [
                "completion",
                "cost",
                "prompt",
                "total"
            ],
            "title": "TokenMetadata"
        }
    }
}

```

The only new model in `src-python/zamm/api/models.py` that we're not going to make use of is `ChatMethod`. As for the rest of the fields, we make use of them in the final `src-python/api/sample-calls/chat.yaml`, specifying a different temperature than the regular langchain default of 0.7 to make sure that it's being passed through correctly:

```yaml
request:
  - chat
  - >
    {
      ...
      "temperature": 0.65,
      ...
    }
response:
  message: >
    {
      "response": {
        "role": "AI",
        "message": "Hello! Yes, it works. How can I assist you today?"
      },
      "tokens": {
        "total": 44,
        "prompt": 30,
        "completion": 14,
        "cost": 0.000058
      }
    }

```

We check out `src-python/tests/api/sample_call.py` in Git after the `make quicktype` command because the definition of sample calls haven't changed, and we need that custom type definition that we manually added in for mypy.

Next, we actually implement the new API method in `src-python/zamm/api/chat.py`:

```py
"""API for sending LLMs a single prompt."""

from langchain_openai import ChatOpenAI
from zamm.api.methods import ApiMethod
from zamm.api.models import ChatArgs, ChatMessage, ChatResponse, TokenMetadata
from langchain_community.callbacks import get_openai_callback
from langchain_core.messages import (
    BaseMessage as LCBaseMessage,
    HumanMessage,
    SystemMessage,
    AIMessage
)
from langchain_core.outputs import ChatGeneration
from langchain_core.prompt_values import ChatPromptValue
from typing import cast
import os


def to_langchain_prompt(messages: ChatMessage) -> ChatPromptValue:
    """Convert a list of our ChatMessage to a Langchain prompt."""
    lc_messages: list[LCBaseMessage] = []
    for message in messages:
        if message.role == "Human":
            lc_messages.append(HumanMessage(content=message.message))
        elif message.role == "AI":
            lc_messages.append(AIMessage(content=message.message))
        elif message.role == "System":
            lc_messages.append(SystemMessage(content=message.message))
        else:
            raise ValueError(f"Unknown role {message.role}")
    return ChatPromptValue(messages=lc_messages)


def chat(args: ChatArgs) -> ChatResponse:
    """Send a chat message to the LLM."""
    if args.provider == "OpenAI":
        if "ZAMM_DUMMY_API_KEYS" in os.environ:
            api_key = os.environ["OPENAI_API_KEY"]
        else:
            api_key = args.api_key
        llm = ChatOpenAI(api_key=api_key, model=args.llm, temperature=args.temperature)
    else:
        raise NotImplementedError(f"Provider {args.provider} not yet supported")
    
    prompt = to_langchain_prompt(args.prompt)
    with get_openai_callback() as cb:
        result = cast(ChatGeneration, llm.generate_prompt([prompt]).generations[0][0])
        return ChatResponse(
            response=ChatMessage(message=result.text, role="AI"),
            tokens=TokenMetadata(
                completion=cb.completion_tokens,
                prompt=cb.prompt_tokens,
                total=cb.total_tokens,
                cost=cb.total_cost,
            )
        )


chat_method = ApiMethod(ChatArgs, str, chat)

```

We edit `src-python/Makefile` next to make sure that `ZAMM_DUMMY_API_KEYS` is being set before the test:

```Makefile
tests: export ZAMM_DUMMY_API_KEYS = true
tests:
	poetry run pytest -v

```

We need to refer to this new method in the execution declarations file, and to make that convenient we re-export it and the greet method in `src-python/zamm/api/__init__.py`:

```py
"""API methods for the Python CLI."""

from zamm.api.greet import greet_method
from zamm.api.chat import chat_method

__all__ = ["greet_method", "chat_method"]

```

Now we can import both in one line in `src-python/zamm/execution.py`:

```py
...

from zamm.api import chat_method, greet_method

METHODS = {
    "chat": chat_method,
    ...
}

...
```

We test this in `src-python/tests/api/test_chat.py`:

```py
"""Test that chat invocations work."""

from tests.api.test_helpers import compare_io
import vcr_langchain as vcr


@vcr.use_cassette()
def test_openai_chat() -> None:
    """Make sure a regular chat message works."""
    compare_io("api/sample-calls/chat.yaml")

```

We run the tests and manually check the newly recorded API call to OpenAI at `src-python/tests/api/test_openai_chat.yaml` to ensure that it's calling with the correct content:

```yaml
interactions:
- request:
    body: '{"messages": [{"role": "system", "content": "You are the brains of ZAMM,
      a chat program."}, {"role": "user", "content": "Hello, does this work?"}], "model":
      "gpt-3.5-turbo-1106", ..., "temperature": 0.65}'
    headers:
      host:
      - api.openai.com
      x-stainless-async:
      - 'false'
    method: POST
    uri: https://api.openai.com/v1/chat/completions
  response:
    content: "{...\"model\": \"gpt-3.5-turbo-1106\",\n
      ... \"message\": {\n        \"role\":
      \"assistant\",\n        \"content\": \"Hello! Yes, it works. How can I assist
      you today?\"\n      },... \"usage\": {\n    \"prompt_tokens\": 30,\n    \"completion_tokens\":
      14,\n    \"total_tokens\": 44\n  },..."
    headers:
      ...
```

We also make sure that the test runs successfully the next time with `ZAMM_DUMMY_API_KEYS` unset, to ensure that it is actually replaying cached API calls to OpenAI.

The tests all pass, but annoying warnings are generated. We edit `src-python/pyproject.toml` again to make warnings a failure, and to ignore these warnings:

```toml
...

[tool.pytest.ini_options]
filterwarnings = [
    "error",
    'ignore::pydantic.warnings.PydanticDeprecatedSince20',
]

...
```

#### Commit errors

We run into a bunch of issues with the quicktype-generated file and mypy. We go through them:

```
src-python/zamm/api/models.py:27: error: Function is missing a type annotation  [no-untyped-def]
```

This refers to

```py
def from_union(fs, x):
    ...
```

We change it again as we did in [`api-boundary-type-safety.md`](/zamm/resources/tutorials/setup/tauri/api-boundary-type-safety.md):

```py
def from_union(fs: List[Callable], x: Any) -> Any:
    ...
```

Next:

```
src-python/zamm/api/chat.py:20: error: "ChatMessage" has no attribute "__iter__" (not iterable)  [attr-defined]
```

It is:

```py
def to_langchain_prompt(messages: ChatMessage) -> ChatPromptValue:
    ...
    for message in messages:
        ...
```

We fix with:

```py
def to_langchain_prompt(messages: list[ChatMessage]) -> ChatPromptValue:
    ...
    for mesage in messages:
        ...
```

This also fixes the later error

```
src-python/zamm/api/chat.py:43: error: Argument 1 to "to_langchain_prompt" has incompatible type "list[ChatMessage]"; expected "ChatMessage"  [arg-type]
```

at

```py
def chat(args: ChatArgs) -> ChatResponse:
    ...
    prompt = to_langchain_prompt(args.prompt)
    ...
```

Next:

```
src-python/zamm/api/chat.py:57: error: Argument 3 to "ApiMethod" has incompatible type "Callable[[ChatArgs], ChatResponse]"; expected "Callable[[ChatArgs], str]"  [arg-type]
```

caused by

```py
chat_method = ApiMethod(ChatArgs, str, chat)
```

We fix with:

```py
chat_method = ApiMethod(ChatArgs, ChatResponse, chat)
```

Next:

```
src-python/zamm/execution.py:17: error: "object" has no attribute "args_type"  [attr-defined]
src-python/zamm/execution.py:18: error: "object" has no attribute "invoke"  [attr-defined]
```

caused by:

```py
...

METHODS = {
    "chat": chat_method,
    "greet": greet_method,
}


def handle_commandline_args(method_name: str, args_dict_str: str) -> str:
    """Handle commandline arguments."""
    method = METHODS[method_name]
    args_dict = json.loads(args_dict_str)
    args = method.args_type.from_dict(args_dict)
    response = method.invoke(args)
    ...

```

We fix by editing `src-python/zamm/api/__init__.py` again to export `ApiMethod`:

```py
...
from zamm.api.methods import ApiMethod

__all__ = [..., "ApiMethod"]

```

and then adding better type hinting to `src-python/zamm/execution.py`:

```py
...

from zamm.api import ApiMethod, ...

METHODS: dict[str, ApiMethod] = {
    ...
}

...
```

#### Saving model name

We realize that OpenAI returns the specific model used in the response, which may be different from the more general model requested. We should save this information as well. We edit `src-python/api/schemas/chat_method.json` in preparation:

```json
{
  "$schema": "http://json-schema.org/draft-06/schema#",
  ...
  "definitions": {
    ...
    "ChatResponse": {
      ...
      "properties": {
        ...
        "llm": {
          "type": "string"
        }
      },
      "required": [..., "llm"],
      ...
    },
    ...
  }
}
```

We edit the existing file `src-python/api/sample-calls/chat.yaml`:

```yaml
request:
  - chat
  - >
    {
      ...,
      "llm": "gpt-3.5-turbo-1106",
      ...
    }
response:
  message: >
    {
      ...,
      "llm": "gpt-3.5-turbo-1106"
    }
```

and create a new one at `src-python/api/sample-calls/chat_reply.yaml` where we demonstrate this in action:

```yaml
request:
  - chat
  - >
    {
      "provider": "OpenAI",
      "api_key": "dummy-key",
      "llm": "gpt-3.5-turbo",
      "temperature": 0.3,
      "prompt": [
        {
          "role": "System",
          "message": "You are the brains of ZAMM, a chat program."
        },
        {
          "role": "Human",
          "message": "Hello, does this work?"
        },
        {
          "role": "AI",
          "message": "Hello! Yes, it works. How can I assist you today?"
        },
        {
          "role": "Human",
          "message": "How do you start a new Tauri project?"
        }
      ]
    }
response:
  message: >
    {
      "response": {
        "role": "AI",
        "message": "TBD"
      },
      ...
    }

```

We edit `src-python/tests/api/test_chat.py` to make this new sample call:

```py
@vcr.use_cassette()
def test_openai_chat_reply() -> None:
    """Make sure a response to an AI message works."""
    compare_io("api/sample-calls/chat_reply.yaml")

```

The file `src-python/tests/api/test_openai_chat_reply.yaml` gets recorded:

```yaml
interactions:
- request:
    body: '{"messages": [{"role": "system", "content": "You are the brains of ZAMM,
      a chat program."}, {"role": "user", "content": "Hello, does this work?"}, {"role":
      "assistant", "content": "Hello! Yes, it works. How can I assist you today?"},
      {"role": "user", "content": "How do you start a new Tauri project?"}], "model":
      "gpt-3.5-turbo", "n": 1, "stream": false, "temperature": 0.3}'
    headers:
      host:
      - api.openai.com
      x-stainless-async:
      - 'false'
    method: POST
    uri: https://api.openai.com/v1/chat/completions
  response:
    content: "{\n  \"id\": \"chatcmpl-8fbMpBNgyxD4RF9e2CcdsPWtgJ3qq\",\n  \"object\":
      \"chat.completion\",\n  \"created\": 1704925779,\n  \"model\": \"gpt-3.5-turbo-0613\",\n
      \ \"choices\": [\n    {\n      \"index\": 0,\n      \"message\": {\n        \"role\":
      \"assistant\",\n        \"content\": \"To start a new Tauri project, you can
      follow these steps:\\n\\n1. Install Node.js: Make sure you have Node.js installed
      on your machine. You can download it from the official Node.js website (https://nodejs.org).\\n\\n2.
      Install Tauri: Open your terminal or command prompt and run the following command
      to install Tauri globally:\\n   ```\\n   npm install -g tauri\\n   ```\\n\\n3.
      Create a new Tauri project: Navigate to the directory where you want to create
      your new Tauri project. Then, run the following command to create a new Tauri
      project:\\n   ```\\n   tauri init\\n   ```\\n\\n4. Configure your project: During
      the initialization process, you'll be prompted to answer a few questions to
      configure your project. You can choose the desired options based on your requirements.\\n\\n5.
      Build your project: Once the project is initialized, navigate into the project
      directory and run the following command to build your Tauri project:\\n   ```\\n
      \  tauri build\\n   ```\\n\\n6. Run your project: After the build process is
      complete, you can run your Tauri project using the following command:\\n   ```\\n
      \  tauri dev\\n   ```\\n\\nThese steps will help you get started with a new
      Tauri project. Remember to consult the official Tauri documentation (https://tauri.studio/)
      for more detailed information and additional configuration options.\"\n      },\n
      \     \"logprobs\": null,\n      \"finish_reason\": \"stop\"\n    }\n  ],\n
      \ \"usage\": {\n    \"prompt_tokens\": 63,\n    \"completion_tokens\": 295,\n
      \   \"total_tokens\": 358\n  },\n  \"system_fingerprint\": null\n}\n"
    headers:
      CF-Cache-Status:
      - DYNAMIC
      CF-RAY:
      - 84385c27eb03ef90-PDX
      Cache-Control:
      - no-cache, must-revalidate
      Connection:
      - keep-alive
      Content-Encoding:
      - gzip
      Content-Type:
      - application/json
      Date:
      - Wed, 10 Jan 2024 22:29:49 GMT
      Transfer-Encoding:
      - chunked
      openai-model:
      - gpt-3.5-turbo-0613
      openai-processing-ms:
      - '10154'
    http_version: HTTP/1.1
    status_code: 200
version: 1

```

Note that the request is for `gpt-3.5-turbo`, but the response comes back as `gpt-3.5-turbo-0613`. This is what we're looking to save. We go back and update the sample call file to reflect the expected output.

We do `make quicktype` again, and `src-python/zamm/api/models.py` gets updated.

We find that we have to create a custom OpenAI callback handler ourselves, because the default one does not save model information. We do so in `src-python/zamm/api/chat.py`:

```py
...
from typing import cast, Any

from langchain_community.callbacks import OpenAICallbackHandler
...


class CustomOpenAICallbackHandler(OpenAICallbackHandler):
    """Custom OpenAI callback handler."""

    model_name: str | None

    def __init__(self) -> None:
        """Initialize the OpenAI callback handler."""
        super().__init__()
        self.model_name = None

    def on_llm_end(self, response: LLMResult, **kwargs: Any) -> None:
        """Collect remaining metadata."""
        super().on_llm_end(response, **kwargs)
        self.model_name = response.llm_output["model_name"]


...

def chat(args: ChatArgs) -> ChatResponse:
    ...
    cb = CustomOpenAICallbackHandler()
    result = cast(ChatGeneration, llm.generate_prompt([prompt], callbacks=[cb]).generations[0][0])
    return ChatResponse(
        llm=cb.model_name or args.llm,
        response=ChatMessage(message=result.text, role="AI"),
        tokens=TokenMetadata(
            completion=cb.completion_tokens,
            prompt=cb.prompt_tokens,
            total=cb.total_tokens,
            cost=cb.total_cost,
        ),
    )

...
```

Unfortunately the tests fail. After digging in deeper to debug for a bit, we edit `langchain_openai/chat_models/base.py` of `langchain-openai` version 0.0.2 to print out the response:

```py
class ChatOpenAI(BaseChatModel):
    ...

    def _create_chat_result(self, response: Union[dict, BaseModel]) -> ChatResult:
        generations = []
        if not isinstance(response, dict):
            response = response.dict()
        print(response["model"])
        for res in response["choices"]:
            ...
        token_usage = response.get("usage", {})
        llm_output = {
            "token_usage": token_usage,
            "model_name": self.model_name,
            "system_fingerprint": response.get("system_fingerprint", ""),
        }
        return ChatResult(generations=generations, llm_output=llm_output)
```

`gpt-3.5-turbo-0613` gets printed correctly, but for some reason the returned model is actually being discarded completely in this code, with the original request model name being used instead.

At this point, we'll have to fork or patch `langchain-openai` ourselves, and the package doesn't even appear to be open-source. The whole point of using langchain was to enable us to quickly add new LLMs later on, and having to customize LLM-specific code to this level means that we might as well just use the OpenAI package directly. But if we're using the Python OpenAI package directly, we might as well just use the community-provided Rust package directly.

We commit everything just in case we want to revisit this path later, and go back to where we were before we started working on supporting the chat API in Python.

### Rust

The `async_openai` crate appears to be the most complete. We add that and `rVCR` for testing:

```bash
$ cargo add reqwest
$ cargo add async_openai
$ cargo add --dev rVCR
$ cargo add --dev reqwest_middleware
```

#### Capturing network calls for testing

Our goal is to capture a request as soon as possible so that we can start iterating on tests without waiting for slow network requests.

It turns out there is an [outstanding issue](https://github.com/ChorusOne/rvcr/issues/5) where reqwest-middleware version `0.2.x` breaks rVCR with a message such as

```
error[E0277]: expected a `Fn<(Request, &'a mut task_local_extensions::extensions::Extensions, Next<'a>)>` closure, found `VCRMiddleware`
  --> framework/tests/acceptance.rs:19:15
   |
19 |         .with(middleware)
   |          ---- ^^^^^^^^^^ expected an `Fn<(Request, &'a mut task_local_extensions::extensions::Extensions, Next<'a>)>` closure, found `VCRMiddleware`
   |          |
   |          required by a bound introduced by this call
   |
   = help: the trait `for<'a> Fn<(Request, &'a mut task_local_extensions::extensions::Extensions, Next<'a>)>` is not implemented for `VCRMiddleware`
   = note: required for `VCRMiddleware` to implement `reqwest_middleware::Middleware`
```

We therefore install the latest `0.1.x` version instead, which is `0.1.6` according to [this page](https://lib.rs/crates/reqwest-middleware/versions).

```
[dev-dependencies]
...
reqwest-middleware = "0.1.6"
...
```

and

```bash
$ cargo generate-lockfile
```

During implementation, we realize that we need `async-openai` to make use of `reqwest_middleware` clients instead of `reqwest` clients so that we can swap in test clients. As such, we move `reqwest-middleware` to the regular dependencies section instead of the dev dependencies section.

We edit `src-tauri/src/commands/errors.rs` to introduce new errors:

```rs
use std::{fmt, sync::PoisonError};
use crate::setup::api_keys::Service;

...

#[derive(thiserror::Error, Debug)]
pub enum Error {
    ...
    #[error("Missing API key for {service}")]
    MissingApiKey {
        service: Service,
    },
    #[error("Lock poisoned")]
    Poison {},
    ...
    #[error(transparent)]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error(transparent)]
    OpenAI {
        #[from]
        source: async_openai::error::OpenAIError,
    },
    ...
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Self::Poison {}
    }
}

...
```

where the Poison error is something we copied from [this answer](https://stackoverflow.com/a/68804367) due to the presence of lifetime and type parameters that would make it impossible to derive automatically without unnecessarily complicating the `Error` enum to have lifetimes and the such.

We now create `src-tauri/src/commands/llms/chat.rs`:

```rs
use crate::commands::errors::ZammResult;
use crate::setup::api_keys::Service;
use crate::commands::Error;
use crate::ZammApiKeys;
use specta::specta;
use tauri::State;
use async_openai::config::OpenAIConfig;
use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
};


async fn chat_helper(zamm_api_keys: &ZammApiKeys, http_client: reqwest_middleware::ClientWithMiddleware) -> ZammResult<()> {
    let api_keys = zamm_api_keys.0.lock()?;
    if api_keys.openai.is_none() {
        return Err(Error::MissingApiKey {
            service: Service::OpenAI,
        });
    }

    let config = OpenAIConfig::new()
      .with_api_key(api_keys.openai.as_ref().unwrap());

    
    let openai_client = async_openai::Client::with_config(config).with_http_client(http_client);
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are ZAMM, a chat program. Respond in first person.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Hello, does this work?")
                .build()?
                .into(),
        ])
        .build()?;
    let response = openai_client.chat().create(request).await?;
    println!("{:#?}", response);
    Ok(())
}

#[tauri::command(async)]
#[specta]
pub async fn chat(api_keys: State<'_, ZammApiKeys>) -> ZammResult<()> {
    let http_client = reqwest::ClientBuilder::new().build()?;
    let client_with_middleware = reqwest_middleware::ClientBuilder::new(http_client)
        .build();
    chat_helper(&api_keys, client_with_middleware).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::setup::api_keys::ApiKeys;
    use std::sync::Mutex;
    use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
    use rvcr::{VCRMiddleware, VCRMode};
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_start_conversation() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys {
            openai: env::var("OPENAI_API_KEY").ok(),
        }));

        let recording_path = PathBuf::from("api/sample-call-requests/start-conversation.json");
        let is_recording = !recording_path.exists();
        let middleware: VCRMiddleware = if is_recording {
            VCRMiddleware::try_from(recording_path)
            .unwrap()
            .with_mode(VCRMode::Record)
            .with_modify_request(|req| {
                let header_blacklist = ["authorization"];

                // Overwrite query params with filtered ones
                req.headers = req.headers.clone().iter().filter_map(
                    |(k, v)| {
                        if header_blacklist.contains(&k.as_str()) {
                            None
                        } else {
                            Some((k.clone(), v.clone()))
                        }
                    }
                ).collect();
            }).with_modify_response(|resp| {
                let header_blacklist = ["set-cookie", "openai-organization"];

                // Overwrite query params with filtered ones
                resp.headers = resp.headers.clone().iter().filter_map(
                    |(k, v)| {
                        if header_blacklist.contains(&k.as_str()) {
                            None
                        } else {
                            Some((k.clone(), v.clone()))
                        }
                    }
                ).collect();
            })
        } else {
            VCRMiddleware::try_from(recording_path)
            .unwrap()
            .with_mode(VCRMode::Replay)
        };

        let vcr_client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
            .with(middleware)
            .build();

        let result = chat_helper(&api_keys, vcr_client).await;
        assert!(!result.is_ok());
    }
}

```

with just a debug print statement to see what output we're getting. We make the test fail so that the stdout output will show up. According to [this answer](https://stackoverflow.com/a/67156994), we have to use Tokio to get async tests to run in Rust:

```bash
$ cargo add --dev tokio --features macros
```

We pipe this command through to `src-tauri/src/commands/llms/mod.rs`:

```rs
mod chat;

pub use chat::chat;

```

and `src-tauri/src/commands/mod.rs`:

```rs
...
mod llms;

pub use errors::Error;
...
pub use llms::chat;
```

where we also re-export `errors::Error` for convenience. Now that the chat code file is accessible from the top-level of the project, `cargo build` will surface any compilation errors and make it easier for us to develop.

We need `async-openai` to make use of `request-middleware`, but the types cannot be converted into one another because `reqwest`'s Client is a struct, not a trait. As such, we clone `async-openai` into `forks/async-openai` and make changes on our own branch:

```bash
$ mkdir -p forks
$ cd forks
$ git clone git@github.com:amosjyng/async-openai.git
$ git checkout -b rvcr
```

We edit their `forks/async-openai/async-openai/Cargo.toml` to add our new package in:

```toml
...

[dependencies]
...
reqwest-middleware = "0.1.6"
...
```

We edit `forks/async-openai/async-openai/src/client.rs` to use the middleware client where possible:

```rs
...

#[derive(Debug, Clone)]
/// Client is a container for config, backoff and http_client
/// used to make API calls.
pub struct Client<C: Config> {
    http_client: reqwest_middleware::ClientWithMiddleware,
    streaming_http_client: reqwest::Client,
    ...
}

impl Client<OpenAIConfig> {
    /// Client with default [OpenAIConfig]
    pub fn new() -> Self {
        Self {
            http_client: reqwest_middleware::ClientBuilder::new(
                reqwest::Client::new(),
            ).build(),
            streaming_http_client: reqwest::Client::new(),
            ...
        }
    }
}

impl<C: Config> Client<C> {
    /// Create client with [OpenAIConfig] or [crate::config::AzureConfig]
    pub fn with_config(config: C) -> Self {
        Self {
            http_client: reqwest_middleware::ClientBuilder::new(
                reqwest::Client::new(),
            ).build(),
            streaming_http_client: reqwest::Client::new(),
            ...
        }
    }

    /// Provide your own [client] to make HTTP requests with.
    ///
    /// [client]: reqwest_middleware::ClientWithMiddleware
    pub fn with_http_client(mut self, http_client: reqwest_middleware::ClientWithMiddleware) -> Self {
        self.http_client = http_client;
        self
    }

    ...

    /// Execute a HTTP request and retry on rate limit
    ///
    /// ...
    async fn execute_raw<M, Fut>(&self, request_maker: M) -> Result<Bytes, OpenAIError>
    where
        M: Fn() -> Fut,
        Fut: core::future::Future<Output = Result<reqwest::Request, OpenAIError>>,
    {
            ...
            let response = client
                .execute(request)
                .await
                .map_err(OpenAIError::ReqwestMiddleware)
                ...;
            ...
    }

    ...

    /// Make HTTP POST request to receive SSE
    pub(crate) async fn post_stream<I, O>(
        &self,
        ...
    ) -> Pin<Box<dyn Stream<Item = Result<O, OpenAIError>> + Send>>
    where
        ...
    {
        let event_source = self
            .streaming_http_client
            ...
            .eventsource()
            .unwrap();

        stream(event_source).await
    }

    /// Make HTTP GET request to receive SSE
    pub(crate) async fn get_stream<Q, O>(
        &self,
        ...
    ) -> Pin<Box<dyn Stream<Item = Result<O, OpenAIError>> + Send>>
    where
        ...
    {
        let event_source = self
            .streaming_http_client
            ...
            .eventsource()
            .unwrap();

        stream(event_source).await
    }
}

...
```

We retain the `reqwest::Client` for streaming requests because `reqwest_middleware` does not support streaming requests.

We edit `forks/async-openai/async-openai/src/error.rs` to add a new error type from the new middleware crate:

```rs
...

#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    /// Underlying error from reqwest library after an API call was made
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("http error: {0}")]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    ...
}

...
```

We commit, push, and tag this commit for easy future reference from our project:

```bash
$ git tag zamm/v0.0.0
$ git push --tags
```

We edit our own `src-tauri/Cargo.toml` to feature our custom fork of async-openai:

```toml
...

[patch.crates-io]
async-openai = { path = "../forks/async-openai/async-openai" }
```

We finally get things running, and after filtering out forbidden headers from both the request and the response, we end up with `src-tauri/api/sample-call-requests/start-conversation.json`:

```json
{
  "http_interactions": [
    {
      "response": {
        "body": {
          "encoding": null,
          "string": "{\n  \"id\": \"chatcmpl-8fsDA6VQfreZpjatOXPffkmyC6HO6\",\n  \"object\": \"chat.completion\",\n  \"created\": 1704990528,\n  \"model\": \"gpt-3.5-turbo-0613\",\n  \"choices\": [\n    {\n      \"index\": 0,\n      \"message\": {\n        \"role\": \"assistant\",\n        \"content\": \"Hello! I'm ZAMM, a chat program. I'm here to help and provide information. How can I assist you today?\"\n      },\n      \"logprobs\": null,\n      \"finish_reason\": \"stop\"\n    }\n  ],\n  \"usage\": {\n    \"prompt_tokens\": 32,\n    \"completion_tokens\": 28,\n    \"total_tokens\": 60\n  },\n  \"system_fingerprint\": null\n}\n"
        },
        "http_version": "1.1",
        "status": {
          "code": 200,
          "message": "OK"
        },
        "headers": {
          "x-request-id": [
            "f77666796e1630fbf7197f9ff5c960ab"
          ],
          "x-ratelimit-limit-tokens": [
            "60000"
          ],
          "content-type": [
            "application/json"
          ],
          "openai-processing-ms": [
            "1326"
          ],
          "access-control-allow-origin": [
            "*"
          ],
          "openai-version": [
            "2020-10-01"
          ],
          "cf-ray": [
            "843e88f25dc4ef20-PDX"
          ],
          "x-ratelimit-reset-tokens": [
            "37ms"
          ],
          "openai-model": [
            "gpt-3.5-turbo-0613"
          ],
          "x-ratelimit-reset-requests": [
            "8.64s"
          ],
          "connection": [
            "keep-alive"
          ],
          "x-ratelimit-limit-requests": [
            "10000"
          ],
          "cf-cache-status": [
            "DYNAMIC"
          ],
          "alt-svc": [
            "h3=\":443\"; ma=86400"
          ],
          "x-ratelimit-remaining-tokens": [
            "59963"
          ],
          "date": [
            "Thu, 11 Jan 2024 16:28:49 GMT"
          ],
          "x-ratelimit-remaining-requests": [
            "9999"
          ],
          "strict-transport-security": [
            "max-age=15724800; includeSubDomains"
          ],
          "cache-control": [
            "no-cache, must-revalidate"
          ],
          "server": [
            "cloudflare"
          ],
          "content-length": [
            "552"
          ]
        }
      },
      "request": {
        "uri": "https://api.openai.com/v1/chat/completions",
        "body": {
          "encoding": null,
          "string": "{\"messages\":[{\"content\":\"You are ZAMM, a chat program. Respond in first person.\",\"role\":\"system\"},{\"content\":\"Hello, does this work?\",\"role\":\"user\"}],\"model\":\"gpt-3.5-turbo\"}"
        },
        "method": "post",
        "headers": {
          "openai-beta": [
            "assistants=v1"
          ],
          "content-type": [
            "application/json"
          ]
        }
      },
      "recorded_at": "Thu, 11 Jan 2024 16:28:49 +0000"
    }
  ],
  "recorded_with": "rVCR 0.1.5"
}
```

The test fails with the output

```
CreateChatCompletionResponse {
    id: "chatcmpl-8fsDA6VQfreZpjatOXPffkmyC6HO6",
    choices: [
        ChatChoice {
            index: 0,
            message: ChatCompletionResponseMessage {
                content: Some(
                    "Hello! I'm ZAMM, a chat program. I'm here to help and provide information. How can I assist you today?",
                ),
                tool_calls: None,
                role: Assistant,
                function_call: None,
            },
            finish_reason: Some(
                Stop,
            ),
            logprobs: None,
        },
    ],
    created: 1704990528,
    model: "gpt-3.5-turbo-0613",
    system_fingerprint: None,
    object: "chat.completion",
    usage: Some(
        CompletionUsage {
            prompt_tokens: 32,
            completion_tokens: 28,
            total_tokens: 60,
        },
    ),
}
thread 'commands::llms::chat::tests::test_start_conversation' panicked at 'assertion failed: !result.is_ok()', src/commands/llms/chat.rs:119:9
```

Now that we can see clearly what data we're working with, we should create the database models for it. We try to commit some of our work first, but see this message:

```bash
$ gi add forks/                                                    
warning: adding embedded git repository: forks/async-openai
hint: You've added another git repository inside your current repository.
hint: Clones of the outer repository will not contain the contents of
hint: the embedded repository and will not know how to obtain it.
hint: If you meant to add a submodule, use:
hint: 
hint:   git submodule add <url> forks/async-openai
hint: 
hint: If you added this path by mistake, you can remove it from the
hint: index with:
hint: 
hint:   git rm --cached forks/async-openai
hint: 
hint: See "git help submodule" for more information.
```

We follow the instructions:

```bash
$ git rm --cached forks/async-openai
error: the following file has staged content different from both the
file and the HEAD:
    forks/async-openai
(use -f to force removal)
$ git rm -f --cached forks/async-openai
rm 'forks/async-openai'
$ git submodule add git@github.com:amosjyng/async-openai.git forks/async-openai
Adding existing repo at 'forks/async-openai' to the index
```

To avoid the unused warnings, we pipe our new command all the way through to `src-tauri/src/main.rs`:

```rs
...
use commands::{
    chat, ...
};

...

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            ...,
            chat
        ],
        "../src-svelte/src/lib/bindings.ts",
    )
    .unwrap();

    ...

    tauri::Builder::default()
        ...
        .invoke_handler(tauri::generate_handler![
            ...,
            chat
        ])
        ...;
}
```

We are faced with a compilation error next:

```
    Checking zamm v0.0.0 (/root/zamm/src-tauri)
error: future cannot be sent between threads safely
   --> src/commands/llms/chat.rs:46:1
    |
46  |   #[tauri::command(async)]
    |   ^^^^^^^^^^^^^^^^^^^^^^^^ future returned by `chat` is not `Send`
    |
   ::: src/main.rs:56:25
    |
56  |           .invoke_handler(tauri::generate_handler![
    |  _________________________-
57  | |             greet,
58  | |             get_api_keys,
59  | |             set_api_key,
...   |
64  | |             chat
65  | |         ])
    | |_________- in this macro invocation
    |
    = help: within `impl futures::Future<Output = std::result::Result<(), commands::errors::Error>>`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, setup::api_keys::ApiKeys>`
note: future is not `Send` as this value is used across an await
   --> src/commands/llms/chat.rs:41:57
    |
17  |     let api_keys = zamm_api_keys.0.lock()?;
    |         -------- has type `std::sync::MutexGuard<'_, setup::api_keys::ApiKeys>` which is not `Send`
...
41  |     let response = openai_client.chat().create(request).await?;
    |                                                         ^^^^^ await occurs here, with `api_keys` maybe used later
...
44  | }
    | - `api_keys` is later dropped here
note: required by a bound in `tauri::command::private::ResultFutureTag::future`
   --> /root/.asdf/installs/rust/1.71.1/registry/src/index.crates.io-6f17d22bba15001f/tauri-1.5.4/src/command.rs:293:42
    |
289 |     pub fn future<T, E, F>(self, value: F) -> impl Future<Output = Result<Value, InvokeError>>
    |            ------ required by a bound in this associated function
...
293 |       F: Future<Output = Result<T, E>> + Send,
    |                                          ^^^^ required by this bound in `ResultFutureTag::future`
    = note: this error originates in the macro `__cmd__chat` which comes from the expansion of the macro `tauri::generate_handler` (in Nightly builds, run with -Z macro-backtrace for more info)

```

We see from [this answer](https://stackoverflow.com/a/67277503) that it appears we may need to use a different mutex after all with a different executor. We try to use `tokio::sync::Mutex` instead of `std::sync::Mutex`, and start by editing `src-tauri/Cargo.toml` to move `tokio` from the dev dependencies section into the regular dependencies:

```toml
...

[dependencies]
...
tokio = { version = "1.35.1", features = ["macros"] }

...
```

We use the new mutex in `src-tauri/src/main.rs`; the name is the same, so we only need to change the import:

```rs
...
use tokio::sync::Mutex;
...
```

We edit `src-tauri/src/commands/llms/chat.rs` to use `await` instead of `lock()?`:

```rs
...

async fn chat_helper(
    zamm_api_keys: &ZammApiKeys,
    ...
) -> ZammResult<()> {
    let api_keys = zamm_api_keys.0.lock().await;
    ...
}

...

#[cfg(test)]
mod tests {
    ...
    use tokio::sync::Mutex;

    ...
}
```

We propagate these changes to the rest of the project. As we see from [this piece](https://tauri.app/v1/guides/features/command/#async-commands) of Tauri documentation, we need to make some changes such as using a result. As such, some files such as `src-tauri/src/commands/keys/get.rs` will see greater changes:

```rs
use crate::commands::errors::ZammResult;
...

async fn get_api_keys_helper(zamm_api_keys: &ZammApiKeys) -> ApiKeys {
    zamm_api_keys.0.lock().await.clone()
}

#[tauri::command(async)]
#[specta]
pub async fn get_api_keys(api_keys: State<'_, ZammApiKeys>) -> ZammResult<ApiKeys> {
    Ok(get_api_keys_helper(&api_keys).await)
}

#[cfg(test)]
pub mod tests {
    ...
    use tokio::sync::Mutex;

    pub async fn check_get_api_keys_sample(file_prefix: &str, rust_input: &ZammApiKeys) {
        ...

        let actual_result = get_api_keys_helper(rust_input).await;
        ...
    }

    #[tokio::test]
    async fn test_get_empty_keys() {
        ...

        check_get_api_keys_sample(
            ...
        ).await;
    }

    #[tokio::test]
    async fn test_get_openai_key() {
        ...

        check_get_api_keys_sample(
            ...
        ).await;
    }
}
```

Now we encounter various errors with the `async-openai` crate we forked:

```
warning: unused import: `crate::types::ChatCompletionFunctions`
 --> /root/zamm/forks/async-openai/async-openai/src/types/assistant_impls.rs:1:5
  |
1 | use crate::types::ChatCompletionFunctions;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
...
```

From [this question](https://stackoverflow.com/a/63593012), which links to various issues such as [this one](https://github.com/rust-lang/cargo/issues/8546) that's still open, this is an outstanding problem with cargo that we'll have to work around. We edit `src-tauri/clippy.sh` to only consider the bit after the `Checking zamm v0.0.0 (/root/zamm/src-tauri)` part of the output:

```sh
...
zamm_output=$(echo "$clippy_output" | awk '/Checking zamm /{flag=1; next} flag')

if [[ $zamm_output == *"warning"* ]]; then
  exit 1;
fi

```

Now our tests are failing, perhaps because `rvcr` is not applying the transformations we defined to the recording. We try to fork that library as well, cloning our own version of it rather than the original repo.

```bash
$ git submodule add git@github.com:ChorusOne/rvcr.git forks/rvcr
$ <reset repo...>
$ git submodule add git@github.com:amosjyng/rvcr.git forks/rvcr 
fatal: A git directory for 'forks/rvcr' is found locally with remote(s):
  origin        git@github.com:ChorusOne/rvcr.git
If you want to reuse this local git directory instead of cloning again from
  git@github.com:amosjyng/rvcr.git
use the '--force' option. If the local git directory is not the correct repo
or you are unsure what this means choose another name with the '--name' option.
$ git submodule add --force git@github.com:amosjyng/rvcr.git forks/rvcr
$ git checkout -b zamm-fixes
```

We then edit `forks/rvcr/src/lib.rs` to provide a better debugging experience:

```rs
...

impl VCRMiddleware {
    ...

    fn header_values_to_string(&self, header_values: Option<&Vec<String>>) -> String {
        match header_values {
            Some(values) => values.join(", "),
            None => "<MISSING>".to_string(),
        }
    }

    fn find_response_in_vcr(&self, req: vcr_cassette::Request) -> Option<vcr_cassette::Response> {
        ...
        // save diff in a string for debugging purposes
        let mut diff = String::new();
        for interaction in iteractions {
            if interaction.request == req {
                return Some(interaction.response.clone());
            } else {
                diff.push_str(&format!(
                    "Unmatched {method:?} to {uri}:\n",
                    method = interaction.request.method,
                    uri = interaction.request.uri.as_str()
                ));
                if interaction.request.method != req.method {
                    diff.push_str(&format!(
                        "  Method differs: recorded {expected:?}, got {got:?}\n",
                        expected = interaction.request.method,
                        got = req.method
                    ));
                }
                if interaction.request.uri != req.uri {
                    diff.push_str("  URI differs:\n");
                    diff.push_str(&format!(
                        "    recorded: \"{}\"\n",
                        interaction.request.uri.as_str()
                    ));
                    diff.push_str(&format!("    got:      \"{}\"\n", req.uri.as_str()));
                }
                if interaction.request.headers != req.headers {
                    diff.push_str("  Headers differ:\n");
                    for (recorded_header_name, recorded_header_values) in
                        &interaction.request.headers
                    {
                        let expected = self.header_values_to_string(Some(recorded_header_values));
                        let got =
                            self.header_values_to_string(req.headers.get(recorded_header_name));
                        if expected != got {
                            diff.push_str(&format!("    {}:\n", recorded_header_name));
                            diff.push_str(&format!("      recorded: \"{}\"\n", expected));
                            diff.push_str(&format!("      got:      \"{}\"\n", got));
                        }
                    }
                    for (got_header_name, got_header_values) in &req.headers {
                        if !interaction.request.headers.contains_key(got_header_name) {
                            let got = self.header_values_to_string(Some(got_header_values));
                            diff.push_str(&format!("    {}:\n", got_header_name));
                            diff.push_str(&format!("      recorded: <MISSING>\n"));
                            diff.push_str(&format!("      got:      \"{}\"\n", got));
                        }
                    }
                }
                if interaction.request.body != req.body {
                    diff.push_str("  Body differs:\n");
                    diff.push_str(&format!(
                        "    recorded: \"{}\"\n",
                        interaction.request.body.string
                    ));
                    diff.push_str(&format!("    got:      \"{}\"\n", req.body.string));
                }
            }
        }
        eprintln!("{}", diff);
        None
    }

    ...
}

#[async_trait::async_trait]
impl Middleware for VCRMiddleware {
    async fn handle(
        &self,
        ...
    ) -> reqwest_middleware::Result<reqwest::Response> {
        ...

        match self.mode {
            VCRMode::Record => {
                ...
            }
            VCRMode::Replay => {
                let vcr_response = self.find_response_in_vcr(vcr_request).unwrap_or_else(|| {
                    panic!(
                        "Cannot find corresponding request in cassette {:?}",
                        self.path
                    )
                });
                ...
            }
        }
    }
}

...
```

With this, we are able to see that the problem is indeed that the request being compared against does not have the header stripped out:

```
Unmatched Post to https://api.openai.com/v1/chat/completions:
  Headers differ:
    authorization:
      recorded: <MISSING>
      got:      "Bearer sk-..."
```

As such, we go and fix our logic in `src-tauri/src/commands/llms/chat.rs` to make sure that the same middleware setup is always used, just with different recording modes:

```rs
    #[tokio::test]
    async fn test_start_conversation() {
        ...
        let vcr_mode = if is_recording {
            VCRMode::Record
        } else {
            VCRMode::Replay
        };
        let middleware = VCRMiddleware::try_from(recording_path)
                .unwrap()
                .with_mode(vcr_mode)
                ...;
        ...
    }
```

Now we decide that we want to say `<CENSORED>` instead of simply leaving the headers out, so that it's still clear what's actually being sent or received. We do some refactoring and work directly with the headers hashmap:

```bash
$ cargo add --dev vcr-cassette
```

and edit `src-tauri/src/commands/llms/chat.rs` while deciding that it's okay to set cookies after all as they are not login cookies:

```rs
...

#[cfg(test)]
mod tests {
    ...
    use vcr_cassette::Headers;

    const CENSORED: &str = "<CENSORED>";

    fn censor_headers(headers: &Headers, blacklisted_keys: &[&str]) -> Headers {
        return headers.clone()
            .iter()
            .map(|(k, v)| {
                if blacklisted_keys.contains(&k.as_str()) {
                    (k.clone(), vec![CENSORED.to_string()])
                } else {
                    (k.clone(), v.clone())
                }
            })
            .collect()
    }

    #[tokio::test]
    async fn test_start_conversation() {
        ...
        let middleware = VCRMiddleware::try_from(recording_path)
            ...
            .with_modify_request(|req| {
                req.headers = censor_headers(&req.headers, &["authorization"]);
            })
            .with_modify_response(|resp| {
                resp.headers = censor_headers(&resp.headers, &["openai-organization"]);
            });
        ...
    }
}
```

#### Database persistence

Now we create new database tables to store our chat API calls in:

```bash
$ diesel migration generate create_llm_calls
Creating migrations/2024-01-13-023144_create_llm_calls/up.sql
Creating migrations/2024-01-13-023144_create_llm_calls/down.sql
```

We edit `src-tauri/migrations/2024-01-13-023144_create_llm_calls/up.sql`:

```sql
CREATE TABLE llm_calls (
  id VARCHAR PRIMARY KEY NOT NULL,
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  provider VARCHAR NOT NULL,
  llm_requested VARCHAR NOT NULL,
  llm VARCHAR NOT NULL,
  temperature REAL NOT NULL,
  prompt_tokens INTEGER,
  response_tokens INTEGER,
  prompt TEXT NOT NULL,
  completion TEXT NOT NULL
)

```

and `src-tauri/migrations/2024-01-13-023144_create_llm_calls/down.sql`:

```sql
DROP TABLE llm_calls

```

and run

```bash
$ diesel migration run --database-url /root/.local/share/zamm/zamm.sqlite3
Running migration 2024-01-13-023144_create_llm_calls
```

We can see that `src-tauri/src/schema.rs` gets updated automatically:

```rs
...

diesel::table! {
    llm_calls (id) {
        id -> Text,
        timestamp -> Timestamp,
        provider -> Text,
        llm_requested -> Text,
        llm -> Text,
        temperature -> Float,
        prompt_tokens -> Nullable<Integer>,
        response_tokens -> Nullable<Integer>,
        prompt -> Text,
        completion -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(..., llm_calls,);

```

Next we have to define the models, but we prepare for this by first refactoring our model module to allow for multiple submodules. We rename `src-tauri/src/models.rs` to `src-tauri/src/models/api_keys.rs` and create `src-tauri/src/models/mod.rs`:

```rs
mod api_keys;

pub use api_keys::{ApiKey, NewApiKey};

```

We commit the specific above changes, and then continue. We initially create a wrapper around the OpenAI types for SQL expressions, because we can't directly implement that for types we don't own:

```rs
#[derive(AsExpression, FromSqlRow, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[diesel(sql_type = Text)]
pub struct OpenAiPrompt {
    #[serde(flatten)]
    pub prompt: Vec<ChatCompletionRequestMessage>,
}

impl Deref for OpenAiPrompt {
    type Target = Vec<ChatCompletionRequestMessage>;

    fn deref(&self) -> &Self::Target {
        &self.prompt
    }
}

impl ToSql<Text, Sqlite> for OpenAiPrompt
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json_str = serde_json::to_string(&self.prompt)?;
        out.set_value(json_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for OpenAiPrompt
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let json_str = String::from_sql(bytes)?;
        let parsed_json: Vec<ChatCompletionRequestMessage> = serde_json::from_str(&json_str)?;
        Ok(OpenAiPrompt { prompt: parsed_json })
    }
}

#[derive(AsExpression, FromSqlRow, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[diesel(sql_type = Text)]
pub struct OpenAiCompletion {
    #[serde(flatten)]
    pub completion: ChatCompletionResponseMessage,
}

impl Deref for OpenAiCompletion {
    type Target = ChatCompletionResponseMessage;

    fn deref(&self) -> &Self::Target {
        &self.completion
    }
}

impl ToSql<Text, Sqlite> for OpenAiCompletion
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json_str = serde_json::to_string(&self.completion)?;
        out.set_value(json_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for OpenAiCompletion
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let json_str = String::from_sql(bytes)?;
        let parsed_json: ChatCompletionResponseMessage = serde_json::from_str(&json_str)?;
        Ok(OpenAiCompletion { completion: parsed_json })
    }
}
```

However, because `async_openai` disables Serde tagging, the tests come up with the wrong type when results are deserialized from JSON. Additionally, given that the database is the one thing that will have implications for backwards compatibility, it seems best for us to retain full control over the format we use. Finally, using more generic types will make it easier to add in other LLMs from other providers.

As such, we create our own types in `src-tauri/src/models/llm_calls.rs`, along with convenience conversion functions:

```rs
use crate::commands::Error;
use crate::schema::llm_calls;
use crate::setup::api_keys::Service;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessageContent,
    ChatCompletionResponseMessage, Role,
};
use chrono::naive::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use serde_json;
use std::ops::Deref;
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    AsExpression,
    FromSqlRow,
    specta::Type,
)]
#[diesel(sql_type = Text)]
#[serde(tag = "role")]
pub enum ChatMessage {
    System { text: String },
    Human { text: String },
    AI { text: String },
}

impl TryFrom<ChatCompletionRequestMessage> for ChatMessage {
    type Error = Error;

    fn try_from(message: ChatCompletionRequestMessage) -> Result<Self, Self::Error> {
        match message {
            ChatCompletionRequestMessage::System(system_message) => {
                Ok(ChatMessage::System {
                    text: system_message.content,
                })
            }
            ChatCompletionRequestMessage::User(user_message) => {
                match user_message.content {
                    ChatCompletionRequestUserMessageContent::Text(text) => {
                        Ok(ChatMessage::Human { text })
                    }
                    ChatCompletionRequestUserMessageContent::Array(_) => {
                        Err(Error::UnexpectedOpenAiResponse {
                            reason: "Image chat not supported yet".to_string(),
                        })
                    }
                }
            }
            ChatCompletionRequestMessage::Assistant(assistant_message) => {
                match assistant_message.content {
                    Some(content) => Ok(ChatMessage::AI { text: content }),
                    None => Err(Error::UnexpectedOpenAiResponse {
                        reason: "AI function calls not supported yet".to_string(),
                    }),
                }
            }
            _ => Err(Error::UnexpectedOpenAiResponse {
                reason: "Only AI text chat is supported".to_string(),
            }),
        }
    }
}

impl TryFrom<ChatCompletionResponseMessage> for ChatMessage {
    type Error = Error;

    fn try_from(message: ChatCompletionResponseMessage) -> Result<Self, Self::Error> {
        let text = message.content.ok_or(Error::UnexpectedOpenAiResponse {
            reason: "No content in response".to_string(),
        })?;
        match message.role {
            Role::System => Ok(ChatMessage::System { text }),
            Role::User => Ok(ChatMessage::Human { text }),
            Role::Assistant => Ok(ChatMessage::AI { text }),
            _ => Err(Error::UnexpectedOpenAiResponse {
                reason: "Only AI text chat is supported".to_string(),
            }),
        }
    }
}

impl ToSql<Text, Sqlite> for ChatMessage
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json_str = serde_json::to_string(&self)?;
        out.set_value(json_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for ChatMessage
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let json_str = String::from_sql(bytes)?;
        let parsed_json: Self = serde_json::from_str(&json_str)?;
        Ok(parsed_json)
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    AsExpression,
    FromSqlRow,
    specta::Type,
)]
#[diesel(sql_type = Text)]
pub struct ChatPrompt {
    messages: Vec<ChatMessage>,
}

impl Deref for ChatPrompt {
    type Target = Vec<ChatMessage>;

    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

impl TryFrom<Vec<ChatCompletionRequestMessage>> for ChatPrompt {
    type Error = Error;

    fn try_from(
        messages: Vec<ChatCompletionRequestMessage>,
    ) -> Result<Self, Self::Error> {
        let messages = messages
            .into_iter()
            .map(|message| message.try_into())
            .collect::<Result<Vec<ChatMessage>, Self::Error>>()?;
        Ok(ChatPrompt { messages })
    }
}

impl ToSql<Text, Sqlite> for ChatPrompt
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json_str = serde_json::to_string(&self)?;
        out.set_value(json_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for ChatPrompt
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let json_str = String::from_sql(bytes)?;
        let parsed_json: Self = serde_json::from_str(&json_str)?;
        Ok(parsed_json)
    }
}
```

As shown in [`diesel.md`](/zamm/resources/tutorials/libraries/rust/diesel.md), we use `EntityId` as a wrapper around UUID, except that we name the `uuid` field `id` for JSON serialization and implement `Deref` for it because we want to pretend as if the wrapper didn't actually exist:

```rs
#[derive(
    AsExpression, FromSqlRow, Debug, Serialize, Deserialize, Clone, specta::Type,
)]
#[diesel(sql_type = Text)]
pub struct EntityId {
    #[serde(rename = "id")]
    pub uuid: Uuid,
}

impl Deref for EntityId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.uuid
    }
}

impl ToSql<Text, Sqlite> for EntityId
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let uuid_str = self.uuid.to_string();
        out.set_value(uuid_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for EntityId
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let uuid_str = String::from_sql(bytes)?;
        let parsed_uuid = Uuid::parse_str(&uuid_str)?;
        Ok(EntityId { uuid: parsed_uuid })
    }
}
```

We define the rest of the groupings and the data structure we want to be represented in the database:

```rs
#[derive(Debug, Serialize, Deserialize, Clone, specta::Type)]
pub struct Llm {
    pub name: String,
    pub requested: String,
    pub provider: Service,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct Request {
    pub prompt: ChatPrompt,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct Response {
    pub completion: ChatMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TokenMetadata {
    pub prompt_tokens: Option<i32>,
    pub response_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct LlmCall {
    #[serde(flatten)]
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub llm: Llm,
    pub request: Request,
    pub response: Response,
    pub token_metadata: TokenMetadata,
}
```

It is not clear how to represent this as a Disel row, so we create separate data structures for Diesel instead:

```rs
#[derive(Debug, Queryable, Selectable, Clone)]
#[diesel(table_name = llm_calls)]
pub struct LlmCallRow {
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub provider: Service,
    pub llm_requested: String,
    pub llm: String,
    pub temperature: f32,
    pub prompt_tokens: Option<i32>,
    pub response_tokens: Option<i32>,
    pub prompt: ChatPrompt,
    pub completion: ChatMessage,
}

#[derive(Insertable)]
#[diesel(table_name = llm_calls)]
pub struct NewLlmCallRow<'a> {
    pub id: &'a EntityId,
    pub timestamp: &'a NaiveDateTime,
    pub provider: &'a Service,
    pub llm_requested: &'a str,
    pub llm: &'a str,
    pub temperature: &'a f32,
    pub prompt_tokens: Option<&'a i32>,
    pub response_tokens: Option<&'a i32>,
    pub prompt: &'a ChatPrompt,
    pub completion: &'a ChatMessage,
}
```

and add ways to convert between the two:

```rs
impl LlmCall {
    pub fn as_sql_row(&self) -> NewLlmCallRow {
        NewLlmCallRow {
            id: &self.id,
            timestamp: &self.timestamp,
            provider: &self.llm.provider,
            llm_requested: &self.llm.requested,
            llm: &self.llm.name,
            temperature: &self.request.temperature,
            prompt_tokens: self.token_metadata.prompt_tokens.as_ref(),
            response_tokens: self.token_metadata.response_tokens.as_ref(),
            prompt: &self.request.prompt,
            completion: &self.response.completion,
        }
    }
}

impl From<LlmCallRow> for LlmCall {
    fn from(row: LlmCallRow) -> Self {
        let id = row.id;
        let timestamp = row.timestamp;
        let llm = Llm {
            name: row.llm,
            requested: row.llm_requested,
            provider: row.provider,
        };
        let request = Request {
            prompt: row.prompt,
            temperature: row.temperature,
        };
        let response = Response {
            completion: row.completion,
        };
        let token_metadata = TokenMetadata {
            prompt_tokens: row.prompt_tokens,
            response_tokens: row.response_tokens,
        };
        LlmCall {
            id,
            timestamp,
            llm,
            request,
            response,
            token_metadata,
        }
    }
}

```

Note that we have to make use of a number of new dependencies or new features to existing dependencies in `src-tauri/Cargo.toml`:

```toml
...

[dependencies]
...
diesel = { ..., features = [..., "chrono"] }
...
uuid = { ..., features = [..., "serde"] }
specta = { ..., features = ["uuid", "chrono"] }
...
chrono = "0.4.31"

...
```

Note that we also need to add this new error in `src-tauri/src/commands/errors.rs`:

```rs
#[derive(thiserror::Error, Debug)]
pub enum Error {
    ...
    #[error("Unexpected JSON: {reason}")]
    UnexpectedOpenAiResponse { reason: String },
    ...
}
```

We export the new modules in `src-tauri/src/models/mod.rs`:

```rs
pub mod api_keys;
pub mod llm_calls;

...
```

We want to save the chat request afterwards, so we edit our fork of `forks/async-openai/async-openai/src/chat.rs` to make sure that the request is only borrowed and not moved into the function call, so that it is still available for use afterwards:

```rs
impl<'c, C: Config> Chat<'c, C> {
    ...

    /// Creates a model response for the given chat conversation.
    pub async fn create(
        &self,
        request: &CreateChatCompletionRequest,
    ) -> ... {
        ...
    }

    ...
}
```

We commit, cherry-pick and create the PR [here](https://github.com/64bit/async-openai/pull/181) in case the upstream maintainers are interested. We go back to the parent directory and edit `src-tauri/src/commands/llms/chat.rs` to save the chat interaction to the database:

```rs
...
use crate::models::llm_calls::{
    EntityId, Llm, LlmCall, Request, Response, TokenMetadata,
};
use crate::schema::llm_calls;
...
use crate::{ZammApiKeys, ZammDatabase};
...
use diesel::RunQueryDsl;
...
use uuid::Uuid;

async fn chat_helper(
    ...,
    zamm_db: &ZammDatabase,
    ...
) -> ZammResult<LlmCall> {
    ...

    let db = &mut zamm_db.0.lock().await;

    let config = OpenAIConfig::new().with_api_key(api_keys.openai.as_ref().unwrap());
    let requested_model = "gpt-3.5-turbo";
    let temperature = 1.0;

    ...
    let request = CreateChatCompletionRequestArgs::default()
        .model(requested_model)
        .temperature(temperature)
        ...;
    let response = openai_client.chat().create(&request).await?;

    let token_metadata = TokenMetadata {
        prompt_tokens: response
            .usage
            .as_ref()
            .map(|usage| usage.prompt_tokens as i32),
        response_tokens: response
            .usage
            .as_ref()
            .map(|usage| usage.completion_tokens as i32),
    };
    let sole_choice = response
        .choices
        .first()
        .ok_or(Error::UnexpectedOpenAiResponse {
            reason: "Zero choices".to_owned(),
        })?
        .message
        .to_owned();
    let llm_call = LlmCall {
        id: EntityId {
            uuid: Uuid::new_v4(),
        },
        timestamp: chrono::Utc::now().naive_utc(),
        llm: Llm {
            provider: Service::OpenAI,
            name: response.model.clone(),
            requested: requested_model.to_owned(),
        },
        request: Request {
            temperature,
            prompt: request.messages.try_into()?,
        },
        response: Response {
            completion: sole_choice.try_into()?,
        },
        token_metadata,
    };

    if let Some(conn) = db.as_mut() {
        diesel::insert_into(llm_calls::table)
            .values(llm_call.as_sql_row())
            .execute(conn)?;
    } // todo: warn users if DB write unsuccessful

    Ok(llm_call)
}

#[tauri::command(async)]
#[specta]
pub async fn chat(
    api_keys: State<'_, ZammApiKeys>,
    database: State<'_, ZammDatabase>,
) -> ZammResult<LlmCall> {
    ...
    chat_helper(&api_keys, &database, client_with_middleware).await
}

#[cfg(test)]
mod tests {
    ...
    use crate::models::llm_calls::{ChatMessage, LlmCallRow};
    ...
    use crate::setup::db::MIGRATIONS;
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;
    ...

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    pub fn setup_zamm_db() -> ZammDatabase {
        ZammDatabase(Mutex::new(Some(setup_database())))
    }

    async fn get_llm_call(db: &ZammDatabase, call_id: &EntityId) -> LlmCall {
        use crate::schema::llm_calls::dsl::*;
        let mut conn_mutex = db.0.lock().await;
        let conn = conn_mutex.as_mut().unwrap();
        llm_calls
            .filter(id.eq(call_id))
            .first::<LlmCallRow>(conn)
            .unwrap()
            .into()
    }

    #[tokio::test]
    async fn test_start_conversation() {
        ...

        let db = setup_zamm_db();
        let result = chat_helper(&api_keys, &db, vcr_client).await;
        assert!(result.is_ok(), "Error: {:?}", result.err());
        let ok_result = result.unwrap();
        assert!(ok_result.request.prompt.len() > 0);
        match &ok_result.response.completion {
            ChatMessage::AI { text } => assert!(!text.is_empty()),
            _ => panic!("Unexpected response type"),
        }

        // check that it made it into the database
        let stored_llm_call = get_llm_call(&db, &ok_result.id).await;
        assert_eq!(stored_llm_call.request.prompt, ok_result.request.prompt);
        assert_eq!(
            stored_llm_call.response.completion,
            ok_result.response.completion
        );
    }
}

```

We re-record `src-tauri/api/sample-call-requests/start-conversation.json` to use the new temperature specification.

Now that the tests pass, we can be reasonably confident that database persistence is working.

#### API boundary

Now we can iterate on creating the API boundary. We create `src-tauri/api/sample-calls/chat-start-conversation.yaml` to define how we want the API to look:

```yaml
request:
  - chat
  - >
    {
      "provider": "OpenAI",
      "llm": "gpt-3.5-turbo",
      "temperature": null,
      "prompt": {
        "messages": [
          {
            "role": "System",
            "text": "You are ZAMM, a chat program. Respond in first person."
          },
          {
            "role": "Human",
            "text": "Hello, does this work?"
          }
        ]
      }
    }
response:
  message: >
    {
      "id": "d5ad1e49-f57f-4481-84fb-4d70ba8a7a74",
      "timestamp": "2024-01-16T08:50:19.738093890",
      "llm": {
        "name": "gpt-3.5-turbo-0613",
        "requested": "gpt-3.5-turbo",
        "provider": "OpenAI"
      },
      "request": {
        "prompt": {
          "messages": [
            {
              "role": "System",
              "text": "You are ZAMM, a chat program. Respond in first person."
            },
            {
              "role": "Human",
              "text": "Hello, does this work?"
            }
          ]
        },
        "temperature": 1.0
      },
      "response": {
        "completion": {
          "role": "AI",
          "text": "Hello! I'm ZAMM, a chat program. I'm here to assist you. What can I help you with today?"
        }
      },
      "token_metadata": {
        "prompt_tokens": 32,
        "response_tokens": 27
      }
    }

```

The `request` part in the response is technically redundant as it doesn't need to refer to anything, but we don't need to optimize that away right now.

We need to define some additional conversion functions, so we edit `src-tauri/src/models/llm_calls.rs`:

```rs
...
use async_openai::types::{
    ..., ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
    ChatCompletionRequestAssistantMessage
};
...

impl Into<ChatCompletionRequestMessage> for ChatMessage {
    fn into(self) -> ChatCompletionRequestMessage {
        match self {
            ChatMessage::System { text } => {
                ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessage {
                        content: text,
                        role: Role::System,
                        ..Default::default()
                    }
                )
            }
            ChatMessage::Human { text } => {
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(
text
                    ),
                    role: Role::User,
                    ..Default::default()
                })
            }
            ChatMessage::AI { text } => {
                ChatCompletionRequestMessage::Assistant(
                    ChatCompletionRequestAssistantMessage {
                        content: Some(text),
                        role: Role::Assistant,
                        ..Default::default()
                    }
                )
            }
        }
    }
}

...

impl Into<Vec<ChatCompletionRequestMessage>> for ChatPrompt {
    fn into(self) -> Vec<ChatCompletionRequestMessage> {
        self.messages
            .into_iter()
            .map(|message| message.into())
            .collect()
    }
}

...

#[derive(..., Clone, ...)]
pub struct Request {
    ...
}

#[derive(..., Clone, ...)]
pub struct Response {
    ...
}

#[derive(..., Clone, ...)]
pub struct LlmCall {
    ...
}
```

Finally, we edit `src-tauri/src/commands/llms/chat.rs` to actually handle a real API call instead of the fake one we've hard-coded into it:

```rs
...
use crate::models::llm_calls::{
    ..., ChatPrompt
};
...
use async_openai::types::{
    CreateChatCompletionRequestArgs, ChatCompletionRequestMessage
};
...

async fn chat_helper(
    ...,
    provider: Service,
    llm: String,
    temperature: Option<f32>,
    prompt: ChatPrompt,
    ...,
) -> ZammResult<LlmCall> {
    ...

    let config = match provider {
        Service::OpenAI => {
            let openai_api_key = api_keys.openai.as_ref().ok_or(Error::MissingApiKey { service: Service::OpenAI })?;
            OpenAIConfig::new().with_api_key(openai_api_key)
        },
    };

    let requested_model = llm;
    let requested_temperature = temperature.unwrap_or(1.0);

    let openai_client =
        async_openai::Client::with_config(config).with_http_client(http_client);
    let messages: Vec<ChatCompletionRequestMessage> = prompt.into();
    let request = CreateChatCompletionRequestArgs::default()
        .model(&requested_model)
        .temperature(requested_temperature)
        .messages(messages)
        .build()?;

    ...

    let llm_call = LlmCall {
        ...
        request: Request {
            temperature: requested_temperature,
            ...
        },
        ...
    }

    ...
}

#[tauri::command(async)]
#[specta]
pub async fn chat(
    ...,
    provider: Service,
    llm: String,
    temperature: Option<f32>,
    prompt: ChatPrompt,
) -> ZammResult<LlmCall> {
    ...
    chat_helper(..., provider, llm, temperature, prompt, ...).await
}

#[cfg(test)]
mod tests {
    ...
    use serde::{Deserialize, Serialize};
    use crate::sample_call::SampleCall;
    use std::fs;
    ...

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ChatRequest {
        provider: Service,
        llm: String,
        temperature: Option<f32>,
        prompt: ChatPrompt,
    }

    fn parse_request(request_str: &str) -> ChatRequest {
        serde_json::from_str(request_str).unwrap()
    }

    fn parse_response(response_str: &str) -> LlmCall {
        serde_json::from_str(response_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    #[tokio::test]
    async fn test_start_conversation() {
        ...
        let api_keys = if is_recording {
            ZammApiKeys(Mutex::new(ApiKeys {
                openai: env::var("OPENAI_API_KEY").ok(),
            }))
        } else {
            ZammApiKeys(Mutex::new(ApiKeys {
                openai: Some("dummy".to_string()),
            }))
        };

        ...
        // end dependencies setup

        let sample = read_sample("api/sample-calls/chat-start-conversation.yaml");
        assert_eq!(sample.request.len(), 2);
        assert_eq!(sample.request[0], "chat");

        let request = parse_request(&sample.request[1]);

        let result = chat_helper(&api_keys, &db, request.provider, request.llm, request.temperature, request.prompt, vcr_client).await;
        ...

        // check that the API call returns the expected JSON
        let expected_llm_call = parse_response(&sample.response.message);
        // swap out non-deterministic parts before JSON comparison
        let deterministic_llm_call = LlmCall {
            id: expected_llm_call.id,
            timestamp: expected_llm_call.timestamp,
            ..ok_result.clone()
        };
        let actual_json = serde_json::to_string_pretty(&deterministic_llm_call).unwrap();
        let expected_json = sample.response.message.trim();
        assert_eq!(actual_json, expected_json);

        ...
    }
}

```

We make sure the tests pass now, and commit.

##### Flattening the JSON request/response

Since the nested nature of the `ChatPrompt` struct is just an artifact of the Rust type system, we can flatten it out in the JSON request/response. We can also make the `token_metadata` key less verbose. We edit `src-tauri/api/sample-calls/chat-start-conversation.yaml` to reflect what we want:

```yaml
request:
  - chat
  - >
    {
      ...,
      "prompt": [
        {
          "role": "System",
          "text": "You are ZAMM, a chat program. Respond in first person."
        },
        {
          "role": "Human",
          "text": "Hello, does this work?"
        }
      ]
    }
response:
  message: >
    {
      ...,
      "request": {
        "prompt": [
          {
            "role": "System",
            "text": "You are ZAMM, a chat program. Respond in first person."
          },
          {
            "role": "Human",
            "text": "Hello, does this work?"
          }
        ],
        ...
      },
      ...,
      "tokens": {
        "prompt": 32,
        "response": 27
      }
    }

```

We then edit `src-tauri/src/models/llm_calls.rs`:

```rs
...

#[derive(
    ...
)]
#[diesel(sql_type = Text)]
pub struct ChatPrompt {
    pub prompt: Vec<ChatMessage>,
}

impl Deref for ChatPrompt {
    ...

    fn deref(&self) -> &Self::Target {
        &self.prompt
    }
}

impl TryFrom<Vec<ChatCompletionRequestMessage>> for ChatPrompt {
    type Error = Error;

    fn try_from(
        ...
    ) -> Result<Self, Self::Error> {
        ...
        Ok(ChatPrompt { prompt: messages })
    }
}

impl From<ChatPrompt> for Vec<ChatCompletionRequestMessage> {
    fn from(val: ChatPrompt) -> Self {
        val.prompt
            ...
    }
}

...

#[derive(...)]
pub struct Request {
    #[serde(flatten)]
    pub prompt: ChatPrompt,
    ...
}

...

#[derive(...)]
pub struct TokenMetadata {
    pub prompt: Option<i32>,
    ...
}

...

#[derive(...)]
pub struct LlmCall {
    ...
    pub tokens: TokenMetadata,
}

impl LlmCall {
    pub fn as_sql_row(&self) -> NewLlmCallRow {
        NewLlmCallRow {
            ...
            prompt_tokens: self.tokens.prompt.as_ref(),
            response_tokens: self.tokens.response.as_ref(),
            ...
        }
    }
}

impl From<LlmCallRow> for LlmCall {
    fn from(row: LlmCallRow) -> Self {
        ...
        let token_metadata = TokenMetadata {
            prompt: row.prompt_tokens,
            response: row.response_tokens,
        };
        LlmCall {
            ...,
            tokens: token_metadata,
        }
    }
}
```

and `src-tauri/src/commands/llms/chat.rs`:

```rs
...
use crate::models::llm_calls::{
    ..., ChatMessage, ...
};
...

async fn chat_helper(
    ...,
    prompt: Vec<ChatMessage>,
    ...
) -> ZammResult<LlmCall> {
    ...
    let messages: Vec<ChatCompletionRequestMessage> = prompt.clone().into_iter().map(|m| m.into()).collect();
    ...
    let token_metadata = TokenMetadata {
        prompt: response
            ...,
        response: response
            ...,
    };
    ...
    let llm_call = LlmCall {
        ...
        request: Request {
            ...
            prompt: ChatPrompt { prompt },
        },
        ...
        tokens: token_metadata,
    };
    ...
}

#[tauri::command(async)]
#[specta]
pub async fn chat(
    ...,
    prompt: Vec<ChatMessage>,
) -> ZammResult<LlmCall> {
    ...
}

#[cfg(test)]
mod tests {
    ...

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ChatRequest {
        ...,
        prompt: Vec<ChatMessage>,
    }

    ...
}
```

##### Fixing the build

While `cargo test` passes, `cargo build` does not. We get errors such as

```
error[E0277]: the trait bound `NaiveDateTime: Deserialize<'_>` is not satisfied
    --> src/models/llm_calls.rs:315:20
     |
315  |     pub timestamp: NaiveDateTime,
     |                    ^^^^^^^^^^^^^ the trait `Deserialize<'_>` is not implemented for `NaiveDateTime`
     |
     = help: the following other types implement trait `Deserialize<'de>`:
               <&'a [u8] as Deserialize<'de>>
               <&'a serde_json::value::RawValue as Deserialize<'de>>
               <&'a std::path::Path as Deserialize<'de>>
               <&'a str as Deserialize<'de>>
               <() as Deserialize<'de>>
               <(T0, T1) as Deserialize<'de>>
               <(T0, T1, T2) as Deserialize<'de>>
               <(T0, T1, T2, T3) as Deserialize<'de>>
             and 444 others
note: required by a bound in `next_value`
    --> /root/.asdf/installs/rust/1.71.1/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.195/src/de/mod.rs:1865:12
     |
1863 |     fn next_value<V>(&mut self) -> Result<V, Self::Error>
     |        ---------- required by a bound in this associated function
1864 |     where
1865 |         V: Deserialize<'de>,
     |            ^^^^^^^^^^^^^^^^ required by this bound in `MapAccess::next_value`

error[E0277]: the trait bound `NaiveDateTime: Deserialize<'_>` is not satisfied
   --> src/models/llm_calls.rs:315:5
    |
315 |     pub timestamp: NaiveDateTime,
    |     ^^^ the trait `Deserialize<'_>` is not implemented for `NaiveDateTime`
    |
    = help: the following other types implement trait `Deserialize<'de>`:
              <&'a [u8] as Deserialize<'de>>
              <&'a serde_json::value::RawValue as Deserialize<'de>>
              <&'a std::path::Path as Deserialize<'de>>
              <&'a str as Deserialize<'de>>
              <() as Deserialize<'de>>
              <(T0, T1) as Deserialize<'de>>
              <(T0, T1, T2) as Deserialize<'de>>
              <(T0, T1, T2, T3) as Deserialize<'de>>
            and 444 others
note: required by a bound in `python_api::_::_serde::__private::de::missing_field`
   --> /root/.asdf/installs/rust/1.71.1/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.195/src/private/de.rs:25:8
    |
23  | pub fn missing_field<'de, V, E>(field: &'static str) -> Result<V, E>
    |        ------------- required by a bound in this function
24  | where
25  |     V: Deserialize<'de>,
    |        ^^^^^^^^^^^^^^^^ required by this bound in `missing_field`

For more information about this error, try `rustc --explain E0277`.
```

As we see from [this answer](https://stackoverflow.com/a/65379272), we need to enable the `serde` feature in the `chrono` crate. For some reason this doesn't fail on test builds. We edit `src-tauri/Cargo.toml` accordingly:

```toml
...
chrono = { ..., features = ["serde"] }

...
```

and now our build completes successfully.

### Frontend API call

Now that we have the backend API call working, we move over to the frontend side of the API boundary. We have Specta automatically regenerate `src-svelte/src/lib/bindings.ts` as usual. We then create a new page at `src-svelte/src/routes/chat/+page.svelte`:

```svelte
<script lang="ts">
  import Chat from "./Chat.svelte";
</script>

<Chat />

```

and define a skeleton version of the `Chat` component at `src-svelte/src/routes/chat/Chat.svelte`, just enough for us to trigger the API call:

```svelte
<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import TextInput from "$lib/controls/TextInput.svelte";
  import Button from "$lib/controls/Button.svelte";
  import { type ChatMessage, chat } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";

  let currentMessage: string;
  let conversation: ChatMessage[] = [
    {
      role: "System",
      text: "You are ZAMM, a chat program. Respond in first person.",
    },
  ];

  async function sendChat() {
    const message = currentMessage.trim();
    if (message) {
      const chatMessage: ChatMessage = {
        role: "Human",
        text: message,
      };
      conversation.push(chatMessage);
      currentMessage = "";

      try {
        let llmCall = await chat("OpenAI", "gpt-3.5-turbo", null, conversation);
        conversation.push(llmCall.response.completion);
      } catch (err) {
        snackbarError(err as string);
      }
    }
  }
</script>

<InfoBox title="Chat">
  <form on:submit|preventDefault={sendChat}>
    <label for="message" class="accessibility-only">Chat with the AI:</label>
    <TextInput name="message" placeholder="Type your message here..." bind:value={currentMessage} />
    <Button text="Send" />
  </form>
</InfoBox>

<style>
  form {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }
</style>
```

We define a new Storybook story at `src-svelte/src/routes/chat/Chat.stories.ts`:

```ts
import Chatcomponent from "./Chat.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: Chatcomponent,
  title: "Screens/Chat/Conversation",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Chatcomponent,
  props: args,
});

export const Tablet: StoryObj = Template.bind({}) as any;
Tablet.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

```

and then define a new test at `src-svelte/src/routes/chat/Chat.test.ts`:

```ts
import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen } from "@testing-library/svelte";
import Chat from "./Chat.svelte";
import userEvent from "@testing-library/user-event";
import { TauriInvokePlayback } from "$lib/sample-call-testing";
import { animationSpeed } from "$lib/preferences";

describe("Chat conversation", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeAll(() => {
    animationSpeed.set(10);
  });

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );

    vi.stubGlobal("requestAnimationFrame", (fn: FrameRequestCallback) => {
      return window.setTimeout(() => fn(Date.now()), 16);
    });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  async function sendChatMessage(message: string, correspondingApiCallSample: string) {
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples(correspondingApiCallSample);

    const chatInput = screen.getByLabelText("Chat with the AI:");
    expect(chatInput).toHaveValue("");
    await userEvent.type(chatInput, message);
    await userEvent.click(screen.getByRole("button", { name: "Send" }));

    expect(tauriInvokeMock).toBeCalledTimes(1);
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    tauriInvokeMock.mockClear();
  }

  test("can start a conversation", async () => {
    render(Chat, {});
    await sendChatMessage("Hello, does this work?", "../src-tauri/api/sample-calls/chat-start-conversation.yaml");
  });
});

```

We have the line

```ts
expect(tauriInvokeMock).toHaveReturnedTimes(1);
```

because this test suspicously succeeds even when we don't call it with the right arguments, as we can see when we edit `src-svelte/src/lib/sample-call-testing.ts`:

```ts

export class TauriInvokePlayback {
  ...

  mockCall(
    ...
  ): Promise<Record<string, string>> {
    ...
      if (typeof process === "object") {
        console.error(errorMessage);
        throw new Error(errorMessage);
      } else {
        ...
      }
    ...
  }

  ...
}
```

For some the thrown error does not make the test fail in this case. We replace `toBeCalledTimes` with `toHaveReturnedTimes` in the rest of the tests, just in case the same error lurks there. We also replace `toHaveBeenLastCalledWith("get_system_info")` with `toHaveReturnedTimes` in `src-svelte/src/routes/components/Metadata.test.ts`, but not in:

- `src-svelte/src/lib/Switch.test.ts` because `recordSoundDelaySpy` is not an API call
- `src-svelte/src/routes/components/api-keys/Display.test.ts` because the `API key error` test is mocking a hypothetical error because we haven't triggered an actual error call

In `Display.test.ts`, we realize that we haven't tested the return method, and so we instead do a bit of refactoring by introducing `getOpenAiStatus` and then using that to check the result of the API call with the invalid filename:

```ts
describe("API Keys Display", () => {
  ...

  beforeEach(() => {
    ...
    clearAllMessages();
  });

  function getOpenAiStatus() {
    const openAiRow = screen.getByRole("row", { name: /OpenAI/ });
    const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
    return openAiKeyCell.textContent;
  }

  async function checkSampleCall(filename: string, expected_display: string) {
    ...
    expect(getOpenAiStatus()).toBe(expected_display);
  }

  ...

  test("can submit with invalid file", async () => {
    systemInfo.set({
      ...NullSystemInfo,
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_api_key-invalid-filename.yaml",
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
    );

    await toggleOpenAIForm();
    await userEvent.type(screen.getByLabelText("Export from:"), "/");
    await userEvent.type(screen.getByLabelText("API key:"), "0p3n41-4p1-k3y");
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    await waitFor(() => expect(tauriInvokeMock).toHaveBeenCalledTimes(2));
    expect(getOpenAiStatus()).toBe("Active");
    expect(tauriInvokeMock).toHaveReturnedTimes(1);

    render(Snackbar, {});
    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(1);
    expect(alerts[0]).toHaveTextContent("Is a directory (os error 21)");
  });
});
```

All our tests pass after all, so it appears the same error is not lurking around secretly. We continue by creating a second API request that continues the conversation:

```yaml
request:
  - chat
  - >
    {
      "provider": "OpenAI",
      "llm": "gpt-3.5-turbo",
      "temperature": null,
      "prompt": [
        {
          "role": "System",
          "text": "You are ZAMM, a chat program. Respond in first person."
        },
        {
          "role": "Human",
          "text": "Hello, does this work?"
        },
        {
          "role": "AI",
          "text": "Hello! I'm ZAMM, a chat program. I'm here to assist you. What can I help you with today?"
        },
        {
          "role": "Human",
          "text": "Tell me something funny."
        }
      ]
    }
response:
  message: >
    {
      "id": "d5ad1e49-f57f-4481-84fb-4d70ba8a7a74",
      "timestamp": "2024-01-16T08:50:19.738093890",
      "llm": {
        "name": "gpt-3.5-turbo-0613",
        "requested": "gpt-3.5-turbo",
        "provider": "OpenAI"
      },
      "request": {
        "prompt": [
          {
            "role": "System",
            "text": "You are ZAMM, a chat program. Respond in first person."
          },
          {
            "role": "Human",
            "text": "Hello, does this work?"
          },
          {
            "role": "AI",
            "text": "Hello! I'm ZAMM, a chat program. I'm here to assist you. What can I help you with today?"
          },
          {
            "role": "Human",
            "text": "Tell me something funny."
          }
        ],
        "temperature": 1.0
      },
      "response": {
        "completion": {
          "role": "AI",
          "text": "Sure, here's a light-hearted joke for you:\n\nWhy don't scientists trust atoms?\n\nBecause they make up everything!"
        }
      },
      "tokens": {
        "prompt": 72,
        "response": 24
      }
    }

```

We refactor the existing test at `src-tauri/src/commands/llms/chat.rs` to generalize to other API calls and network requests, and apply the new function to our new API call sample:

```rs
...

#[cfg(test)]
mod tests {
    ...

    async fn test_llm_api_call(recording_path: &str, sample_path: &str) {
        let recording_path = PathBuf::from(recording_path);
        ...
        let sample = read_sample(sample_path);
        ...
    }

    #[tokio::test]
    async fn test_start_conversation() {
        test_llm_api_call(
            "api/sample-call-requests/start-conversation.json",
            "api/sample-calls/chat-start-conversation.yaml",
        ).await;        
    }

    #[tokio::test]
    async fn test_continue_conversation() {
        test_llm_api_call(
            "api/sample-call-requests/continue-conversation.json",
            "api/sample-calls/chat-continue-conversation.yaml",
        ).await;        
    }
}

```

The file at `src-tauri/api/sample-call-requests/continue-conversation.json` gets automatically created with the first run of the test, and we update the `yaml` file based on that.

We can now modify `src-svelte/src/routes/chat/Chat.test.ts` to continue the conversation:

```ts
  test("can start and continue a conversation", async () => {
    ...
    await sendChatMessage(
      "Tell me something funny.",
      "../src-tauri/api/sample-calls/chat-continue-conversation.yaml",
    );
  });
```

This works, but note that nothing is displayed onscreen. We edit `src-svelte/src/routes/chat/Chat.svelte` to show the chat message bubbles as well:

```svelte
<script lang="ts">
  ...
  export let conversation: ChatMessage[] = [
    ...
  ];

  async function sendChat() {
    const message = currentMessage.trim();
    if (message) {
      ...
      conversation = [...conversation, chatMessage];
      ...

      try {
        ...
        conversation = [...conversation, llmCall.response.completion];
      } ...
    }
  }
</script>

<InfoBox title="Chat">
  <div class="conversation" role="list">
    {#if conversation.length > 1}
      {#each conversation.slice(1) as message}
        <div class:message class={message.role.toLowerCase()} role="listitem">
          <div class="arrow"></div>
          <div class="text">
            {message.text}
          </div>
        </div>
      {/each}
    {/if}
  </div>

  ...
</InfoBox>

<style>
  .conversation {
    margin-bottom: 1rem;
  }

  .message {
    --message-color: gray;
    --arrow-size: 0.5rem;
    position: relative;
  }

  .message .text {
    margin: 0.5rem var(--arrow-size);
    border-radius: var(--corner-roundness);
    width: fit-content;
    max-width: 60%;
    padding: 0.75rem;
    box-sizing: border-box;
    background-color: var(--message-color);
    white-space: pre-line;
  }

  .message .arrow {
    position: absolute;
    width: 0;
    height: 0;
    bottom: var(--arrow-size);
    border: var(--arrow-size) solid transparent;
  }

  .message.human {
    --message-color: #E5FFE5;
  }

  .message.human .text {
    margin-left: auto;
  }

  .message.human .arrow {
    float: right;
    right: calc(-1 * var(--arrow-size));
    border-left-color: var(--message-color); 
  }

  .message.ai {
    --message-color: #E5E5FF;
  }

  .message.ai .text {
    margin-right: auto;
  }

  .message.ai .arrow {
    float: left;
    left: calc(-1 * var(--arrow-size));
    border-right-color: var(--message-color); 
  }

  ...
</style>

```

Note that we need to change the messages update code from `conversation.push` to `conversation = [...]` because otherwise, Svelte doesn't realize it needs to refresh the DOM.

Also note that we need to set `white-space: pre-line` for the newlines. We make sure to demonstate:

- a long message that requires word wrapping
- a short message that doesn't require word wrapping but should be flush with the side it's next to
- a message with multiple lines.

We edit `src-svelte/src/routes/chat/Chat.stories.ts` to make use of the newly exported `conversation` prop:

```ts
...
import type { ChatMessage } from "$lib/bindings";

...


export const Empty: StoryObj = Template.bind({}) as any;
Empty.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const NotEmpty: StoryObj = Template.bind({}) as any;
const conversation: ChatMessage[] = [
  {
    role: "System",
    text: "You are ZAMM, a chat program. Respond in first person.",
  },
  {
    role: "Human",
    text: "Hello, does this work?",
  },
  {
    role: "AI",
    text: "Hello! I'm ZAMM, a chat program. I'm here to assist you. What can I help you with today?",
  },
  {
    role: "Human",
    text: "Tell me something really funny, like really funny. Make me laugh hard.",
  },
  {
    role: "AI",
    text: "Sure, here's a light-hearted joke for you:\n\nWhy don't scientists trust atoms?\n\nBecause they make up everything!",
  },
];
NotEmpty.args = {
  conversation,
};
...

```

We test these changes in `src-svelte/src/routes/chat/Chat.test.ts`:

```ts
...

import { render, screen, waitFor } from "@testing-library/svelte";
...
import { TauriInvokePlayback, type ParsedCall } from "$lib/sample-call-testing";
...
import type { ChatMessage, LlmCall } from "$lib/bindings";

describe("Chat conversation", () => {
  ...

  async function sendChatMessage(
    ...
  ) {
    ...
    const nextExpectedApiCall: ParsedCall = playback.unmatchedCalls.slice(-1)[0];
    const nextExpectedCallArgs = nextExpectedApiCall.request[1] as Record<string, any>;
    const nextExpectedMessage = nextExpectedCallArgs["prompt"].slice(-1)[0] as ChatMessage;
    const nextExpectedHumanPrompt = nextExpectedMessage.text;

    ...
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(screen.getByText(nextExpectedHumanPrompt)).toBeInTheDocument();

    ...
    const lastResult: LlmCall = tauriInvokeMock.mock.results[0].value;
    const aiResponse = lastResult.response.completion.text;
    const lastSentence = aiResponse.split("\n").slice(-1)[0];
    await waitFor(() => {
      expect(screen.getByText(new RegExp(lastSentence))).toBeInTheDocument();
    });

    tauriInvokeMock.mockClear();
  }

  ...
});
```

`eslint` complains about the line length in `src-svelte/src/routes/chat/Chat.stories.ts`, so instead we do:

```ts
const conversation: ChatMessage[] = [
  ...
  {
    role: "AI",
    text:
      "Hello! I'm ZAMM, a chat program. I'm here to assist you. " +
      "What can I help you with today?",
  },
  ...
  {
    role: "AI",
    text:
      "Sure, here's a light-hearted joke for you:\n\n" +
      "Why don't scientists trust atoms?\n\n" +
      "Because they make up everything!",
  },
];
```
