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

create or replace function array_contains_or_intersects(whole int[], contained_or_intersected int[]) returns boolean as
$$
	select (whole @> contained_or_intersected or whole && contained_or_intersected);
$$
language sql;

create or replace function array_contains_and_intersects(whole int[], contained int[], intersects_with int[]) returns boolean as
$$
	select (whole @> contained and array_contains_or_intersects(whole, intersects_with));
$$ 
language sql;

INSERT INTO grammar_tag (id, name) VALUES
(1, 'Il'),
(2, 'Lo'),
(3, 'La'),
(4, 'I'),
(5, 'Gli'),
(6, 'Le'),
(7, '_'),
(8, 'Un'),
(9, 'Un'''),
(10, 'Uno'),
(11, 'Una')
;

INSERT INTO non_terminal_symbol (id, name) VALUES
(1, 'Start')
;

INSERT INTO production (id, non_terminal_symbol, production) VALUES
(3, 1, '<0:N:F:N:F:Santo> <1:O(0):T:O(0):T:Malaparola>')
;

INSERT INTO semantic_tag (id, name, sticky) VALUES
(1, 'Malaparola', 't'),
(2, 'Santo', 'f'),
(3, 'Divino', 'f'),
(4, 'Animale', 't'),
(5, 'Malattia', 't')
;

INSERT INTO word (id, content, non_repeatable) VALUES
(1, 'Dio', 't'),
(2, 'Madonna', 't'),
(3, 'Cristo', 't'),
(4, 'San Giuseppe', 't'),
(5, 'Gesù', 't'),
(6, 'Maria', 't'),
(7, 'Papa', 't'),
(8, 'Cane', 't'),
(9, 'Porco', 't'),
(10, 'Cagna', 't'),
(11, 'Tubercoloso', 't'),
(12, 'Tubercolosa', 't'),
(13, 'Porca', 't'),
(14, 'Suino', 't'),
(15, 'Suina', 't'),
(16, 'Assassino', 't'),
(17, 'Assassina', 't'),
(18, 'Megattera', 't')
;

INSERT INTO word_grammar (id, word, grammar_tag) VALUES
(1, 1, 1),
(2, 1, 8),
(3, 2, 3),
(4, 2, 11),
(5, 3, 1),
(6, 3, 8),
(7, 4, 1),
(8, 4, 8),
(9, 5, 1),
(10, 5, 8),
(11, 6, 3),
(12, 6, 11),
(13, 7, 1),
(14, 7, 8),
(15, 8, 1),
(16, 8, 8),
(17, 9, 1),
(18, 9, 8),
(19, 10, 3),
(20, 10, 11),
(21, 11, 1),
(22, 11, 8),
(23, 12, 3),
(24, 12, 11),
(25, 13, 3),
(26, 13, 11),
(27, 14, 1),
(28, 14, 8),
(29, 15, 3),
(30, 15, 11),
(31, 16, 1),
(32, 16, 8),
(33, 17, 3),
(34, 17, 11),
(35, 18, 3),
(36, 18, 11)
;

INSERT INTO word_semantic (id, word, semantic_tag) VALUES
(1, 1, 2),
(2, 1, 3),
(3, 2, 2),
(4, 2, 3),
(5, 3, 2),
(6, 3, 3),
(7, 5, 2),
(8, 5, 3),
(9, 4, 2),
(10, 6, 2),
(11, 7, 2),
(12, 8, 1),
(13, 8, 4),
(14, 9, 1),
(15, 9, 4),
(16, 10, 1),
(17, 10, 4),
(18, 13, 1),
(19, 13, 4),
(20, 14, 1),
(21, 14, 4),
(22, 15, 1),
(23, 15, 4),
(24, 16, 1),
(25, 17, 1),
(26, 11, 1),
(27, 11, 5),
(28, 12, 1),
(29, 12, 5),
(37, 18, 1),
(38, 18, 4)
;
