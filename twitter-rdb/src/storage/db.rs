use crate::errors::DatabaseError;
use crate::models::ImplDatabase;

#[cfg(feature = "rdb_postgres")]
use super::PostgresAdapter;
use super::Transaction;

pub struct DatabaseRef<'a> {
    pub db: &'a mut Database,
}

impl<'a> DatabaseRef<'a> {
    pub fn new(db: &'a mut Database) -> Self {
        DatabaseRef { db }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum Inner {
    #[cfg(feature = "rdb_postgres")]
    Postgres(PostgresAdapter),
}

pub enum DatabaseVariant {
    Postgres,
}

pub struct Database {
    pub inner: Inner,
}

impl Database {
    pub fn new(name: DatabaseVariant, connection_str: &str) -> Database {
        match connection_str {
            #[cfg(feature = "rdb_postgres")]
            s if matches!(name, DatabaseVariant::Postgres) => {
                let db = PostgresAdapter::new(s).unwrap();

                Database {
                    inner: Inner::Postgres(db),
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn borrow(&mut self) -> DatabaseRef {
        DatabaseRef::new(self)
    }

    pub fn connection(&self) -> &str {
        macro_rules! impl_transaction_method {
			($($x: ident feat $f: expr),*) => {
				match &self.inner {
					$(
						#[cfg(feature = $f)]
						Inner::$x(v) => {
							v.connection()
						}
					)*
				}
			};
		}
        impl_transaction_method!(
            Postgres feat "rdb_postgres"
        )
    }

    pub fn transaction(&mut self, write: bool) -> Result<Transaction, DatabaseError> {
        macro_rules! impl_transaction_method {
			($($x: ident feat $f: expr),*) => {
				match &mut self.inner {
					$(
						#[cfg(feature = $f)]
						Inner::$x(v) => {
							let tx = v.transaction(write)?;
							Ok(Transaction {
								inner: super::tx::Inner::$x(tx),
							})
						}
					)*
				}
			};
		}
        impl_transaction_method!(
            Postgres feat "rdb_postgres"
        )
    }
}
