use std::time::SystemTime;

use crate::misc::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Follows {
    id: Identifier,
    user_id: Identifier,
    follows_id: Identifier,
    follows_ts: SystemTime,
}

impl Default for Follows {
    fn default() -> Self {
        Self {
            follows_ts: SystemTime::now(),
            follows_id: Default::default(),
            user_id: Default::default(),
            id: Default::default(),
        }
    }
}

impl Follows {
    pub fn partial_new(from: Identifier, to: Identifier) -> Self {
        Follows {
            user_id: from,
            follows_id: to,
            ..Default::default()
        }
    }

    pub fn from(&self) -> Identifier {
        self.user_id
    }

    pub fn to(&self) -> Identifier {
        self.follows_id
    }
}
