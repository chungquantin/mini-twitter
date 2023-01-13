use crate::misc::{Identifier, UnixTimestamp};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Tweet {
    pub tweet_id: Identifier,
    user_id: Identifier,
    pub tweet_ts: UnixTimestamp,
    pub tweet_text: String,
}

impl Tweet {
    pub fn author(&self) -> Identifier {
        self.user_id
    }
}
