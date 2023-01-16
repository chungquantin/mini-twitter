use std::time::SystemTime;

use crate::misc::{Identifier, UnixTimestamp};
use serde::{Deserialize, Serialize};

use super::{FromSuperValues, SuperValue};

#[derive(Debug, Serialize, Deserialize)]
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

impl FromSuperValues for Tweet {
    fn from_super_value(v: Vec<SuperValue>) -> Self {
        Tweet {
            tweet_id: *v[0].get::<Identifier>().unwrap(),
            user_id: *v[1].get::<Identifier>().unwrap(),
            tweet_text: v[2].get::<String>().unwrap().clone(),
            tweet_ts: *v[3].get::<SystemTime>().unwrap(),
        }
    }
}

impl Tweet {
    pub fn author(&self) -> Identifier {
        self.user_id
    }

    pub fn partial_new(tweet_text: &'static str) -> Tweet {
        Tweet {
            tweet_text: tweet_text.to_string(),
            ..Default::default()
        }
    }
}
