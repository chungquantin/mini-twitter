use crate::{
    errors::DatabaseError,
    misc::Identifier,
    models::{Follow, Tweet},
    repo::TwitterRepository,
    storage::{DatabaseRef, Transaction},
    structures::SimpleTransaction,
};

pub struct TwitterApi {
    pub repo: TwitterRepository,
}

impl TwitterApi {
    pub fn new(db_ref: DatabaseRef) -> TwitterApi {
        TwitterApi {
            repo: TwitterRepository::new(db_ref),
        }
    }

    #[allow(dead_code)]
    pub async fn batch_create_follows(
        &mut self,
        f: Vec<Follow>,
        save: bool,
    ) -> Result<(), DatabaseError> {
        let tx = &mut self.repo.mut_tx().await;
        for follow in f {
            self.repo
                .create_follow(tx, follow.from(), follow.to())
                .await?;
        }
        if save {
            tx.commit().await?;
        }
        Ok(())
    }

    pub async fn post_tweet(
        &mut self,
        t: Tweet,
        tx: &mut Transaction,
    ) -> Result<(), DatabaseError> {
        self.repo.create_tweet(tx, t.author(), t.tweet_text).await?;
        Ok(())
    }

    pub async fn batch_post_tweets(
        &mut self,
        t: Vec<Tweet>,
        tx: &mut Transaction,
    ) -> Result<(), DatabaseError> {
        self.repo.batch_create_tweets(tx, t).await?;
        Ok(())
    }

    pub async fn get_timeline(
        &mut self,
        user_id: Identifier,
        tx: &Transaction,
    ) -> Result<Vec<Tweet>, DatabaseError> {
        let tweets = self.repo.get_timeline(tx, user_id).await?;
        Ok(tweets)
    }
}
