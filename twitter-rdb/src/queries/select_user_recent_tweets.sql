SELECT  tweet_id
       ,user_id
       ,tweet_text
       ,tweet_ts
FROM Tweets
WHERE user_id = ($1) 
ORDER BY tweet_ts DESC
LIMIT ($2) OFFSET ($3)
