use std::time::SystemTime;

use crate::structures::{Document, SuperValue};

pub type UnixTimestamp = SystemTime;
pub type Identifier = i32;
pub type Key = Document;
pub type Arg = Vec<SuperValue>;
