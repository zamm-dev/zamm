CREATE TABLE llm_call_variants (
  canonical_id VARCHAR NOT NULL,
  variant_id VARCHAR NOT NULL,
  PRIMARY KEY (canonical_id, variant_id),
  FOREIGN KEY (canonical_id) REFERENCES llm_calls (id) ON DELETE CASCADE,
  FOREIGN KEY (variant_id) REFERENCES llm_calls (id) ON DELETE CASCADE
);

CREATE VIEW llm_call_named_variants AS
  SELECT
    llm_call_variants.canonical_id AS canonical_id,
    canonical.completion AS canonical_completion,
    llm_call_variants.variant_id AS variant_id,
    variant.completion AS variant_completion
  FROM
    llm_call_variants
    JOIN llm_calls AS canonical ON llm_call_variants.canonical_id = canonical.id
    JOIN llm_calls AS variant ON llm_call_variants.variant_id = variant.id
  ORDER BY variant.timestamp ASC;
