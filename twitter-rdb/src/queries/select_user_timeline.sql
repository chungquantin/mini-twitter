-- Get tweets of all user followers
SELECT  t.tweet_id
       ,t.user_id
       ,t.tweet_text
       ,t.tweet_ts
FROM Tweets AS t
INNER JOIN Follows
-- Join follows with tweets 
ON Follows.to_id = t.user_id 
-- Followed by user
AND Follows.from_id = ($1)
-- 10 most recent tweets
ORDER BY t.tweet_ts DESC
LIMIT ($2) OFFSET ($3);