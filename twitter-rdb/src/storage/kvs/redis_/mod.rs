mod tx;
mod ty;

pub use tx::*;
pub use ty::*;

use async_trait::async_trait;

use crate::{
    errors::DatabaseError,
    structures::{DBTransaction, DatabaseAdapter, DatabaseType, ImplDatabase},
};
use redis::Client;

pub struct RedisAdapter(DatabaseAdapter<DBType>);

impl RedisAdapter {
    impl_new_database!(DBType);

    pub async fn connect(
        connection_str: &str,
        _auto_reset: bool,
    ) -> Result<RedisAdapter, DatabaseError> {
        let client = Client::open(connection_str)?;

        Ok(RedisAdapter(DatabaseAdapter::<DBType>::new(
            connection_str.to_string(),
            Box::new(client),
            DatabaseType::KeyValueStore,
        )?))
    }
}

#[async_trait(?Send)]
impl ImplDatabase for RedisAdapter {
    type Transaction = RedisTransaction;

    fn connection(&self) -> &str {
        &self.0.connection_str
    }

    async fn transaction(&mut self, w: bool) -> Result<RedisTransaction, DatabaseError> {
        let mut connection = self
            .get_mut_inner()
            .db_instance
            .get_async_connection()
            .await?;
        // Switch Redis to MULTI mode for ACID transaction
        let status: String = redis::cmd("MULTI").query_async(&mut connection).await?;
        if &status != "OK" {
            return Err(DatabaseError::TxFailure);
        }
        Ok(DBTransaction::<TxType>::new(connection, w, false).unwrap())
    }
}
