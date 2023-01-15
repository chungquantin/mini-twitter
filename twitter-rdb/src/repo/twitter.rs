use crate::{
    errors::DatabaseError,
    misc::Identifier,
    models::{Document, SimpleTransaction, Tweet},
};

impl_repository!(TwitterRepository);

impl<'a> TwitterRepository<'a> {
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
