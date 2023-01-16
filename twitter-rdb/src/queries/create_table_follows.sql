CREATE TABLE IF NOT EXISTS Follows (
	follow_id INT GENERATED ALWAYS AS IDENTITY,
	from_id INT NOT NULL,
	to_id INT NOT NULL,
	follow_ts TIMESTAMP DEFAULT current_timestamp,
	PRIMARY KEY(follow_id)
)