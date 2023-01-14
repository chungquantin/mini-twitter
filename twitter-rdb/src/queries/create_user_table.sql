CREATE TABLE IF NOT EXISTS User (
	id              SERIAL PRIMARY KEY,
	username        VARCHAR UNIQUE NOT NULL,
	user_ts timestamp default current_timestamp
)

