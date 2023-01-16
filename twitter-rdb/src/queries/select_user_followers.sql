SELECT  follow_id
       ,from_id
       ,to_id
       ,follow_ts
FROM Follows
WHERE to_id = ($1) LIMIT ($2) OFFSET ($3)