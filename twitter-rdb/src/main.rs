use crate::{models::Follow, utils::load_from_csv};
use api::TwitterApi;
use colored::Colorize;
use conn::DATABASE_CONNECTIONS;
use errors::DatabaseError;
use indicatif::ProgressBar;
use models::Tweet;
use rand::seq::SliceRandom;
use storage::{Database, DatabaseRef, DatabaseVariant};
use utils::{start_benchmarking, stop_benchmarking};

mod api;
mod conn;
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

static GLOBAL_USE_SAMPLE: bool = true;

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
) -> Result<Vec<i32>, DatabaseError> {
    let mut followers = Vec::<i32>::default();
    let mut unique_map = std::collections::HashMap::<i32, bool>::default();
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

        // Add followers if not in unique map
        unique_map.entry(parsed_user_id).or_insert_with_key(|user| {
            followers.push(*user);
            true
        });

        let follow_id = record.get(1).unwrap();
        let parsed_follow_id = follow_id.parse::<i32>().unwrap();
        let follow = Follow::partial_new(parsed_user_id.clone(), parsed_follow_id.clone());
        follows.push(follow);
    }

    twitter_api.batch_create_follows(follows, true).await?;
    stop_benchmarking(t);
    Ok(followers)
}

#[allow(dead_code)]
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
    let requests = loaded_tweets.len() as u64;
    let time = t.elapsed().as_secs();
    let div = time.checked_div(requests).unwrap();
    let rps = if div == 0 { requests } else { div };
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
    let requests = loaded_tweets.len() as u64;
    let time = t.elapsed().as_secs();
    let div = time.checked_div(requests).unwrap();
    let rps = if div == 0 { requests } else { div };
    println!("==> Request per second: {}", format!("{}", rps).blue());
    stop_benchmarking(t);

    Ok(())
}

fn get_connection_str(variant: DatabaseVariant) -> &'static str {
    DATABASE_CONNECTIONS.get(&variant).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    let variant = DatabaseVariant::Redis;
    let database = Database::connect(variant.clone(), get_connection_str(variant), false).await;
    let database_ref = DatabaseRef::new(database);
    let mut twitter_api = TwitterApi::new(database_ref);

    // First program:
    // Write one program that reads pre-generated tweets from the file tweets.csv. Note that the
    // file contains just the user_id and the text of the tweet.  Your code (or the database) should
    // auto-assign tweet_ids and timestamps as the tweet is loaded into the database. Keep track
    // of how long it takes to load all of the tweets. This program simulates users posting tweets.
    // How many tweets can be posted per second? (Twitter receives 6-10 thousand new tweets
    // per second. Can MySQL keep up?)  Insert tweets as you read them from the file. Batch no
    // more than 5 tweets at a time into the insert.
    let loaded_tweets = benchmark_load_tweets_from_csv();
    let followers = benchmark_load_follows_from_csv(&mut twitter_api).await?;
    benchmark_post_tweets_single_insert(&mut twitter_api, loaded_tweets.to_vec()).await?;
    benchmark_post_tweets_batch_insert(&mut twitter_api, loaded_tweets.to_vec()).await?;

    // Second Program:
    // Write a second program that repeatedly picks a random user and returns that user’s home
    // timeline: The 10 most recent tweets posted by any user that that user follows. For example,
    // if user A follows X, Y, and Z, then A’s home timeline consists of the 10 most recent tweets
    // posted by X, Y, or Z. This process simulates users opening the twitter app on their
    // smartphone and refreshing the home timeline to see new posts. How many home timelines
    // can be retrieved per second? Twitter users worldwide collectively refresh their home
    // timeline 200-300 thousand times per second. Can your program keep up?
    let t = start_benchmarking("USER TIMELINE", "Return that random user’s home timeline");
    let mut total_timelines_fetched = 0;
    while t.elapsed().as_secs() < 100 {
        // Repeatedly select random user from list of followers
        let user_id = followers.choose(&mut rand::thread_rng()).unwrap().clone();
        #[allow(unused)]
        let tweets = twitter_api.get_timeline(user_id).await;
        // println!("tweets: {:?}", tweets); // Uncomment this line to view the fetched tweets
        total_timelines_fetched += 1;
    }
    println!("Total timelines fetched: {}", total_timelines_fetched);
    let tps = total_timelines_fetched / t.elapsed().as_secs();
    println!("Timeline per second: {}", tps);
    stop_benchmarking(t);

    Ok(())
}
