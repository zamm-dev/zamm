INSERT INTO llm_calls VALUES('d5ad1e49-f57f-4481-84fb-4d70ba8a7a74','2024-01-16 08:50:19.738093890','open_ai','gpt-4','gpt-4-0613',1.0,32,12,44,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hello, does this work?"}]}','{"role":"AI","text":"Yes, it works. How can I assist you today?"}');
INSERT INTO llm_calls VALUES('c13c1e67-2de3-48de-a34c-a32079c03316','2024-01-16 09:50:19.738093890','open_ai','gpt-4','gpt-4-0613',1.0,57,22,79,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hello, does this work?"},{"role":"AI","text":"Yes, it works. How can I assist you today?"},{"role":"Human","text":"Tell me something funny."}]}','{"role":"AI","text":"Sure, here''s a joke for you: Why don''t scientists trust atoms? Because they make up everything!"}');
INSERT INTO llm_calls VALUES('037b28dd-6f24-4e68-9dfb-3caa1889d886','2024-07-29 17:30:11.073212','Unknown Future Provider','unknown-future-llm','unknown-future-llm',1.0,47,14,61,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hi"},{"role":"AI","text":"Hello! How can I assist you today?"},{"role":"Human","text":"Fuck you!"}]}','{"role":"AI","text":"I''m sorry to hear that. How can I assist you better?"}');
INSERT INTO llm_call_follow_ups VALUES('d5ad1e49-f57f-4481-84fb-4d70ba8a7a74','c13c1e67-2de3-48de-a34c-a32079c03316');