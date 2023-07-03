CREATE TABLE users(
	user_id uuid PRIMARY KEY,
	username TEXT NOT NULL UNIQUE,
	password_hash CHAR(128) NOT NULL
);
