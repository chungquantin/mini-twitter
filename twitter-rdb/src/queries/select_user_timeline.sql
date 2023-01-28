-- Get tweets of a random followee
SELECT  t.tweet_id
       ,t.user_id
       ,t.tweet_text
       ,t.tweet_ts
FROM Tweets AS t
INNER JOIN
(
-- Get a random followee
	SELECT  *
	FROM Follows
	WHERE from_id = ($1)
	LIMIT 1 OFFSET FLOOR(random() * (
	SELECT COUNT(*)
	FROM Follows ))
) AS Follows
ON Follows.to_id = t.user_id
LIMIT ($2) OFFSET ($3);