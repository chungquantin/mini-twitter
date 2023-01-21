use api::TwitterApi;
use colored::Colorize;
use errors::DatabaseError;
use indicatif::ProgressBar;
use models::Tweet;
use storage::{Database, DatabaseRef, DatabaseVariant};
use utils::{start_benchmarking, stop_benchmarking};

use crate::{models::Follow, utils::load_from_csv};

mod api;
mod constants;
mod errors;
mod structures;
#[macro_use]
mod macros;
mod misc;
mod models;
mod repo;
mod storage;
mod utils;

static GLOBAL_USE_SAMPLE: bool = false;

fn benchmark_load_tweets_from_csv() -> Vec<Tweet> {
    let t = start_benchmarking("PREPARATION", "Load tweets from CSV file");
    let mut loaded_tweets = vec![];
    let tweets_records = load_from_csv(if GLOBAL_USE_SAMPLE {
        "./dataset/tweet_sample.csv"
    } else {
        "./dataset/tweet.csv"
    });
    let pb = ProgressBar::new(tweets_records.len().try_into().unwrap());
    for record in tweets_records {
        pb.inc(1);
        let user_id = record.get(0).unwrap();
        let parsed_user_id = user_id.parse::<i32>().unwrap();
        let tweet_text = record.get(1).unwrap().to_string();
        let tweet = Tweet::partial_new(parsed_user_id.clone(), tweet_text);
        loaded_tweets.push(tweet);
    }
    stop_benchmarking(t);

    loaded_tweets
}

async fn benchmark_load_follows_from_csv(
    twitter_api: &mut TwitterApi,
) -> Result<(), DatabaseError> {
    let t = start_benchmarking("PREPARATION", "Load and populate follows from CSV file");
    let follows_records = load_from_csv(if GLOBAL_USE_SAMPLE {
        "./dataset/follows_sample.csv"
    } else {
        "./dataset/follows.csv"
    });

    let mut follows = vec![];
    let pb = ProgressBar::new(follows_records.len().try_into().unwrap());
    for record in follows_records {
        pb.inc(1);
        let user_id = record.get(0).unwrap();
        let parsed_user_id = user_id.parse::<i32>().unwrap();
        let follow_id = record.get(1).unwrap();
        let parsed_follow_id = follow_id.parse::<i32>().unwrap();
        let follow = Follow::partial_new(parsed_user_id.clone(), parsed_follow_id.clone());
        follows.push(follow);
    }

    twitter_api.batch_create_follows(follows, true).await?;
    stop_benchmarking(t);
    Ok(())
}

async fn benchmark_post_tweets_single_insert(
    twitter_api: &mut TwitterApi,
    loaded_tweets: Vec<Tweet>,
) -> Result<(), DatabaseError> {
    let t = start_benchmarking("POST TWEETS", "Using single insert");
    let pb = ProgressBar::new(loaded_tweets.len().try_into().unwrap());
    for tweet in loaded_tweets.to_vec() {
        pb.inc(1);
        twitter_api.post_tweet(tweet, false).await?;
    }
    let rps = loaded_tweets.len() as u64 / t.elapsed().as_secs();
    println!("==> Request per second: {}", format!("{}", rps).blue());
    stop_benchmarking(t);
    Ok(())
}

async fn benchmark_post_tweets_batch_insert(
    twitter_api: &mut TwitterApi,
    loaded_tweets: Vec<Tweet>,
) -> Result<(), DatabaseError> {
    let t = start_benchmarking("POST TWEETS", "Batch insert | Batch size = 5");
    let batch_size: usize = 5;
    let mut cur = 0;
    let pb = ProgressBar::new(loaded_tweets.len().try_into().unwrap());
    while cur <= loaded_tweets.len() {
        pb.inc(batch_size.try_into().unwrap());
        let end = std::cmp::min(cur + batch_size, loaded_tweets.len());
        let batch = &loaded_tweets.as_slice()[cur..end];

        cur = cur + batch_size;
        if batch.len() < batch_size {
            for tweet in batch.to_vec() {
                twitter_api.post_tweet(tweet, true).await?;
            }
        } else {
            twitter_api.batch_post_tweets(batch.to_vec(), true).await?;
        }
    }
    let rps = loaded_tweets.len() as u64 / t.elapsed().as_secs();
    println!("==> Request per second: {}", format!("{}", rps).blue());
    stop_benchmarking(t);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    let connection_str = "user=postgres host=localhost port=5433";
    let database = Database::connect(DatabaseVariant::Postgres, connection_str).await;
    let database_ref = DatabaseRef::new(database);
    let mut twitter_api = TwitterApi::new(database_ref);

    let loaded_tweets = benchmark_load_tweets_from_csv();
    benchmark_load_follows_from_csv(&mut twitter_api).await?;
    // benchmark_post_tweets_single_insert(&mut twitter_api, loaded_tweets.to_vec()).await?;
    benchmark_post_tweets_batch_insert(&mut twitter_api, loaded_tweets.to_vec()).await?;

    let user_id = 1;

    let t = start_benchmarking("USER Followers", "Return user followers");
    let tweets = twitter_api.get_followers(user_id, None, None).await?;
    println!("{:?}", tweets);
    stop_benchmarking(t);

    let t = start_benchmarking("USER Followees", "Return user followees");
    let tweets = twitter_api.get_followees(user_id, None, None).await?;
    println!("{:?}", tweets);
    stop_benchmarking(t);

    let t = start_benchmarking("USER TIMELINE", "Return that random user’s home timeline");
    let mut total_timelines_fetched = 0;
    while t.elapsed().as_secs() < 100 {
        twitter_api.get_timeline(user_id).await?;
        total_timelines_fetched += 1;
    }
    println!("Total timelines fetched: {}", total_timelines_fetched);
    let tps = total_timelines_fetched / t.elapsed().as_secs();
    println!("Timeline per second: {}", tps);
    stop_benchmarking(t);

    Ok(())
}
