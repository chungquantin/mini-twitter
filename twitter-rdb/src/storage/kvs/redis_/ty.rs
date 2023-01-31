extern crate redis;

use redis::{Client, Connection};

use crate::structures::DBTransaction;

pub type TxType = &'static mut Connection;
pub type DBType = Box<Client>;
pub type RedisTransaction = DBTransaction<TxType>;
