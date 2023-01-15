use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Clone)]
pub enum Document {
    GENERAL,
    Tweets,
    Follows,
}

impl From<Document> for String {
    fn from(doc: Document) -> String {
        match doc {
            Document::GENERAL => "GENERAL".to_string(),
            Document::Tweets => "Tweets".to_string(),
            Document::Follows => "Follows".to_string(),
        }
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", String::from(self.clone()))
    }
}
