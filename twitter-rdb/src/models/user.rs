use crate::misc::{Identifier, UnixTimestamp};
use serde::{Deserialize, Serialize};

/// TODO: Using Postgres Types to convert to a valid postgres type
#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: Identifier,
    pub username: String,
    pub user_ts: UnixTimestamp,
}
