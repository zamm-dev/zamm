#!/usr/bin/env python3
#
# meant to be run from the src-tauri/ folder

from typing import Generator

yaml_preamble = """api_keys: []
llm_calls:
"""


def json_preamble(offset: int) -> str:
    return """request:
  - get_api_calls
  - >
    {
      "offset": {0}
    }
response:
  message: >
    [
""".replace(
        "{0}", str(offset)
    )


json_postamble = """
    ]
sideEffects:
  database:
    startStateDump: many-api-calls
    endStateDump: many-api-calls
"""


def generate_api_call_yaml(i: int) -> str:
    return f"""- id: d5ad1e49-f57f-4481-84fb-4d70ba8a7a{i:02d}
  timestamp: 2024-01-16T08:{i:02d}:50.738093890
  llm:
    name: gpt-4-0613
    requested: gpt-4
    provider: OpenAI
  request:
    prompt:
      type: Chat
      messages:
      - role: System
        text: You are ZAMM, a chat program. Respond in first person.
      - role: Human
        text: This is a mock conversation.
    temperature: 1.0
  response:
    completion:
      role: AI
      text: Mocking number {i}.
  tokens:
    prompt: 15
    response: 3
    total: 18
"""


def generate_api_call_sql(i: int) -> str:
    return """INSERT INTO llm_calls VALUES('d5ad1e49-f57f-4481-84fb-4d70ba8a7a{0:02d}','2024-01-16 08:{0:02d}:50.738093890','open_ai','gpt-4','gpt-4-0613',1.0,15,3,18,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"This is a mock conversation."}]}','{"role":"AI","text":"Mocking number {0}."}');""".replace(
        "{0:02d}", str(i).zfill(2)
    ).replace(
        "{0}", str(i)
    )


def generate_api_call_json(i: int) -> str:
    return """      {
        "id": "d5ad1e49-f57f-4481-84fb-4d70ba8a7a{0:02d}",
        "timestamp": "2024-01-16T08:{0:02d}:50.738093890",
        "llm": {
          "name": "gpt-4-0613",
          "requested": "gpt-4",
          "provider": "OpenAI"
        },
        "request": {
          "prompt": {
            "type": "Chat",
            "messages": [
              {
                "role": "System",
                "text": "You are ZAMM, a chat program. Respond in first person."
              },
              {
                "role": "Human",
                "text": "This is a mock conversation."
              }
            ]
          },
          "temperature": 1.0
        },
        "response": {
          "completion": {
            "role": "AI",
            "text": "Mocking number {0}."
          }
        },
        "tokens": {
          "prompt": 15,
          "response": 3,
          "total": 18
        }
      }""".replace(
        "{0:02d}", str(i).zfill(2)
    ).replace(
        "{0}", str(i)
    )


def db_index_sequence(n: int) -> Generator[int, None, None]:
    for i in range(0, n, 2):
        yield i
    for i in range(1, n, 2):
        yield i


def generate_yaml(n: int) -> None:
    with open("api/sample-database-writes/many-api-calls/dump.yaml", "wb") as f:
        f.write(yaml_preamble.encode())
        for i in db_index_sequence(n):
            f.write(generate_api_call_yaml(i).encode())


def generate_sql(n: int) -> None:
    with open("api/sample-database-writes/many-api-calls/dump.sql", "wb") as f:
        api_calls = []
        for i in db_index_sequence(n):
            api_calls.append(generate_api_call_sql(i))
        f.write("\n".join(api_calls).encode())


def generate_json(test_name: str, offset: int, start: int, end: int) -> None:
    with open(f"api/sample-calls/get_api_calls-{test_name}.yaml", "wb") as f:
        f.write(json_preamble(offset).encode())
        api_calls = []
        for i in range(start, end, -1):
            api_calls.append(generate_api_call_json(i))
        api_calls_json = ",\n".join(api_calls)
        f.write(api_calls_json.encode())
        f.write(json_postamble.encode())


def generate_json_sample_files(n: int, page_size: int) -> None:
    generate_json(
        "full",
        offset=0,
        start=n - 1,
        end=n - page_size - 1,
    )
    generate_json(
        "offset",
        offset=page_size,
        start=n - page_size - 1,
        end=-1,
    )


if __name__ == "__main__":
    n = 60
    generate_yaml(n)
    generate_sql(n)
    generate_json_sample_files(n, 50)
