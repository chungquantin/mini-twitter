mod tx;
mod ty;

pub use tx::*;
pub use ty::*;

use async_trait::async_trait;

use crate::{
    errors::DatabaseError,
    structures::{DBTransaction, DatabaseAdapter, DatabaseType, ImplDatabase},
};
use redis::{Client, ConnectionLike};

pub struct RedisAdapter(DatabaseAdapter<DBType>);

impl RedisAdapter {
    impl_new_database!(DBType);

    pub async fn connect(
        connection_str: &str,
        auto_reset: bool,
    ) -> Result<RedisAdapter, DatabaseError> {
        let mut client = Client::open(connection_str)?;

        if auto_reset && client.is_open() {
            client.req_command(&redis::cmd("FLUSHALL"))?;
        }

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
        let connection = self
            .get_mut_inner()
            .db_instance
            .get_async_connection()
            .await?;

        Ok(DBTransaction::<TxType>::new(connection, w, false).unwrap())
    }
}
