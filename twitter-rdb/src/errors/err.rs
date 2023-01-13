use postgres::Error as PostgresError;
use thiserror::Error;

use crate::storage::DatabaseVariant;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("[`{0}`] database error: `{1}`")]
    Database(DatabaseVariant, String),
    #[error("unknown data store error")]
    Unknown,
}

impl From<PostgresError> for DatabaseError {
    fn from(err: PostgresError) -> DatabaseError {
        DatabaseError::Unknown
    }
}
