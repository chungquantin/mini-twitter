use async_trait::async_trait;

use crate::errors::DatabaseError;

use super::SimpleTransaction;

pub enum DatabaseType {
    RelationalStore,
    KeyValueStore,
}

pub struct DatabaseAdapter<T> {
    pub connection_str: String,
    pub db_instance: T,
    pub variant: DatabaseType,
}

impl<T> DatabaseAdapter<T> {
    pub fn new(
        connection_str: String,
        db_instance: T,
        variant: DatabaseType,
    ) -> Result<Self, DatabaseError> {
        Ok(DatabaseAdapter {
            connection_str,
            db_instance,
            variant,
        })
    }
}

#[async_trait(?Send)]
pub trait ImplDatabase {
    type Transaction: SimpleTransaction;
    // # Create new database transaction
    // Set `rw` default to false means readable but not readable
    async fn transaction(&mut self, rw: bool) -> Result<Self::Transaction, DatabaseError>;

    fn connection(&self) -> &str;
}
