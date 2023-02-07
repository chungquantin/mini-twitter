use crate::{
    errors::DatabaseError,
    keywords,
    misc::Identifier,
    models::Tweet,
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
            keywords!(),
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

    pub async fn create_follow(
        &mut self,
        tx: &mut Transaction,
        from: Identifier,
        to: Identifier,
    ) -> Result<(), DatabaseError> {
        tx.set(
            Document::Follows,
            vec![SuperValue::Integer(from), SuperValue::Integer(to)],
            keywords!(),
        )
        .await?;

        Ok(())
    }

    pub async fn get_timeline(
        &mut self,
        tx: &Transaction,
        user_id: Identifier,
    ) -> Result<Vec<Tweet>, DatabaseError> {
        let tweets: Vec<Tweet> = tx
            .get(
                Document::Tweets,
                vec![
                    SuperValue::Integer(user_id),
                    SuperValue::BigInteger(10),
                    SuperValue::BigInteger(0),
                ],
                keywords!("tag" => String::from("user_timeline")),
            )
            .await?;

        Ok(tweets)
    }
}
