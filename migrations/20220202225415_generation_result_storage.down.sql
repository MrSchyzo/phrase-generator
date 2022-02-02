-- Add down migration script here
DROP TABLE
  generated_phrase_speech,
  generated_phrase;

DROP TYPE
  lang,
  gender;
