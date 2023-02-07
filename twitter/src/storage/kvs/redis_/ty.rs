extern crate redis;

use redis::{aio::Connection, Client};

use crate::structures::DBTransaction;

pub type TxType = Connection;
pub type DBType = Box<Client>;
pub type RedisTransaction = DBTransaction<TxType>;
