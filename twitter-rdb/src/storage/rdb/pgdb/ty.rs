extern crate tokio_postgres;

use std::cell::Cell;

use tokio_postgres::{Client, Transaction};

use crate::structures::DBTransaction;

pub type DBType = Box<Cell<Client>>;
pub type TxType = Transaction<'static>;
pub type PostgresTransaction = DBTransaction<TxType>;
