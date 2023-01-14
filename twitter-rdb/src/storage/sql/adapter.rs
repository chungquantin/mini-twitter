use crate::errors::DatabaseError;
use crate::models::Document;
use crate::storage::config::StorageConfig;

/// Adapter trait for relation database
pub trait RDBAdapter {
    type ArgumentType;

    fn connect(&mut self, config: StorageConfig) -> Result<(), DatabaseError>;

    fn create(&mut self, doc: Document, args: &[Self::ArgumentType]) -> Result<(), DatabaseError>;

    fn get_all(&mut self, doc: Document) -> Result<(), DatabaseError>;
}
