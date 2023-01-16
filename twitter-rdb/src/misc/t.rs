use std::time::SystemTime;

use crate::models::{Document, SuperValue};

pub type UnixTimestamp = SystemTime;
pub type Identifier = i32;
pub type Key = Document;
pub type Val = Vec<String>;
pub type Arg = Vec<SuperValue>;
