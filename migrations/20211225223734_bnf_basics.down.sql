-- Add down migration script here
DROP TABLE
  word_grammar,
  word_semantic,
  semantic_tag,
  grammar_tag,
  word,
  production,
  non_terminal_symbol;
