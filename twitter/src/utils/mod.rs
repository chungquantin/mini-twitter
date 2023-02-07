mod benchmark;
mod csv_util;
mod file_util;
mod log_util;

pub use benchmark::*;
pub use csv_util::*;
pub use file_util::*;
pub use log_util::*;

pub fn sss(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
