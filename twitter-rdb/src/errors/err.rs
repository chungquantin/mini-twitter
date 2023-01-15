use postgres::Error as PostgresError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("There was a problem with a datastore transaction: {0}")]
    Tx(String),

    /// There was an error when starting a new datastore transaction
    #[error("There was an error when starting a new datastore transaction")]
    TxFailure,

    /// The transaction was already cancelled or committed
    #[error("Couldn't update a finished transaction")]
    TxFinished,

    /// The current transaction was created as read-only
    #[error("Couldn't write to a read only transaction")]
    TxReadonly,

    /// The conditional value in the request was not equal
    #[error("Value being checked was not correct")]
    TxConditionNotMet,

    /// The key being mutated is not in the database
    #[error("The key is not in the database")]
    TxnKeyNotFound,

    /// The key being inserted in the transaction already exists
    #[error("The key being inserted already exists")]
    TxKeyAlreadyExists,

    #[error("Database instance is not initialized")]
    DbNotInitialized,

    #[error("Database Error: {0}")]
    Database(String),

    #[error("unknown data store error")]
    Unknown,
}

impl From<PostgresError> for DatabaseError {
    fn from(err: PostgresError) -> DatabaseError {
        DatabaseError::Database(err.to_string())
    }
}
