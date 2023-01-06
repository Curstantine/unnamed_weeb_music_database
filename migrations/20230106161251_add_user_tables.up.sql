-- Add up migration script here
CREATE TYPE access_level AS ENUM (
	'Admin',
	'Moderator',
	'Contributor',
	'User'
);

CREATE TABLE users (
	id text PRIMARY KEY,
	email varchar(255) NOT NULL UNIQUE,
	username text NOT NULL UNIQUE,
	password_hash varchar(255) NOT NULL,
	access_level access_level NOT NULL,
	created_at timestamptz DEFAULT NOW() NOT NULL,
	updated_at timestamptz DEFAULT NOW() NOT NULL
);

CREATE TABLE refresh_tokens (
	id SERIAL PRIMARY KEY,
	user_id TEXT NOT NULL,
	token TEXT NOT NULL,
	expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
	revoked BOOLEAN DEFAULT false NOT NULL,
	UNIQUE (token),
	FOREIGN KEY (user_id) REFERENCES users(id)
);