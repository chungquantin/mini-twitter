use once_cell::sync::Lazy;
use postgres::{Client, Error as PostgresError, NoTls};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FormatResult};
use thiserror::Error;

type UnixTimestamp = i64;
type Identifier = i64;

#[derive(Clone)]
pub enum Document {
    User,
    Tweet,
    Follows,
}

impl From<Document> for String {
    fn from(doc: Document) -> String {
        match doc {
            Document::User => "USER".to_string(),
            Document::Tweet => "Twitter".to_string(),
            Document::Follows => "Follows".to_string(),
        }
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", String::from(self.clone()))
    }
}

static GLOBAL_SQL_SCRIPTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut scripts = HashMap::new();
    let user_doc = Document::User;
    scripts.insert(
        format!("[{}]CREATE", user_doc),
        "Query goes here".to_string(),
    );
    scripts.insert(format!("[{}]GET_ONE", user_doc), "".to_string());
    scripts
});

pub struct PostgresAdapter {
    pub client: Option<Box<postgres::Client>>,
}

impl Default for PostgresAdapter {
    fn default() -> Self {
        PostgresAdapter { client: None }
    }
}

impl PostgresAdapter {
    pub fn connect(&mut self) -> Result<(), postgres::Error> {
        let connection_str: &'static str =
            "postgresql://chungquantin:password@localhost:5433/postgres";
        let client = Client::connect(connection_str, NoTls)?;
        self.client = Some(Box::new(client));

        Ok(())
    }

    pub fn client(&mut self) -> &mut Client {
        if let Some(c) = &mut self.client {
            return c.as_mut();
        } else {
            panic!("Postgres Database is not executed successfully");
        }
    }

    pub fn create(&mut self, doc: Document) {
        let client: &mut Client = self.client();
        let doc_name: String = String::from(doc.clone());
        let script: &String = GLOBAL_SQL_SCRIPTS.get(doc_name.as_str()).unwrap();
        client.execute(script, &[]).unwrap();
    }
}

#[derive(Debug)]
pub enum DatastoreVariant {
    Postgres,
}

impl Display for DatastoreVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let datastore = format!(
            "{}",
            match self {
                DatastoreVariant::Postgres => "POSTGRES".to_string(),
            }
        );
        write!(f, "{}", datastore)
    }
}

#[derive(Error, Debug)]
pub enum DatastoreError {
    #[error("[`{0}`] Datastore error: `{1}`")]
    Custom(DatastoreVariant, String),
    #[error("unknown data store error")]
    Unknown,
}

impl From<PostgresError> for DatastoreError {
    fn from(err: PostgresError) -> DatastoreError {
        DatastoreError::Unknown
    }
}

/// Active record User model
/// TODO: Using Postgres Types to convert to a valid postgres type
struct User {
    pub user_id: Identifier,
    pub name: String,
    pub user_ts: UnixTimestamp,
}

struct UserRepository {}

impl UserRepository {
    pub fn create_user(user: User) {
        // Code to create user in a database
    }
}

/// Active record Tweet model
struct Tweet {
    pub tweet_id: Identifier,
    user_id: Identifier,
    pub tweet_ts: UnixTimestamp,
    pub tweet_text: String,
}

impl Tweet {
    pub fn author(&self) -> Identifier {
        self.user_id
    }
}
/// Active record Follows model
struct Follows {
    user_id: Identifier,
    follows_id: Identifier,
}

impl Follows {
    pub fn from(&self) -> Identifier {
        self.user_id
    }

    pub fn to(&self) -> Identifier {
        self.follows_id
    }
}

fn main() {}
