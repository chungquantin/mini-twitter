use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::models::Document;

pub static GLOBAL_SQL_SCRIPTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut scripts = HashMap::new();
    let user_doc = Document::User;
    scripts.insert(
        format!("[{}]CREATE", user_doc),
        "Query goes here".to_string(),
    );
    scripts.insert(format!("[{}]GET_ONE", user_doc), "".to_string());
    scripts
});
