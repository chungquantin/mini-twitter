use crate::{
    auth::Auth,
    errors::DatabaseError,
    misc::Identifier,
    models::{Follows, SimpleTransaction, Tweet},
    repo::TwitterRepository,
    storage::DatabaseRef,
};

pub struct TwitterApi {
    repo: TwitterRepository,
    auth: Auth,
}

impl TwitterApi {
    pub fn new(db_ref: DatabaseRef, auth: Auth) -> TwitterApi {
        TwitterApi {
            repo: TwitterRepository::new(db_ref),
            auth: auth.clone(),
        }
    }

    pub async fn load_follows(&mut self, f: Vec<Follows>) -> Result<(), DatabaseError> {
        let tx = &mut self.repo.mut_tx().await;
        for follow in f {
            self.repo
                .create_follow(tx, follow.from(), follow.to())
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn post_tweet(&mut self, t: Tweet) -> Result<(), DatabaseError> {
        let tx = &mut self.repo.mut_tx().await;
        self.repo
            .create_tweet(tx, self.auth.user_id, t.tweet_text)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn batch_post_tweets(&mut self, t: &[Tweet; 5]) -> Result<(), DatabaseError> {
        let tx = &mut self.repo.mut_tx().await;
        self.repo
            .batch_create_tweets(tx, self.auth.user_id, t)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_timeline(&self, user_id: Identifier) -> Result<Vec<Tweet>, DatabaseError> {
        let followees = self.get_followees(user_id);

        Ok(vec![])
    }

    // pub async fn get_followers(&mut self, user_id: Identifier) -> Vec<Identifier> {
    //     let tx = &mut self.repo.tx().await;
    //     let tweets = self.repo.get_user_tweets(tx, user_id).await?;
    //     Ok(tweets)
    // }

    pub fn get_followees(&self, user_id: Identifier) -> Vec<Identifier> {
        vec![]
    }

    pub async fn get_tweets(
        &mut self,
        user_id: Identifier,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Tweet>, DatabaseError> {
        let tx = &mut self.repo.tx().await;
        let tweets = self
            .repo
            .get_user_tweets(tx, user_id, limit, offset)
            .await?;
        Ok(tweets)
    }
}
