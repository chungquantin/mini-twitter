mod tx;
mod ty;

use std::cell::Cell;

use async_trait::async_trait;
use log::info;
pub use tx::*;
pub use ty::*;

use crate::{
    constants::get_sql_script,
    errors::DatabaseError,
    structures::{DBTransaction, DatabaseAdapter, DatabaseType, Document, ImplDatabase, SQLEvent},
};
use tokio_postgres::{Client, NoTls};

pub struct PostgresAdapter(DatabaseAdapter<DBType>);

impl PostgresAdapter {
    impl_new_database!(DBType);

    pub fn client(&mut self) -> Result<&mut Client, DatabaseError> {
        let c = &mut self.0.db_instance;
        return Ok(c.as_mut().get_mut());
    }

    pub async fn connect(
        connection_str: &str,
        auto_reset: bool,
    ) -> Result<PostgresAdapter, DatabaseError> {
        let (client, connection) = tokio_postgres::connect(connection_str, NoTls).await?;
        info!("POSTGRES: Connecting and initializing...");

        // Waiting for connection to settle
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        if auto_reset {
            info!("POSTGRES: Dropping existing tables...");
            client
                .batch_execute(&get_sql_script(Document::GENERAL, SQLEvent::Reset))
                .await?;
        }

        for table_name in vec!["Tweets", "Follows"].iter() {
            client
                .batch_execute(&get_sql_script(
                    Document::GENERAL,
                    SQLEvent::CreateTable(table_name.to_string()),
                ))
                .await?;
        }

        info!("POSTGRES: Connect and successfully initialize database");

        Ok(PostgresAdapter(DatabaseAdapter::<DBType>::new(
            connection_str.to_string(),
            Box::new(Cell::new(client)),
            DatabaseType::RelationalStore,
        )?))
    }
}

#[async_trait(?Send)]
impl ImplDatabase for PostgresAdapter {
    type Transaction = PostgresTransaction;

    fn connection(&self) -> &str {
        &self.0.connection_str
    }

    async fn transaction(&mut self, w: bool) -> Result<PostgresTransaction, DatabaseError> {
        let db = self.client()?;
        let tx = db.transaction().await.unwrap();
        let longer_lifetime_tx = unsafe { extend_tx_lifetime(tx) };
        Ok(DBTransaction::<TxType>::new(longer_lifetime_tx, w, false).unwrap())
    }
}

unsafe fn extend_tx_lifetime(
    tx: tokio_postgres::Transaction<'_>,
) -> tokio_postgres::Transaction<'static> {
    std::mem::transmute::<tokio_postgres::Transaction<'_>, tokio_postgres::Transaction<'static>>(tx)
}
