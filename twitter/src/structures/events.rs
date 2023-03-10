use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Clone)]
pub enum SQLEvent {
    CreateTable(String),
    CreateIndices,
    Insert,
    BatchInsert,
    Select(&'static str),
    Reset,
}

impl Display for SQLEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<SQLEvent> for String {
    fn from(event: SQLEvent) -> String {
        let event_str: String = match event {
            SQLEvent::Insert => "INSERT".to_string(),
            SQLEvent::Select(tag) => format!("SELECT_{}", tag).to_string(),
            SQLEvent::CreateTable(name) => format!("CREATE_TABLE_{}", name).to_string(),
            SQLEvent::Reset => "RESET".to_string(),
            SQLEvent::BatchInsert => "BATCH_INSERT".to_string(),
            SQLEvent::CreateIndices => "CREATE_INDICES".to_string(),
        };
        event_str
    }
}
