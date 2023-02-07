use crate::{
    errors::DatabaseError,
    keywords,
    misc::{Arg, Key},
    structures::{
        DBTransaction, Document, FromPostgresRow, FromRedisValue, KeywordBucket, SimpleTransaction,
        SuperValue,
    },
    REDIS_STRATEGY,
};
use async_trait::async_trait;
use chrono::{self, Utc};
use redis::{aio::Connection, AsyncCommands};
use uuid::Uuid;

use super::ty::TxType;

#[async_trait(?Send)]
impl SimpleTransaction for DBTransaction<TxType> {
    fn closed(&self) -> bool {
        self.ok
    }

    async fn cancel(&mut self) -> Result<(), DatabaseError> {
        if self.ok {
            return Err(DatabaseError::TxFinished);
        }

        self.ok = true;

        Ok(())
    }

    async fn commit(&mut self) -> Result<(), DatabaseError> {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        self.ok = true;

        Ok(())
    }

    async fn set<K, A>(
        &mut self,
        key: K,
        args: A,
        _keywords: KeywordBucket,
    ) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send,
    {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        let mut guarded_tx = self.tx.lock().await;
        let conn = guarded_tx.as_mut().unwrap();
        let key: Key = key.into();
        let args = args.into();

        let params = to_redis_params(args);
        key.execute_redis(conn, params.as_slice(), keywords!())
            .await?;

        Ok(())
    }

    async fn multi_set<K, A>(&mut self, key: K, args: Vec<A>) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send,
    {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        let mut guarded_tx = self.tx.lock().await;
        let conn = guarded_tx.as_mut().unwrap();
        let key: Key = key.into();

        let mut batch_params = vec![];
        for arg in args {
            let arg = arg.into();
            let mut pg_params = to_redis_params(arg);
            batch_params.append(&mut pg_params);
        }

        key.execute_redis(
            conn,
            batch_params.as_slice(),
            keywords!(
                "is_batch" => "_".to_string()
            ),
        )
        .await?;
        Ok(())
    }

    async fn get<K, A, V>(
        &self,
        key: K,
        args: A,
        keywords: KeywordBucket,
    ) -> Result<Vec<V>, DatabaseError>
    where
        A: Into<Arg> + Send,
        K: Into<Key> + Send,
        V: FromPostgresRow + FromRedisValue,
    {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        let mut guarded_tx = self.tx.lock().await;
        let conn = guarded_tx.as_mut().unwrap();
        let key: Key = key.into();
        let args = args.into();
        let params = to_redis_params(args);
        let data: Vec<V> = key
            .query_redis::<V>(conn, params.as_slice(), keywords)
            .await?;

        Ok(data)
    }
}

type RedisReturnType = Box<String>;
fn to_redis_params(params: Vec<SuperValue>) -> Vec<RedisReturnType> {
    let mut result: Vec<RedisReturnType> = vec![];

    for item in params {
        macro_rules! param_convert {
            ($($SuperValueType: ident),*) => {
                match item {
                    $(
                        SuperValue::$SuperValueType(v) => result.push(
                            Box::new(v.to_string())
                        ),
                    )*
                    _ => unimplemented!()
                }
            };
        }
        param_convert!(String, Integer, BigInteger, SmallInteger, Char);
    }

    result
}

impl Document {
    pub async fn query_redis<T>(
        self: &Self,
        conn: &mut Connection,
        args: &[Box<String>],
        keywords: KeywordBucket,
    ) -> Result<Vec<T>, DatabaseError>
    where
        T: FromRedisValue,
    {
        let tag = keywords.get("tag").unwrap();
        match self {
            Document::Tweets => {
                if tag == "user_timeline" {
                    if REDIS_STRATEGY == 1 {
                        println!("Running strategy 1...");
                        let id = format!("FOLLOWS:{}", &args[0]);
                        let followees: Vec<String> = conn.lrange(id, 0, -1).await?;
                        let mut tweets_ids: Vec<String> = vec![];
                        for followee in followees.into_iter() {
                            let followee_tweets_ids: Vec<String> = conn
                                .lrange(format!("USERS:{}", followee).to_string(), 0, -1)
                                .await?;
                            for id in followee_tweets_ids {
                                let tweet: String = conn.get(id).await?;
                                tweets_ids.push(tweet);
                            }
                        }
                        tweets_ids.sort_by(|a, b| {
                            let slice_a = &a[a.len() - 10..a.len()];
                            let slice_b = &b[b.len() - 10..b.len()];
                            slice_a.cmp(&slice_b)
                        });

                        let mut result = vec![];
                        for tweet in tweets_ids {
                            result.push(T::from_redis_value(tweet));
                        }

                        return Ok(result);
                    } else if REDIS_STRATEGY == 2 {
                        let timeline = format!("USER_TIMELINE:{}", &args[0]);
                        let tweets: Vec<String> = conn.zrange(timeline, 0, 10).await?;
                        let mut result = vec![];
                        for tweet in tweets {
                            result.push(T::from_redis_value(tweet));
                        }

                        return Ok(result);
                    }
                }
            }
            _ => unimplemented!(),
        };

        Ok(vec![])
    }

    pub async fn execute_redis(
        self: &Self,
        conn: &mut Connection,
        args: &[Box<String>],
        keywords: KeywordBucket,
    ) -> Result<(), DatabaseError> {
        match self {
            Document::Tweets => {
                // Redis command: HSET Tweets:author_id (id uuidV4) (tweet_ts timestamp) (tweet_text msg) (author id)
                let uuidv4 = Uuid::new_v4();
                // (id uuidV4)
                let timestamp = Utc::now().timestamp().to_string();
                // Add tweets to set owned by the tweet author
                let is_batch = keywords.get("is_batch");
                let user_id = &format!("Users:{}", args[0]).to_string();
                match is_batch {
                    Some(_) => {
                        // IF REDIS_STRATEGY is defined
                        if REDIS_STRATEGY > 0 {
                            let mut pipeline = &mut redis::pipe();
                            let mut batch_id = 0;
                            while batch_id < args.len() {
                                let id = &format!("TWEETS:{}", uuidv4);
                                let author_id = &args[batch_id];
                                let content = &format!(
                                    "{}:{}:{}:{}",
                                    uuidv4,
                                    author_id,
                                    args[batch_id + 1],
                                    timestamp
                                );

                                pipeline = pipeline.set(id, content).rpush(user_id, id);
                                if REDIS_STRATEGY == 2 {
                                    // Add tweet to follower home timeline
                                    let followed_id = format!("FOLLOWED:{}", author_id);
                                    let followers: Vec<String> =
                                        conn.lrange(followed_id, 0, -1).await?;
                                    for follower in followers {
                                        let timeline_id = format!("USER_TIMELINE:{}", follower);
                                        pipeline.zadd(timeline_id, content, timestamp.clone());
                                    }
                                }
                                batch_id += 2;
                            }
                            pipeline.query_async(conn).await?;
                        }
                    }
                    None => {
                        let id = &format!("TWEETS:{}", uuidv4);
                        conn.rpush(user_id, id).await?;
                    }
                }
            }
            Document::Follows => {
                // Initialize follows relationship using lists data structure
                let (from, to) = (&args[0], &args[1]);
                // Redis command: LPUSH [from]:Follows [to]
                let follow_id = format!("FOLLOWS:{}", from);
                conn.lpush(&follow_id, to.to_string()).await?;
                // Redis command: LPUSH Followed:[to] [from]
                let followed_id = format!("FOLLOWED:{}", to);
                conn.lpush(&followed_id, from.to_string()).await?;
            }
            _ => unimplemented!(),
        };
        Ok(())
    }
}
