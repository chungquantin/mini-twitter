use std::time::SystemTime;

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

macro_rules! impl_borrow_from_super_value {
    ($t:ty, $v:path) => {
        impl BorrowFromSuperValue for $t {
            fn from_super_value<'a>(v: &'a SuperValue) -> Result<&'a $t, DatabaseError> {
                match v {
                    $v(e) => Ok(e),
                    _ => Err(DatabaseError::TypeCastError(
                        stringify!($v).to_string(),
                        stringify!($t).to_string(),
                    )),
                }
            }
        }
    };
}

impl SuperValue {
    pub fn get<'a, T>(&'a self) -> Result<&'a T, DatabaseError>
    where
        T: BorrowFromSuperValue,
    {
        T::from_super_value(self)
    }
}

impl_borrow_from_super_value!(bool, SuperValue::Bool);
impl_borrow_from_super_value!(String, SuperValue::String);
impl_borrow_from_super_value!(i8, SuperValue::Char);
impl_borrow_from_super_value!(i16, SuperValue::SmallInteger);
impl_borrow_from_super_value!(i32, SuperValue::Integer);
impl_borrow_from_super_value!(i64, SuperValue::BigInteger);
impl_borrow_from_super_value!(SystemTime, SuperValue::Timestamp);

pub trait FromSuperValues {
    fn from_super_value(v: Vec<SuperValue>) -> Self;
}
