#[cfg(feature = "rdb_postgres")]
use super::PostgresTransaction;
#[cfg(feature = "kvs_redis")]
use super::RedisTransaction;

#[allow(clippy::large_enum_variant)]
pub(super) enum Inner {
    #[cfg(feature = "rdb_postgres")]
    Postgres(PostgresTransaction),
    #[cfg(feature = "kvs_redis")]
    Redis(RedisTransaction),
}

pub struct Transaction {
    pub(super) inner: Inner,
}

impl_global_transaction!(
    Postgres; feat "rdb_postgres",
    Redis; feat "kvs_redis"
);
