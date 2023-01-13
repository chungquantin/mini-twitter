pub struct StorageConfig {
    pub uri: &'static str,
}

impl StorageConfig {
    pub fn new(uri: &'static str) -> StorageConfig {
        StorageConfig { uri }
    }
}
