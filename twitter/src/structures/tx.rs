use crate::{
    errors::DatabaseError,
    misc::{Arg, Key},
};
use async_trait::async_trait;
use futures::lock::Mutex;
use std::sync::Arc;

use super::{FromPostgresRow, FromRedisValue, KeywordBucket};

pub struct DBTransaction<T>
where
    T: 'static,
{
    pub debug: bool,
    pub tx: Arc<Mutex<Option<T>>>,
    pub ok: bool,
    pub writable: bool,
    pub readable: bool,
}

impl<TxType> DBTransaction<TxType>
where
    TxType: 'static,
{
    pub fn new(tx: TxType, w: bool, debug: bool) -> Result<Self, DatabaseError> {
        Ok(DBTransaction {
            tx: Arc::new(Mutex::new(Some(tx))),
            debug,
            ok: false,
            writable: w,
            readable: true,
        })
    }
}

#[async_trait]
pub trait SimpleTransaction {
    // Check if closed
    fn closed(&self) -> bool;

    // Cancel a transaction
    async fn cancel(&mut self) -> Result<(), DatabaseError>;

    // Commit a transaction
    async fn commit(&mut self) -> Result<(), DatabaseError>;

    // // Check if a key exists
    // async fn exi<K: Into<Key> + Send>(&self, cf: CF, key: K) -> Result<bool, DatabaseError>;

    // /// Fetch a key from the database
    // async fn get<K: Into<Key> + Send>(&self, cf: CF, key: K) -> Result<Option<Val>, DatabaseError>;

    /// Insert or update a key in the database
    async fn set<K, A>(
        &mut self,
        key: K,
        args: A,
        keywords: KeywordBucket,
    ) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send;

    async fn multi_set<K, A>(&mut self, key: K, args: Vec<A>) -> Result<(), DatabaseError>
    where
        K: Into<Key> + Send,
        A: Into<Arg> + Send;

    async fn get<K, A, V>(
        &self,
        key: K,
        args: A,
        keywords: KeywordBucket,
    ) -> Result<Vec<V>, DatabaseError>
    where
        A: Into<Arg> + Send,
        K: Into<Key> + Send,
        V: FromPostgresRow + FromRedisValue + Send;

    // /// Insert a key if it doesn't exist in the database
    // async fn put<K: Into<Key> + Send, V: Into<Key> + Send>(
    //     &mut self,
    //     cf: CF,
    //     key: K,
    //     val: V,
    // ) -> Result<(), DatabaseError>;

    // /// Delete a key
    // async fn del<K: Into<Key> + Send>(&mut self, cf: CF, key: K) -> Result<(), DatabaseError>;
}
