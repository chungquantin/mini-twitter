SELECT  tweet_id
       ,user_id
       ,tweet_text
       ,tweet_ts
FROM Tweets
WHERE user_id = ($1) LIMIT ($2) OFFSET ($3)