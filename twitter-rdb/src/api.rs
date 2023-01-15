use crate::{
    errors::DatabaseError,
    misc::Identifier,
    models::{SimpleTransaction, Tweet},
    repo::TwitterRepository,
    storage::DatabaseRef,
};

pub struct TwitterApi {
    repo: TwitterRepository,
}

impl TwitterApi {
    pub fn new(db_ref: DatabaseRef) -> TwitterApi {
        TwitterApi {
            repo: TwitterRepository::new(db_ref),
        }
    }

    pub async fn post_tweet(&mut self, t: Tweet) -> Result<(), DatabaseError> {
        let tx = &mut self.repo.mut_tx().await;
        self.repo.create_tweet(tx, t).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_timeline(&mut self, user_id: Identifier) -> Result<Vec<Tweet>, DatabaseError> {
        let tx = &mut self.repo.mut_tx().await;
        self.repo.get_user_tweets(tx, user_id).await?;
        tx.commit().await?;

        Ok(vec![])
    }

    pub fn get_followers(user_id: Identifier) -> Vec<Identifier> {
        vec![]
    }
    pub fn get_followees(user_id: Identifier) -> Vec<Identifier> {
        vec![]
    }
    pub fn get_tweets(user_id: Identifier) -> Vec<Tweet> {
        vec![]
    }
}
