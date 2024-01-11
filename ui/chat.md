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
