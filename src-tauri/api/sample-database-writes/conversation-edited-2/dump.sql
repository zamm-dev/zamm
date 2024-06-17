INSERT INTO llm_calls VALUES('d5ad1e49-f57f-4481-84fb-4d70ba8a7a74','2024-01-16 08:50:19.738093890','open_ai','gpt-4','gpt-4-0613',1.0,32,12,44,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hello, does this work?"}]}','{"role":"AI","text":"Yes, it works. How can I assist you today?"}');
INSERT INTO llm_calls VALUES('c13c1e67-2de3-48de-a34c-a32079c03316','2024-01-16 09:50:19.738093890','open_ai','gpt-4','gpt-4-0613',1.0,57,22,79,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hello, does this work?"},{"role":"AI","text":"Yes, it works. How can I assist you today?"},{"role":"Human","text":"Tell me something funny."}]}','{"role":"AI","text":"Sure, here''s a joke for you: Why don''t scientists trust atoms? Because they make up everything!"}');
INSERT INTO llm_calls VALUES('0e6bcadf-2b41-43d9-b4cf-81008d4f4771','2024-05-23 09:30:37.854241700','open_ai','gpt-4','gpt-4-0613',1.0,98,46,144,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hello, does this work?"},{"role":"AI","text":"Yes, it works. How can I assist you today?"},{"role":"Human","text":"Tell me something funny."},{"role":"AI","text":"Sure, here''s a joke for you: Why don''t scientists trust atoms? Because they make up everything!"},{"role":"Human","text":"Write me a Python script that prints that joke out."}]}','{"role":"AI","text":"Sure, here is a simple Python script that will print out the joke:\n\n```python\nprint(\"Why don''t scientists trust atoms? Because they make up everything!\")\n```\n\nJust run this script and it will display the joke."}');
INSERT INTO llm_calls VALUES('63b5c02e-b864-4efe-a286-fbef48b152ef','2024-05-23 09:34:38.572764500','open_ai','gpt-4','gpt-4-0613',1.0,98,58,156,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hello, does this work?"},{"role":"AI","text":"Yes, it works. How can I assist you today?"},{"role":"Human","text":"Tell me something funny."},{"role":"AI","text":"Sure, here''s a joke for you: Why don''t scientists trust atoms? Because they make up everything!"},{"role":"Human","text":"Write me a Rust script that prints that joke out."}]}','{"role":"AI","text":"Sure, here is a simple Rust program that prints out the joke:\n\n```rust\nfn main() {\n    println!(\"Why don''t scientists trust atoms? Because they make up everything!\");\n}\n```\nTo run this program, you''d simply compile and run the Rust file containing this code."}');
INSERT INTO llm_calls VALUES('f39a5017-89d4-45ec-bcbb-25c2bd43cfc1','2024-06-08 06:20:40.601356700','open_ai','gpt-4','gpt-4-0613',1.0,58,25,83,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hello, does this work?"},{"role":"AI","text":"Yes, it works. How can I assist you today?"},{"role":"Human","text":"Tell me a funny joke."}]}','{"role":"AI","text":"Sure, here is a light-hearted joke for you: \n\nWhy don''t scientists trust atoms?\n\nBecause they make up everything!"}');
INSERT INTO llm_calls VALUES('7a35a4cf-f3d9-4388-bca8-2fe6e78c9648','2024-06-08 09:40:22.392223700','open_ai','gpt-4','gpt-4-0613',1.0,59,19,78,'{"type":"Chat","messages":[{"role":"System","text":"You are ZAMM, a chat program. Respond in first person."},{"role":"Human","text":"Hello, does this really work?"},{"role":"AI","text":"Yes, it works. How can I assist you today?"},{"role":"Human","text":"Tell me a funny joke."}]}','{"role":"AI","text":"Sure, here you go: Why don''t scientists trust atoms? Because they make up everything!"}');
INSERT INTO llm_call_follow_ups VALUES('d5ad1e49-f57f-4481-84fb-4d70ba8a7a74','c13c1e67-2de3-48de-a34c-a32079c03316');
INSERT INTO llm_call_follow_ups VALUES('c13c1e67-2de3-48de-a34c-a32079c03316','0e6bcadf-2b41-43d9-b4cf-81008d4f4771');
INSERT INTO llm_call_follow_ups VALUES('c13c1e67-2de3-48de-a34c-a32079c03316','63b5c02e-b864-4efe-a286-fbef48b152ef');
INSERT INTO llm_call_variants VALUES('c13c1e67-2de3-48de-a34c-a32079c03316','f39a5017-89d4-45ec-bcbb-25c2bd43cfc1');
INSERT INTO llm_call_variants VALUES('c13c1e67-2de3-48de-a34c-a32079c03316','7a35a4cf-f3d9-4388-bca8-2fe6e78c9648');