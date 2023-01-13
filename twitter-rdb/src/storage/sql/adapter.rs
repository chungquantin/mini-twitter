use crate::storage::config::StorageConfig;
use crate::models::Document;
use crate::errors::DatabaseError;

/// Adapter trait for relation database
pub trait RDBAdapter {
    type ArgumentType;

    fn connect(&mut self, config: StorageConfig) -> Result<(), DatabaseError>;

    fn create(&mut self, doc: Document, args: &[Self::ArgumentType]) -> Result<(), DatabaseError>;
}
