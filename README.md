# Mini Twitter 🕊 💙
## Description
The project is made for "CS4300: Large-scale Information Storage & Retrieval". It is a minimal version of Twitter backend: [Read more about Twitter system design](https://github.com/donnemartin/system-design-primer/blob/master/solutions/system_design/twitter/README.md). The end goal of this project is to show how bad is relational database in handling large connected data which is solved by NoSQL databases. There are two test cases are covered to benchmar:
- `Get home timeline`: This use case is the critical key point of Twitter scalable system. While Twitter has other modular services like `FanoutService` and `TimelineService` that maximize the power of redis to fetch user home timeline on the fly, as soon as there is new post. However, in the scope of this project, Redis Pub / Sub is not used. I just try to mock the message queue by using `List` data structure in Redis.
- `Post tweets (Batch size: 5)`: Tweets are created using Redis pipeline and PostgreSQL multiple insert.
### About Redis strategy
- *Strategy 1*: When you post a tweet, it is a simple set operation, where the key is the tweet ID (perhaps 
“Tweet:12345”) and the value is the contents of the tweet. The getTimeline operation will require that look 
up the tweets of each user being followed and that you construct the home timeline on the fly.
 
- *Strategy 2*: When post each tweet, copy the tweet (or a reference to the tweet) to the user’s 
home timeline automatically.  Write performance should now be slower, but since the timeline is ready and 
waiting, fetching the timeline should be a much faster operation.
 
## Solution stack
- `Async runtime`: I choose Tokio as an async runtime that handle asynchronous requests from database APIs
- `Parallel engine`: Rust rayon library is a very good one for parallel sorting and mapping
- Primary design pattern is Adapter Pattern which can be observed easily if you read the source code
## Getting started
Entry file: `twitter/main.rs`
There are two databases adapted into the system to benchmark the performance of each databases (I suppose you are familiar with adapter pattern to understand the blackbox code thoroughly)
- Relational database (PostgreSQL): `DatabaseVariant::Postgres`
- In-memory database (Redis): `DatabaseVariant::Redis`
## Benchmarking
The result returned from benchmarking shows that (RPS stands for Request per Second):
| Database      | Post tweets (RPS) | Retrieve timeline (RPS)|
| ----------- | ----------- | -------- |
| Redis (Strategy 1)      |   20408.0     | 1838.0 |
| Redis (Strategy 2) | 6756.0| 5971.0|
| PostgreSQL  | 9433.0 |846.0 |
					
Check these functions from entry file for benchmarking and tuning
```rs
// Handle loading tweets from CSV
fn benchmark_load_tweets_from_csv() -> Vec<Tweet>

// Handle load follows relationship from CSV and populate to database
async fn benchmark_load_follows_from_csv(
    twitter_api: &mut TwitterApi,
    save: bool,
) -> Result<Vec<i32>, DatabaseError>

// Handle post single tweet at the time
async fn benchmark_post_tweets_single_insert(
    twitter_api: &mut TwitterApi,
    loaded_tweets: Vec<Tweet>,
) -> Result<(), DatabaseError>

// Handle bacth post tweets 
async fn benchmark_post_tweets_batch_insert(
    twitter_api: &mut TwitterApi,
    loaded_tweets: Vec<Tweet>,
) -> Result<(), DatabaseError>
```
To run the benchmarking code: `cd twitter && cargo run`
