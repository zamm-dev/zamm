CREATE TABLE llm_calls (
  id VARCHAR PRIMARY KEY NOT NULL,
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  provider VARCHAR NOT NULL,
  llm_requested VARCHAR NOT NULL,
  llm VARCHAR NOT NULL,
  temperature REAL NOT NULL,
  prompt_tokens INTEGER,
  response_tokens INTEGER,
  total_tokens INTEGER,
  prompt TEXT NOT NULL,
  completion TEXT NOT NULL
)
