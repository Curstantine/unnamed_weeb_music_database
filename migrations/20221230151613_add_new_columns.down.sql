-- Add down migration script here
/* Remove the new columns from the tables */
ALTER TABLE songs_artists DROP COLUMN join_phrase;
ALTER TABLE songs_releases RENAME COLUMN song_id TO songs_id;
ALTER TABLE songs_releases RENAME COLUMN release_id TO releases_id;
ALTER TABLE releases DROP COLUMN release_date;
ALTER TABLE releases DROP COLUMN label;
ALTER TABLE releases DROP COLUMN length;
ALTER TABLE releases DROP COLUMN script_language;
ALTER TABLE songs DROP COLUMN track_length;
ALTER TABLE artists DROP COLUMN based_in;
ALTER TABLE artists DROP COLUMN founded_in;
ALTER TABLE artists DROP COLUMN artist_type;

/* Drop the tables created in the up migration */
DROP TYPE IF EXISTS artist_type;