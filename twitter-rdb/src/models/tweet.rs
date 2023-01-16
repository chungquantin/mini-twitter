use std::time::SystemTime;

use crate::misc::{Identifier, UnixTimestamp};
use crate::structures::FromPostgresRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub tweet_id: Identifier,
    user_id: Identifier,
    pub tweet_ts: UnixTimestamp,
    pub tweet_text: String,
}

impl Default for Tweet {
    fn default() -> Self {
        Self {
            tweet_ts: SystemTime::now(),
            tweet_id: Default::default(),
            user_id: Default::default(),
            tweet_text: Default::default(),
        }
    }
}

impl FromPostgresRow for Tweet {
    fn from_pg_row(r: tokio_postgres::Row) -> Self {
        Tweet {
            tweet_id: r.get(0),
            user_id: r.get(1),
            tweet_text: r.get(2),
            tweet_ts: r.get(3),
        }
    }
}

impl Tweet {
    pub fn author(&self) -> Identifier {
        self.user_id
    }

    pub fn partial_new(user_id: Identifier, tweet_text: String) -> Tweet {
        Tweet {
            user_id,
            tweet_text,
            ..Default::default()
        }
    }
}
