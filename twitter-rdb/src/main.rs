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

    let mut twitter_api = TwitterApi::new(database_ref);
    let auth = Auth { user_id: 1 };

    twitter_api
        .post_tweet(Tweet::partial_new(auth.user_id, "Hello world".to_string()))
        .await?;

    Ok(())
}
