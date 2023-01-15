use crate::models::{Document, SQLEvent};
use crate::utils::read_file_string;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn load_script(script: &'static str) -> String {
    let query = read_file_string(&format!("../queries/{}.sql", script).to_string()).unwrap();
    query
}

pub fn scriptify(doc: Document, event: SQLEvent) -> String {
    let script_name = format!("[{}][{}]", doc, event);
    script_name
}

pub fn get_sql_script(doc: Document, method: SQLEvent) -> String {
    let doc_name: String = String::from(doc.clone());
    let method_name: String = String::from(method.clone());
    let script_name = format!("{}:{}", doc_name, method_name);
    let script: &String = GLOBAL_SQL_SCRIPTS.get(&script_name).unwrap();

    script.clone()
}

pub static GLOBAL_SQL_SCRIPTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut scripts = HashMap::new();
    scripts.insert(
        scriptify(Document::GENERAL, SQLEvent::CreateTable),
        load_script("create_table"),
    );
    scripts.insert(
        scriptify(Document::Tweets, SQLEvent::Insert),
        load_script("insert_tweet"),
    );
    scripts.insert(
        scriptify(Document::Tweets, SQLEvent::SelectWhere),
        load_script("select_user_tweets"),
    );
    scripts
});
