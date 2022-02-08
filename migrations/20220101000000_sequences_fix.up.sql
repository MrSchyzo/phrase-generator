SELECT pg_catalog.setval(pg_get_serial_sequence('word', 'id'), MAX(id)) FROM word;
SELECT pg_catalog.setval(pg_get_serial_sequence('semantic_tag', 'id'), MAX(id)) FROM semantic_tag;
SELECT pg_catalog.setval(pg_get_serial_sequence('grammar_tag', 'id'), MAX(id)) FROM grammar_tag;
SELECT pg_catalog.setval(pg_get_serial_sequence('non_terminal_symbol', 'id'), MAX(id)) FROM non_terminal_symbol;
SELECT pg_catalog.setval(pg_get_serial_sequence('production', 'id'), MAX(id)) FROM production;
SELECT pg_catalog.setval(pg_get_serial_sequence('word_grammar', 'id'), MAX(id)) FROM word_grammar;
SELECT pg_catalog.setval(pg_get_serial_sequence('word_semantic', 'id'), MAX(id)) FROM word_semantic;
