CREATE TABLE non_terminal_symbol (
  id serial primary key not null,
  name varchar(8) unique not null
);

CREATE TABLE production (
  id serial primary key not null,
  non_terminal_symbol int not null,
  production varchar(1024) not null,
  nts_amount int not null generated always as ((character_length(production) - character_length(replace(production, '{', '')))) stored,
  foreign key (non_terminal_symbol) references non_terminal_symbol (id)
);
CREATE INDEX idx_production_non_terminal_symbol ON production (non_terminal_symbol);
CREATE INDEX idx_production_nts_amount ON production (nts_amount);

CREATE TABLE word (
  id serial primary key not null,
  content varchar(64) not null,
  non_repeatable boolean not null default true
);
CREATE INDEX idx_word_content ON word (content varchar_pattern_ops);

CREATE TABLE grammar_tag (
  id serial primary key not null,
  name varchar(32) unique not null
);

CREATE TABLE semantic_tag (
  id serial primary key not null,
  name varchar(32) unique not null,
  sticky boolean not null default TRUE
);

CREATE TABLE word_semantic (
  id serial primary key not null,
  word int not null,
  semantic_tag int not null,
  foreign key (word) references word (id),
  foreign key (semantic_tag) references semantic_tag (id)
);
CREATE INDEX idx_word_semantic_word ON word_semantic (word);
CREATE INDEX idx_word_semantic_semantic_tag ON word_semantic (semantic_tag);
CREATE UNIQUE INDEX idx_word_semantic_uniqueness ON word_semantic (word, semantic_tag);

CREATE TABLE word_grammar (
  id serial primary key not null,
  word int not null,
  grammar_tag int not null,
  foreign key (word) references word (id),
  foreign key (grammar_tag) references grammar_tag (id)
);
CREATE INDEX idx_word_grammar_word ON word_grammar (word);
CREATE INDEX idx_word_grammar_grammar_tag ON word_grammar (grammar_tag);
CREATE UNIQUE INDEX idx_word_grammar_uniqueness ON word_grammar (word, grammar_tag);

create or replace function array_contains_and_intersects(whole int[], contained int[], intersects_with int[]) returns boolean as
$$
	select (whole @> contained and whole && intersects_with);
$$ 
language sql;
