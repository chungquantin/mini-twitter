mod tx;
mod ty;

use std::cell::Cell;

pub use tx::*;
pub use ty::*;

use crate::{
    errors::DatabaseError,
    models::{DBTransaction, DatabaseAdapter, DatabaseType, ImplDatabase},
};
use postgres::{Client, NoTls};

pub struct PostgresAdapter(DatabaseAdapter<DBType>);

impl PostgresAdapter {
    impl_new_database!(DBType);

    pub fn client(&mut self) -> Result<&mut Client, DatabaseError> {
        let c = &mut self.0.db_instance;
        return Ok(c.as_mut().get_mut());
    }

    pub fn new(connection_str: &str) -> Result<PostgresAdapter, DatabaseError> {
        let client = Client::connect(connection_str, NoTls)?;
        Ok(PostgresAdapter(DatabaseAdapter::<DBType>::new(
            connection_str.to_string(),
            Box::new(Cell::new(client)),
            DatabaseType::RelationalStore,
        )?))
    }
}

impl ImplDatabase for PostgresAdapter {
    type Transaction = PostgresTransaction;

    fn default() -> Self {
        let connection_str = "postgresql://chungquantin:password@localhost:5433/postgres";
        PostgresAdapter::new(connection_str).unwrap()
    }

    fn spawn(&self) -> Self {
        PostgresAdapter::default()
    }

    fn connection(&self) -> &str {
        &self.0.connection_str
    }

    fn transaction(&mut self, w: bool) -> Result<PostgresTransaction, DatabaseError> {
        let db = self.client()?;
        let tx = db.transaction().unwrap();

        let tx = unsafe { extend_tx_lifetime(tx) };

        Ok(DBTransaction::<TxType>::new(tx, w).unwrap())
    }
}

unsafe fn extend_tx_lifetime(tx: postgres::Transaction<'_>) -> postgres::Transaction<'static> {
    std::mem::transmute::<postgres::Transaction<'_>, postgres::Transaction<'static>>(tx)
}
