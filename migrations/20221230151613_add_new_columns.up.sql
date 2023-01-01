-- Add up migration script here
/* songs_artists got a new column called join_phrase */
ALTER TABLE songs_artists ADD COLUMN join_phrase text;

/* new type artist_type */
CREATE TYPE artist_type AS ENUM('Solo','Group','Orchestra','Choir','Other');

/* songs_releases had a column called songs_id renamed to song_id, same with releases_id */
ALTER TABLE songs_releases RENAME COLUMN songs_id TO song_id;
ALTER TABLE songs_releases RENAME COLUMN releases_id TO release_id;

/* releases got a few new columns, release_date, label, length and script_language */
ALTER TABLE releases ADD COLUMN release_date date;
ALTER TABLE releases ADD COLUMN label text[];
ALTER TABLE releases ADD COLUMN length integer;
ALTER TABLE releases ADD COLUMN script_language text[];

/* songs got a new column called track_length */
ALTER TABLE songs ADD COLUMN track_length integer;

/* artists got a few new columns, based_in, founded_in(timestamptz) and artist_type */
ALTER TABLE artists ADD COLUMN based_in text[];
ALTER TABLE artists ADD COLUMN founded_in timestamptz;
ALTER TABLE artists ADD COLUMN artist_type artist_type;