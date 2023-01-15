use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Clone)]
pub enum SQLEvent {
    CreateTable,
    Insert,
    SelectWhere,
    SelectOne,
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
            SQLEvent::SelectWhere => "SELECT_WHERE".to_string(),
            SQLEvent::SelectOne => "SELECT_ONE".to_string(),
            SQLEvent::CreateTable => "CREATE_TABLE".to_string(),
        };
        event_str
    }
}
