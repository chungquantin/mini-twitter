use async_trait::async_trait;
use log::info;

use crate::{
    errors::DatabaseError,
    misc::{Arg, Key},
    structures::{DBTransaction, FromPostgresRow, KeywordBucket, SimpleTransaction, SuperValue},
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

        let mut guarded_tx = self.tx.lock().await;
        let conn = guarded_tx.as_mut().unwrap();

        let _: () = redis::cmd("DISCARD").query_async(conn).await?;
        // if &status != "OK" {
        //     return Err(DatabaseError::Tx(
        //         "Transaction is not discarded".to_string(),
        //     ));
        // }

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

        let mut guarded_tx = self.tx.lock().await;
        let conn = guarded_tx.as_mut().unwrap();
        let _: () = redis::cmd("EXEC").query_async(conn).await?;
        // if &status != "OK" {
        //     return Err(DatabaseError::Tx(
        //         "Transaction is not committed".to_string(),
        //     ));
        // }

        Ok(())
    }

    async fn set<K, A>(
        &mut self,
        key: K,
        args: A,
        keywords: KeywordBucket,
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
        let conn = guarded_tx.as_mut().unwrap();
        let key = key.into();

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

        unimplemented!();

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

        Ok(vec![])
    }
}

type RedisReturnType = Box<String>;
fn to_redis_params(params: Vec<SuperValue>) -> Vec<RedisReturnType> {
    let mut result: Vec<RedisReturnType> = vec![];

    for item in params {
        macro_rules! param_convert {
            ($($SuperValueType: ident),*) => {
                match item {
                    $(
                        SuperValue::$SuperValueType(v) => result.push(
                            Box::new(v.to_string())
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
