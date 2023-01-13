use crate::{
    errors::DatabaseError, models::Document, sql::GLOBAL_SQL_SCRIPTS,
    storage::config::StorageConfig, storage::sql::RDBAdapter, storage::DatabaseVariant,
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
        let doc_name: String = String::from(doc.clone());
        let script: &String = GLOBAL_SQL_SCRIPTS.get(doc_name.as_str()).unwrap();

        client.execute(script, args).unwrap();

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
