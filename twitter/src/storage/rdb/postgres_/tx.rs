use async_trait::async_trait;
use tokio_postgres::types::ToSql;

use crate::{
    constants::get_sql_script,
    errors::DatabaseError,
    misc::{Arg, Key},
    structures::{
        DBTransaction, FromPostgresRow, KeywordBucket, SQLEvent, SimpleTransaction, SuperValue,
    },
    utils::sss,
};

use super::ty::TxType;

type PostgresArgType<'a> = &'a (dyn ToSql + Sync);

#[async_trait]
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

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        self.ok = true;

        let mut tx = self.tx.lock().await;
        match tx.take() {
            Some(tx) => tx.commit().await?,
            None => unreachable!(),
        }
        Ok(())
    }

    async fn set<K, A>(
        &mut self,
        key: K,
        args: A,
        _keywords: KeywordBucket,
    ) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send,
    {
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

        Ok(())
    }

    async fn multi_set<K, A>(&mut self, key: K, args: Vec<A>) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send,
    {
        if self.closed() {
            return Err(DatabaseError::TxFinished);
        }

        if !self.writable {
            return Err(DatabaseError::TxReadonly);
        }

        let mut guarded_tx = self.tx.lock().await;
        let tx = guarded_tx.as_mut().unwrap();

        let mut batch_params: Vec<Box<dyn ToSql + Send + Sync>> = vec![];
        for arg_item in args {
            let mut pg_params = to_pg_prams(arg_item.into());
            batch_params.append(&mut pg_params);
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

        Ok(())
    }

    async fn get<K, A, V>(
        &self,
        key: K,
        args: A,
        keywords: KeywordBucket,
    ) -> Result<Vec<V>, DatabaseError>
    where
        A: Into<Arg> + Send,
        K: Into<Key> + Send,
        V: FromPostgresRow,
    {
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

        let script = sss(keywords.get("tag").unwrap());
        let rows = tx
            .query(
                &get_sql_script(key.clone(), SQLEvent::Select(script)),
                &pg_params_ref,
            )
            .await?;

        let mut result = vec![];
        for row in rows {
            let v = V::from_pg_row(row);
            result.push(v);
        }
        Ok(result)
    }
}

type PostgresReturnType = Box<(dyn ToSql + Send + Sync + 'static)>;
fn to_pg_prams(params: Vec<SuperValue>) -> Vec<PostgresReturnType> {
    let mut result: Vec<PostgresReturnType> = vec![];
    for item in params {
        macro_rules! param_convert {
            ($($SuperValueType: ident),*) => {
                match item {
                    $(
                        SuperValue::$SuperValueType(v) => result.push(
                            Box::new(v)
                        ),
                    )*
                    _ => unimplemented!()
                }
            };
        }
        param_convert!(String, Integer, BigInteger, SmallInteger, Char);
    }
    result
}
