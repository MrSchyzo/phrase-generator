-- Add up migration script here
CREATE TABLE generated_phrase (
  id uuid primary key not null default gen_random_uuid(),
  content text not null
);
CREATE UNIQUE INDEX idx_generated_phrase_uniqueness ON generated_phrase (content);
CREATE INDEX idx_generated_phrase_lookup ON generated_phrase (content);

create type lang AS ENUM ('ita');
create type gender AS ENUM ('male', 'female');

CREATE TABLE generated_phrase_speech (
  id serial primary key not null,
  generated_phrase uuid not null,
  lang lang not null,
  gender gender not null,
  url text not null,
  foreign key (generated_phrase) references generated_phrase (id)
);
CREATE UNIQUE INDEX idx_generated_phrase_speech_uniqueness ON generated_phrase_speech (generated_phrase, lang, gender);
CREATE INDEX idx_generated_phrase_speech_lookup ON generated_phrase_speech (generated_phrase, lang, gender);
