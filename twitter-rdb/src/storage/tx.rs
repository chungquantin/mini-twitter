#[cfg(feature = "rdb_postgres")]
use super::PostgresTransaction;

#[allow(clippy::large_enum_variant)]
pub(super) enum Inner {
    #[cfg(feature = "rdb_postgres")]
    Postgres(PostgresTransaction),
}

pub struct Transaction {
    pub(super) inner: Inner,
}

impl_global_transaction!(
    Postgres; feat "rdb_postgres"
);
