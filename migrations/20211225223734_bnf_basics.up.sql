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
  content varchar(64) unique not null,
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

INSERT INTO grammar_tag (name) VALUES
('Il'),
('Lo'),
('La'),
('I'),
('Gli'),
('Le'),
('_'),
('Un'),
('Un'''),
('Uno'),
('Una')
;

INSERT INTO non_terminal_symbol (name) VALUES
('Start')
;

INSERT INTO production (non_terminal_symbol, production) VALUES
(1, '<0:N:F:N:F:Santo> <1:O(0):T:O(0):T:Malaparola>')
;

INSERT INTO semantic_tag (name, sticky) VALUES
('Malaparola', 't'),
('Santo', 'f'),
('Divino', 'f'),
('Animale', 't'),
('Malattia', 't')
;

INSERT INTO word (content, non_repeatable) VALUES
('Dio', 't'),
('Madonna', 't'),
('Cristo', 't'),
('San Giuseppe', 't'),
('Gesù', 't'),
('Maria', 't'),
('Papa', 't'),
('Cane', 't'),
('Porco', 't'),
('Cagna', 't'),
('Tubercoloso', 't'),
('Tubercolosa', 't'),
('Porca', 't'),
('Suino', 't'),
('Suina', 't'),
('Assassino', 't'),
('Assassina', 't'),
('Megattera', 't')
;

INSERT INTO word_grammar (word, grammar_tag) VALUES
(1, 1),
(1, 8),
(2, 3),
(2, 11),
(3, 1),
(3, 8),
(4, 1),
(4, 8),
(5, 1),
(5, 8),
(6, 3),
(6, 11),
(7, 1),
(7, 8),
(8, 1),
(8, 8),
(9, 1),
(9, 8),
(10, 3),
(10, 11),
(11, 1),
(11, 8),
(12, 3),
(12, 11),
(13, 3),
(13, 11),
(14, 1),
(14, 8),
(15, 3),
(15, 11),
(16, 1),
(16, 8),
(17, 3),
(17, 11),
(18, 3),
(18, 11)
;

INSERT INTO word_semantic (word, semantic_tag) VALUES
(1, 2),
(1, 3),
(2, 2),
(2, 3),
(3, 2),
(3, 3),
(5, 2),
(5, 3),
(4, 2),
(6, 2),
(7, 2),
(8, 1),
(8, 4),
(9, 1),
(9, 4),
(10, 1),
(10, 4),
(13, 1),
(13, 4),
(14, 1),
(14, 4),
(15, 1),
(15, 4),
(16, 1),
(17, 1),
(11, 1),
(11, 5),
(12, 1),
(12, 5),
(18, 1),
(18, 4)
;
