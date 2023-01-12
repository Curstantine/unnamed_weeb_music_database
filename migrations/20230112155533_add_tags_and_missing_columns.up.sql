-- Add up migration script here
CREATE TABLE IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY,
    name text NOT null UNIQUE,
    description text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS song_tags (
    id SERIAL PRIMARY KEY,
    song_id TEXT REFERENCES songs(id),
    tag_id INTEGER REFERENCES tags(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (song_id, tag_id)
);

CREATE TABLE IF NOT EXISTS release_tags (
    id SERIAL PRIMARY KEY,
    release_id TEXT REFERENCES releases(id),
    tag_id INTEGER REFERENCES tags(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (release_id, tag_id)
);

ALTER TABLE SONGS ADD COLUMN release_date DATE;
