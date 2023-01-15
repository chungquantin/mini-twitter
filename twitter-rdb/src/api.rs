use crate::{
    errors::DatabaseError,
    misc::Identifier,
    models::{SimpleTransaction, Tweet},
    repo::TwitterRepository,
    storage::DatabaseRef,
};

pub struct TwitterApi<'a> {
    repo: TwitterRepository<'a>,
}

impl<'a> TwitterApi<'a> {
    pub fn new(db_ref: DatabaseRef<'a>) -> TwitterApi<'a> {
        TwitterApi {
            repo: TwitterRepository::new(db_ref),
        }
    }

    pub async fn post_tweet(&mut self, t: Tweet) -> Result<(), DatabaseError> {
        // action initialization
        let tx = &mut self.repo.mut_tx();
        // action dispatch
        self.repo.create_tweet(tx, t).await?;
        // action finished
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_timeline(&mut self, user_id: Identifier) -> Result<Vec<Tweet>, DatabaseError> {
        // action initialization
        let tx = &mut self.repo.mut_tx();
        // action dispatch
        self.repo.get_user_tweets(tx, user_id).await?;
        // action finished
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
