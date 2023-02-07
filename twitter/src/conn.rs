use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::storage::DatabaseVariant;

type ConnectionMap = HashMap<DatabaseVariant, &'static str>;
pub static DATABASE_CONNECTIONS: Lazy<ConnectionMap> = Lazy::new(|| {
    HashMap::from([
        (
            DatabaseVariant::Postgres,
            "user=postgres host=localhost port=5433",
        ),
        (DatabaseVariant::Redis, "redis://127.0.0.1:6379"),
    ])
});
