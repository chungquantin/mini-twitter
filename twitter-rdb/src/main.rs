use api::TwitterApi;
use auth::Auth;
use errors::DatabaseError;
use models::Tweet;
use storage::{Database, DatabaseRef, DatabaseVariant};

mod api;
mod auth;
mod constants;
mod errors;
#[macro_use]
mod macros;
mod misc;
mod models;
mod repo;
mod storage;
mod utils;

#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    let connection_str = "user=postgres host=localhost port=5433";
    let database = Database::connect(DatabaseVariant::Postgres, connection_str).await;
    let database_ref = DatabaseRef::new(database);

    let auth = Auth { user_id: 1 };
    let mut twitter_api = TwitterApi::new(database_ref, auth);

    twitter_api
        .post_tweet(Tweet::partial_new("Hello world"))
        .await?;

    twitter_api
        .batch_post_tweets(&[
            Tweet::partial_new("Batch tweet item 1"),
            Tweet::partial_new("Batch tweet item 2"),
            Tweet::partial_new("Batch tweet item 3"),
            Tweet::partial_new("Batch tweet item 4"),
            Tweet::partial_new("Batch tweet item 5"),
        ])
        .await?;

    let tweets = twitter_api.get_tweets(auth.user_id, None, None).await?;

    println!("{:?}", tweets);

    Ok(())
}
