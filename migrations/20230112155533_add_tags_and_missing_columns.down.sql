-- Add down migration script here

DROP TABLE IF EXISTS song_tags;
DROP TABLE IF EXISTS release_tags;
DROP TABLE IF EXISTS tags;

ALTER TABLE songs DROP COLUMN release_date;