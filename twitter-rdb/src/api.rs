use crate::{
    errors::DatabaseError,
    misc::Identifier,
    models::{Follow, Tweet},
    repo::TwitterRepository,
    storage::DatabaseRef,
    structures::SimpleTransaction,
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

    pub async fn post_tweet(&mut self, t: Tweet, save: bool) -> Result<(), DatabaseError> {
        let tx = &mut self.repo.mut_tx().await;
        self.repo.create_tweet(tx, t.author(), t.tweet_text).await?;
        if save {
            tx.commit().await?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn batch_post_tweets(
        &mut self,
        t: Vec<Tweet>,
        save: bool,
    ) -> Result<(), DatabaseError> {
        let tx = &mut self.repo.mut_tx().await;
        self.repo.batch_create_tweets(tx, t).await?;
        if save {
            tx.commit().await?;
        }
        Ok(())
    }

    pub async fn get_timeline(&mut self, user_id: Identifier) -> Result<Vec<Tweet>, DatabaseError> {
        let tx = self.repo.tx().await;
        let tweets = self.repo.get_timeline(tx, user_id).await?;
        Ok(tweets)
    }

    pub async fn get_followers(
        &mut self,
        user_id: Identifier,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Follow>, DatabaseError> {
        let tx = self.repo.tx().await;
        let tweets = self
            .repo
            .get_user_followers(tx, user_id, limit, offset)
            .await?;
        Ok(tweets)
    }

    pub async fn get_followees(
        &mut self,
        user_id: Identifier,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Follow>, DatabaseError> {
        let tx = self.repo.tx().await;
        let tweets = self
            .repo
            .get_user_followees(tx, user_id, limit, offset)
            .await?;
        Ok(tweets)
    }

    pub async fn get_most_recent_tweets(
        &mut self,
        user_id: Identifier,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Tweet>, DatabaseError> {
        let tx = &mut self.repo.tx().await;
        let tweets = self
            .repo
            .get_most_recent_tweets(tx, user_id, limit, offset)
            .await?;
        Ok(tweets)
    }
}
