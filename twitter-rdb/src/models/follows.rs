use std::time::SystemTime;

use crate::misc::Identifier;
use serde::{Deserialize, Serialize};

use super::{FromSuperValues, SuperValue};

#[derive(Clone, Serialize, Deserialize)]
pub struct Follow {
    id: Identifier,
    user_id: Identifier,
    follows_id: Identifier,
    follows_ts: SystemTime,
}

impl Default for Follow {
    fn default() -> Self {
        Self {
            follows_ts: SystemTime::now(),
            follows_id: Default::default(),
            user_id: Default::default(),
            id: Default::default(),
        }
    }
}
impl FromSuperValues for Follow {
    fn from_super_value(v: Vec<SuperValue>) -> Self {
        Follow {
            id: *v[0].get::<Identifier>().unwrap(),
            user_id: *v[1].get::<Identifier>().unwrap(),
            follows_id: v[2].get::<Identifier>().unwrap().clone(),
            follows_ts: *v[3].get::<SystemTime>().unwrap(),
        }
    }
}

impl Follow {
    pub fn partial_new(from: Identifier, to: Identifier) -> Self {
        Follow {
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
