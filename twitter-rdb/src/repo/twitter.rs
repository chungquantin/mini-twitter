use crate::{
    errors::DatabaseError,
    misc::Identifier,
    models::{Follow, Tweet},
    storage::{Database, DatabaseRef, Transaction},
    structures::{Document, SimpleTransaction, SuperValue},
};
use std::cell::Cell;

pub struct TwitterRepository {
    pub ds_ref: Cell<DatabaseRef>,
}

impl TwitterRepository {
    pub fn new(ds_ref: DatabaseRef) -> Self {
        TwitterRepository {
            ds_ref: Cell::new(ds_ref),
        }
    }

    fn db(&mut self) -> &mut Database {
        &mut self.ds_ref.get_mut().db
    }

    pub async fn tx(&mut self) -> Transaction {
        self.db().transaction(false).await.unwrap()
    }

    pub async fn mut_tx(&mut self) -> Transaction {
        self.db().transaction(true).await.unwrap()
    }
}

impl TwitterRepository {
    pub async fn create_tweet(
        &mut self,
        tx: &mut Transaction,
        user_id: Identifier,
        text: String,
    ) -> Result<(), DatabaseError> {
        tx.set(
            Document::Tweets,
            vec![SuperValue::Integer(user_id), SuperValue::String(text)],
        )
        .await?;

        Ok(())
    }

    pub async fn batch_create_tweets(
        &mut self,
        tx: &mut Transaction,
        tweets: Vec<Tweet>,
    ) -> Result<(), DatabaseError> {
        let mut params = vec![];
        for tweet in tweets {
            let sub_params = vec![
                SuperValue::Integer(tweet.author()),
                SuperValue::String(tweet.tweet_text.clone()),
            ];
            params.push(sub_params);
        }
        tx.multi_set(Document::Tweets, params).await?;

        Ok(())
    }

    pub async fn get_user_tweets(
        &mut self,
        tx: &Transaction,
        user_id: Identifier,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Tweet>, DatabaseError> {
        let tweets: Vec<Tweet> = tx
            .get_filtered(
                Document::Tweets,
                vec![
                    SuperValue::Integer(user_id),
                    match limit {
                        Some(l) => SuperValue::BigInteger(l),
                        None => SuperValue::BigInteger(i64::MAX),
                    },
                    match offset {
                        Some(o) => SuperValue::BigInteger(o),
                        None => SuperValue::BigInteger(0),
                    },
                ],
                &["user_tweets"],
            )
            .await?;

        Ok(tweets)
    }

    pub async fn create_follow(
        &mut self,
        tx: &mut Transaction,
        from: Identifier,
        to: Identifier,
    ) -> Result<(), DatabaseError> {
        tx.set(
            Document::Follows,
            vec![SuperValue::Integer(from), SuperValue::Integer(to)],
        )
        .await?;

        Ok(())
    }

    pub async fn get_user_followers(
        &mut self,
        tx: Transaction,
        user_id: Identifier,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Follow>, DatabaseError> {
        let followers: Vec<Follow> = tx
            .get_filtered(
                Document::Follows,
                vec![
                    SuperValue::Integer(user_id),
                    match limit {
                        Some(l) => SuperValue::BigInteger(l),
                        None => SuperValue::BigInteger(i64::MAX),
                    },
                    match offset {
                        Some(o) => SuperValue::BigInteger(o),
                        None => SuperValue::BigInteger(0),
                    },
                ],
                &["user_followers"],
            )
            .await?;

        Ok(followers)
    }

    pub async fn get_user_followees(
        &mut self,
        tx: Transaction,
        user_id: Identifier,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Follow>, DatabaseError> {
        let followers: Vec<Follow> = tx
            .get_filtered(
                Document::Follows,
                vec![
                    SuperValue::Integer(user_id),
                    match limit {
                        Some(l) => SuperValue::BigInteger(l),
                        None => SuperValue::BigInteger(i64::MAX),
                    },
                    match offset {
                        Some(o) => SuperValue::BigInteger(o),
                        None => SuperValue::BigInteger(0),
                    },
                ],
                &["user_followees"],
            )
            .await?;

        Ok(followers)
    }

    pub async fn get_random_followee(
        &mut self,
        tx: Transaction,
        user_id: Identifier,
    ) -> Result<Option<Follow>, DatabaseError> {
        let followers: Vec<Follow> = tx
            .get_filtered(
                Document::Follows,
                vec![SuperValue::Integer(user_id)],
                &["user_random_followee"],
            )
            .await?;

        Ok(followers.first().cloned())
    }
}
