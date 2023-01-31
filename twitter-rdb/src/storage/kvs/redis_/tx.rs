use async_trait::async_trait;
use log::info;
use tokio_postgres::types::ToSql;

use crate::{
    errors::DatabaseError,
    misc::{Arg, Key},
    structures::{DBTransaction, FromPostgresRow, SimpleTransaction, SuperValue},
};

use super::ty::TxType;

#[async_trait(?Send)]
impl SimpleTransaction for DBTransaction<TxType> {
    fn closed(&self) -> bool {
        self.ok
    }

    async fn cancel(&mut self) -> Result<(), DatabaseError> {
        if self.ok {
            return Err(DatabaseError::TxFinished);
        }

        self.ok = true;

        let mut tx = self.tx.lock().await;
        unimplemented!();
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), DatabaseError> {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        self.ok = true;

        let mut tx = self.tx.lock().await;
        unimplemented!();
        Ok(())
    }

    async fn set<K, A>(&mut self, key: K, args: A) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send,
    {
        info!("POSTGRES [START]: Inserting one row...");
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        let mut guarded_tx = self.tx.lock().await;
        let tx = guarded_tx.as_mut().unwrap();
        let key = key.into();

        unimplemented!();
        Ok(())
    }

    async fn multi_set<K, A>(&mut self, key: K, args: Vec<A>) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send,
    {
        info!("POSTGRES [START]: Batch inserting...");
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        let mut guarded_tx = self.tx.lock().await;
        let tx = guarded_tx.as_mut().unwrap();

        unimplemented!();

        Ok(())
    }

    async fn get_filtered<K, A, V>(
        &self,
        key: K,
        args: A,
        keywords: &[&'static str],
    ) -> Result<Vec<V>, DatabaseError>
    where
        A: Into<Arg> + Send,
        K: Into<Key> + Send,
        V: FromPostgresRow,
    {
        info!("POSTGRES [START]: Querying...");
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        unimplemented!();
        // Ok(result)
    }
}

fn to_pg_prams(params: Vec<SuperValue>) -> Vec<Box<(dyn ToSql + Send + Sync + 'static)>> {
    let mut pg_params: Vec<Box<(dyn ToSql + Send + Sync + 'static)>> = vec![];
    for item in params {
        match item {
            SuperValue::String(v) => pg_params.push(Box::new(v)),
            SuperValue::Integer(v) => pg_params.push(Box::new(v)),
            SuperValue::BigInteger(v) => pg_params.push(Box::new(v)),
            SuperValue::SmallInteger(v) => pg_params.push(Box::new(v)),
            SuperValue::Char(v) => pg_params.push(Box::new(v)),
            _ => unimplemented!(),
        };
    }

    pg_params
}
