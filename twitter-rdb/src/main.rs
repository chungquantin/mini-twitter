use api::TwitterApi;
use colored::Colorize;
use errors::DatabaseError;
use models::Tweet;
use std::time::Instant;
use storage::{Database, DatabaseRef, DatabaseVariant};

use crate::{
    models::Follow,
    utils::{average, load_from_csv, log_stage},
};

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

pub fn start_benchmarking(stage: &'static str, title: &'static str) -> Instant {
    log_stage(stage, title);
    Instant::now()
}

pub fn stop_benchmarking(instant: Instant) {
    let elapsed = instant.elapsed();
    println!(
        "==> Total execution time: {}",
        format!("{:.2?}", elapsed).blue()
    );
}

#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    let connection_str = "user=postgres host=localhost port=5433";
    let database = Database::connect(DatabaseVariant::Postgres, connection_str).await;
    let database_ref = DatabaseRef::new(database);
    let mut twitter_api = TwitterApi::new(database_ref);
    let use_sample = false;

    let mut t = start_benchmarking("PREPARATION", "Load tweets from CSV file");
    let mut loaded_tweets = vec![];
    let tweets_records = load_from_csv(if use_sample {
        "./dataset/tweet_sample.csv"
    } else {
        "./dataset/tweet.csv"
    });
    for record in tweets_records {
        let user_id = record.get(0).unwrap();
        let parsed_user_id = user_id.parse::<i32>().unwrap();
        let tweet_text = record.get(1).unwrap().to_string();
        let tweet = Tweet::partial_new(parsed_user_id.clone(), tweet_text);
        loaded_tweets.push(tweet);
    }
    stop_benchmarking(t);

    t = start_benchmarking("PREPARATION", "Load and populate follows from CSV file");
    let follows_records = load_from_csv(if use_sample {
        "./dataset/follows_sample.csv"
    } else {
        "./dataset/follows.csv"
    });
    let mut follows = vec![];
    for record in follows_records {
        let user_id = record.get(0).unwrap();
        let parsed_user_id = user_id.parse::<i32>().unwrap();
        let follow_id = record.get(1).unwrap();
        let parsed_follow_id = follow_id.parse::<i32>().unwrap();
        let follow = Follow::partial_new(parsed_user_id.clone(), parsed_follow_id.clone());
        follows.push(follow);
    }
    twitter_api.batch_create_follows(follows).await?;
    stop_benchmarking(t);

    // t = start_benchmarking("POST TWEETS", "Using single insert");
    // // time per request
    // let mut tps_ptr = Instant::now();
    // let mut tps: Vec<u128> = vec![];
    // for tweet in loaded_tweets.to_vec() {
    //     twitter_api.post_tweet(tweet).await?;
    //     let cur = tps_ptr.elapsed();
    //     tps.push(cur.as_millis());
    //     tps_ptr = Instant::now();
    // }
    // let avg_tps = average(tps);
    // println!(
    //     "==> Average time per request: {} ms",
    //     format!("{}", avg_tps).blue()
    // );
    // stop_benchmarking(t);

    t = start_benchmarking("POST TWEETS", "Batch insert | Batch size = 5");
    let mut tps_ptr = Instant::now();
    let mut tps: Vec<u128> = vec![];

    let batch_size = 5;
    let mut cur = 0;
    while cur <= loaded_tweets.len() {
        let end = std::cmp::min(cur + batch_size, loaded_tweets.len());
        let batch = &loaded_tweets.as_slice()[cur..end];
        let cur_dur = tps_ptr.elapsed();
        tps.push(cur_dur.as_nanos());

        cur = cur + batch_size;
        if batch.len() < batch_size {
            for tweet in batch.to_vec() {
                twitter_api.post_tweet(tweet).await?;
            }
        } else {
            twitter_api.batch_post_tweets(batch.to_vec()).await?;
        }

        tps_ptr = Instant::now();
    }
    let avg_tps = average(tps);
    println!(
        "==> Average time per request: {} ns",
        format!("{:.2?}", avg_tps).blue()
    );
    stop_benchmarking(t);

    let user_id = 1;

    t = start_benchmarking("USER TIMELINE", "Return user followers");
    let tweets = twitter_api.get_followers(user_id, None, None).await?;
    println!("{:?}", tweets);
    stop_benchmarking(t);

    t = start_benchmarking("USER TIMELINE", "Return user followees");
    let tweets = twitter_api.get_followees(user_id, None, None).await?;
    println!("{:?}", tweets);
    stop_benchmarking(t);

    t = start_benchmarking("USER TIMELINE", "Return that random user’s home timeline");
    let tweets = twitter_api.get_timeline(user_id).await?;
    println!("{:?}", tweets);
    stop_benchmarking(t);

    Ok(())
}
