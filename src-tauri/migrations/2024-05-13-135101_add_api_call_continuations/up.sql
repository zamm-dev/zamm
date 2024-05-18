CREATE TABLE llm_call_continuations (
  previous_call_id VARCHAR NOT NULL,
  next_call_id VARCHAR NOT NULL,
  PRIMARY KEY (previous_call_id, next_call_id),
  FOREIGN KEY (previous_call_id) REFERENCES llm_calls (id) ON DELETE CASCADE,
  FOREIGN KEY (next_call_id) REFERENCES llm_calls (id) ON DELETE CASCADE
);

CREATE VIEW llm_call_named_continuations AS
  SELECT
    llm_call_continuations.previous_call_id AS previous_call_id,
    previous_call.completion AS previous_call_completion,
    llm_call_continuations.next_call_id AS next_call_id,
    next_call.completion AS next_call_completion
  FROM
    llm_call_continuations
    JOIN llm_calls AS previous_call ON llm_call_continuations.previous_call_id = previous_call.id
    JOIN llm_calls AS next_call ON llm_call_continuations.next_call_id = next_call.id;
