CREATE INDEX tweet_user_id_index
ON Tweets (user_id);

CREATE INDEX follow_to_id_index
ON Follows (to_id);

CREATE INDEX follow_from_id_index
ON Follows (from_id);


