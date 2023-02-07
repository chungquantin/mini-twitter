CREATE TABLE IF NOT EXISTS Tweets (
	tweet_id INT GENERATED ALWAYS AS IDENTITY,
	user_id INT NOT NULL,
	tweet_text VARCHAR(255) NOT NULL,
	tweet_ts TIMESTAMP DEFAULT current_timestamp,
	PRIMARY KEY(tweet_id)
	-- CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES Users(user_id)
)

