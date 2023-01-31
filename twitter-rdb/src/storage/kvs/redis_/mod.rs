mod tx;
mod ty;

pub use tx::*;
pub use ty::*;

use async_trait::async_trait;

use crate::{
    errors::DatabaseError,
    structures::{DBTransaction, DatabaseAdapter, DatabaseType, ImplDatabase},
};
use redis::{Client, Connection};

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
        let connection = &mut self.get_mut_inner().db_instance.get_connection()?;
        let static_connection = unsafe { extend_conn_lifetime(connection) };
        Ok(DBTransaction::<TxType>::new(static_connection, w, false).unwrap())
    }
}

unsafe fn extend_conn_lifetime(conn: &mut Connection) -> &'static mut Connection {
    std::mem::transmute::<&mut Connection, &'static mut Connection>(conn)
}
