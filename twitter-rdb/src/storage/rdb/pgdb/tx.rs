use async_trait::async_trait;
use tokio_postgres::types::ToSql;

use crate::{
    constants::get_sql_script,
    errors::DatabaseError,
    misc::{Arg, Key, Val},
    models::{DBTransaction, SQLEvent, SimpleTransaction},
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

        // Mark this transaction as done
        self.ok = true;

        let mut tx = self.tx.lock().await;
        match tx.take() {
            Some(tx) => tx.rollback().await?,
            None => unreachable!(),
        }

        Ok(())
    }

    async fn commit(&mut self) -> Result<(), DatabaseError> {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        // Check to see if transaction is writable
        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        // Mark this transaction as done
        self.ok = true;

        let mut tx = self.tx.lock().await;
        match tx.take() {
            Some(tx) => tx.commit().await?,
            None => unreachable!(),
        }

        Ok(())
    }

    async fn set<K, A>(&mut self, key: K, args: A) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send,
    {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        // Check to see if transaction is writable
        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        let mut guarded_tx = self.tx.lock().await;
        let tx = guarded_tx.as_mut().unwrap();
        let key = key.into();

        let mut casted_arguments: Vec<&(dyn ToSql + Sync)> = vec![];

        // for arg in args.into().iter() {
        //     let argument: &(dyn ToSql + Sync) = &arg.to_string();
        //     casted_arguments.push(argument);
        // }

        tx.execute(&get_sql_script(key, SQLEvent::Insert), &casted_arguments)
            .await?;

        Ok(())
    }

    async fn get_filtered<K, A>(&self, key: K, args: A) -> Result<Val, DatabaseError>
    where
        A: Into<Arg> + Send,
        K: Into<Key> + Send,
    {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        let guarded_tx = self.tx.lock().await;
        let tx = guarded_tx.as_ref().unwrap();
        let (key, args) = (key.into(), args.into());

        Ok(vec![])
    }
}
