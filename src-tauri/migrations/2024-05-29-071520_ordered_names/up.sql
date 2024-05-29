DROP VIEW llm_call_named_follow_ups;
CREATE VIEW llm_call_named_follow_ups AS
  SELECT
    llm_call_follow_ups.previous_call_id AS previous_call_id,
    previous_call.completion AS previous_call_completion,
    llm_call_follow_ups.next_call_id AS next_call_id,
    next_call.completion AS next_call_completion
  FROM
    llm_call_follow_ups
    JOIN llm_calls AS previous_call ON llm_call_follow_ups.previous_call_id = previous_call.id
    JOIN llm_calls AS next_call ON llm_call_follow_ups.next_call_id = next_call.id
  ORDER BY next_call.timestamp ASC;
