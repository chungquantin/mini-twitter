use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Debug)]
pub enum DatabaseVariant {
    Postgres,
}

impl Display for DatabaseVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let datastore = format!(
            "{}",
            match self {
                DatabaseVariant::Postgres => "POSTGRES".to_string(),
            }
        );
        write!(f, "{}", datastore)
    }
}
