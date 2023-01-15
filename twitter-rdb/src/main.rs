use api::TwitterApi;
use auth::Auth;
use errors::DatabaseError;
use models::Tweet;
use storage::{Database, DatabaseVariant};

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
    let connection_str = "postgresql://chungquantin:password@localhost:5433/postgres";
    let mut database = Database::new(DatabaseVariant::Postgres, connection_str);
    let database_ref = database.borrow();

    let mut twitter_api = TwitterApi::new(database_ref);
    let auth = Auth { user_id: 1 };

    twitter_api
        .post_tweet(Tweet::partial_new(auth.user_id, "Hello world".to_string()))
        .await?;

    Ok(())
}
