use std::time::SystemTime;

use tokio_postgres::Row;

use crate::errors::DatabaseError;

/// ## SuperValue
/// Allows to convert between each database specific types
/// ### SuperValue -> Postgres Types
/// ---
/// | Rust type   | Postgres type                        |
/// | ----------- | ------------------------------------ |
/// | bool        | BOOL                                 |
/// | i8          | "char"                               |
/// | i16         | SMALLINT, SMALLSERIAL                |
/// | i32         | INT, SERIAL                          |
/// | u32         | OID                                  |
/// | i64         | BIGINT, BIGSERIAL                    |
/// | f32         | REAL                                 |
/// | f64         | DOUBLE PRECISION                     |
/// | &str/String | VARCHAR, CHAR(n), TEXT, CITEXT, NAME |
/// |             | LTREE, LQUERY, LTXTQUERY             |
/// ---
pub enum SuperValue {
    Bool(bool),
    Char(i8),
    SmallInteger(i16),
    Integer(i32),
    OID(u32),
    Real(f32),
    Double(f64),
    BigInteger(i64),
    String(String),
    Timestamp(SystemTime),
}

#[doc(hidden)]
pub trait BorrowFromSuperValue: Sized {
    fn from_super_value<'a>(v: &'a SuperValue) -> Result<&'a Self, DatabaseError>;
}

pub trait FromPostgresRow {
    fn from_pg_row(r: Row) -> Self;
}
