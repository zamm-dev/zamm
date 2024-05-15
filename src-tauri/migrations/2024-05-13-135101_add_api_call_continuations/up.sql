CREATE TABLE llm_call_continuations (
  previous_call_id VARCHAR NOT NULL,
  next_call_id VARCHAR NOT NULL,
  PRIMARY KEY (previous_call_id, next_call_id),
  FOREIGN KEY (previous_call_id) REFERENCES llm_calls (id) ON DELETE CASCADE,
  FOREIGN KEY (next_call_id) REFERENCES llm_calls (id) ON DELETE CASCADE
)
