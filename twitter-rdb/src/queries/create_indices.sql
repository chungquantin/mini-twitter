CREATE INDEX tweet_ts_index
ON Tweets (tweet_ts);

CREATE INDEX follow_from_id_index
ON Follows (from_id);