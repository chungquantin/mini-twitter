use crate::{
    constants::{get_sql_script, GLOBAL_SQL_SCRIPTS},
    errors::DatabaseError,
    models::{Document, SQLEvent},
    storage::config::StorageConfig,
    storage::sql::RDBAdapter,
    storage::DatabaseVariant,
};
use postgres::{Client, NoTls};

pub struct PostgresAdapter {
    pub client: Option<Box<postgres::Client>>,
}

impl Default for PostgresAdapter {
    fn default() -> Self {
        PostgresAdapter { client: None }
    }
}

impl RDBAdapter for PostgresAdapter {
    type ArgumentType = &'static (dyn postgres::types::ToSql + Sync);

    fn connect(&mut self, config: StorageConfig) -> Result<(), DatabaseError> {
        let connection_str: &'static str = config.uri;
        // "postgresql://chungquantin:password@localhost:5433/postgres";
        let client = Client::connect(connection_str, NoTls)?;
        self.client = Some(Box::new(client));

        Ok(())
    }

    fn create(&mut self, doc: Document, args: &[Self::ArgumentType]) -> Result<(), DatabaseError> {
        let client: &mut Client = self.client()?;
        let script = get_sql_script(doc, SQLEvent::Insert);
        client.execute(&script, args).unwrap();

        Ok(())
    }

    fn get_all(&mut self, doc: Document) -> Result<(), DatabaseError> {
        let client: &mut Client = self.client()?;
        let script = get_sql_script(doc, SQLEvent::Select);
        client.execute(&script, &[]).unwrap();

        Ok(())
    }
}

impl PostgresAdapter {
    pub fn client(&mut self) -> Result<&mut Client, DatabaseError> {
        if let Some(c) = &mut self.client {
            return Ok(c);
        } else {
            return Err(DatabaseError::Database(
                DatabaseVariant::Postgres,
                "Postgres database is not connected successfully".to_string(),
            ));
        }
    }
}
