use crate::misc::{Identifier, UnixTimestamp};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Tweet {
    pub tweet_id: Identifier,
    user_id: Identifier,
    pub tweet_ts: UnixTimestamp,
    pub tweet_text: String,
}

impl Tweet {
    pub fn author(&self) -> Identifier {
        self.user_id
    }

    pub fn partial_new(author: Identifier, tweet_text: String) -> Tweet {
        Tweet {
            tweet_text,
            user_id: author,
            ..Default::default()
        }
    }
}
