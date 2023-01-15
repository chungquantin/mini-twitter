use crate::storage::{Database, DatabaseRef, Transaction};
use crate::{
    errors::DatabaseError,
    misc::Identifier,
    models::{Document, SimpleTransaction, Tweet},
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
        tweet: Tweet,
    ) -> Result<(), DatabaseError> {
        tx.set(Document::Tweets, vec![tweet.tweet_text]).await?;

        Ok(())
    }

    pub async fn get_user_tweets(
        &mut self,
        tx: &Transaction,
        user_id: Identifier,
    ) -> Result<(), DatabaseError> {
        tx.get_filtered(Document::Tweets, vec![user_id.to_string()])
            .await?;

        Ok(())
    }
}
