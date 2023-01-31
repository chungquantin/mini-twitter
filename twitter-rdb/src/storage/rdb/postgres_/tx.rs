use async_trait::async_trait;
use log::info;
use tokio_postgres::types::ToSql;

use crate::{
    constants::get_sql_script,
    errors::DatabaseError,
    misc::{Arg, Key},
    structures::{DBTransaction, FromPostgresRow, SQLEvent, SimpleTransaction, SuperValue},
};

use super::ty::TxType;

type PostgresArgType<'a> = &'a (dyn ToSql + Sync);

#[async_trait(?Send)]
impl SimpleTransaction for DBTransaction<TxType> {
    fn closed(&self) -> bool {
        self.ok
    }

    async fn cancel(&mut self) -> Result<(), DatabaseError> {
        info!("POSTGRES [START]: Rolling back...");
        if self.ok {
            return Err(DatabaseError::TxFinished);
        }

        self.ok = true;

        let mut tx = self.tx.lock().await;
        match tx.take() {
            Some(tx) => tx.rollback().await?,
            None => unreachable!(),
        }
        info!("POSTGRES [END]: Rolled back...");
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), DatabaseError> {
        info!("POSTGRES [START]: Committing...");
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        self.ok = true;

        let mut tx = self.tx.lock().await;
        match tx.take() {
            Some(tx) => tx.commit().await?,
            None => unreachable!(),
        }
        info!("POSTGRES [END]: Committed");
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

        let pg_params = to_pg_prams(args.into());
        let pg_params_ref = pg_params
            .iter()
            .map(|x| -> PostgresArgType { x.as_ref() })
            .collect::<Vec<PostgresArgType>>();

        tx.execute(
            &get_sql_script(key.clone(), SQLEvent::Insert),
            &pg_params_ref,
        )
        .await?;

        info!(
            "POSTGRES [END]: Insert one row to table {:?} successfully",
            key
        );

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

        let mut batch_params: Vec<Box<dyn ToSql + Send + Sync>> = vec![];
        let mut len = 0;
        for arg_item in args {
            let mut pg_params = to_pg_prams(arg_item.into());
            batch_params.append(&mut pg_params);
            len += 1;
        }

        let pg_params_ref = batch_params
            .iter()
            .map(|x| -> PostgresArgType { x.as_ref() })
            .collect::<Vec<PostgresArgType>>();

        let k = key.into();
        tx.execute(
            &get_sql_script(k.clone(), SQLEvent::BatchInsert),
            &pg_params_ref,
        )
        .await?;

        info!("POSTGRES [END]: Insert {:?} rows", len);

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

        let guarded_tx = self.tx.lock().await;
        let tx = guarded_tx.as_ref().unwrap();
        let (key, args) = (key.into(), args.into());

        let pg_params = to_pg_prams(args.into());
        let pg_params_ref = pg_params
            .iter()
            .map(|x| -> PostgresArgType { x.as_ref() })
            .collect::<Vec<PostgresArgType>>();

        let rows = tx
            .query(
                &get_sql_script(key.clone(), SQLEvent::Select(keywords[0])),
                &pg_params_ref,
            )
            .await?;

        let mut result = vec![];
        for row in rows {
            let v = V::from_pg_row(row);
            result.push(v);
        }
        info!("POSTGRES [END]: Found {:?} items", result.len());
        Ok(result)
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
