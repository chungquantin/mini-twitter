SELECT  *
FROM Follows
WHERE from_id = ($1)
LIMIT 1 OFFSET FLOOR(random() * (
SELECT  COUNT(*)
FROM Follows ));