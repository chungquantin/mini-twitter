use crate::misc::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Follows {
    user_id: Identifier,
    follows_id: Identifier,
}

impl Follows {
    pub fn from(&self) -> Identifier {
        self.user_id
    }

    pub fn to(&self) -> Identifier {
        self.follows_id
    }
}
