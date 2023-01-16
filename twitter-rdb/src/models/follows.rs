use std::time::SystemTime;

use crate::{misc::Identifier, structures::FromPostgresRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl FromPostgresRow for Follow {
    fn from_pg_row(r: tokio_postgres::Row) -> Self {
        Follow {
            id: r.get(0),
            user_id: r.get(1),
            follows_id: r.get(2),
            follows_ts: r.get(3),
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
