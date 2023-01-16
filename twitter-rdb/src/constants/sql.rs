use crate::structures::{Document, SQLEvent};
use crate::utils::read_file_string;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn load_script(script: &'static str) -> String {
    let path = &format!("./src/queries/{}.sql", script).to_string();
    let query = read_file_string(path).unwrap();
    query
}

pub fn scriptify(doc: Document, event: SQLEvent) -> String {
    let script_name = format!("{}:{}", doc, event);
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
    // General scripts
    scripts.insert(
        scriptify(Document::GENERAL, SQLEvent::Reset),
        load_script("reset"),
    );
    scripts.insert(
        scriptify(
            Document::GENERAL,
            SQLEvent::CreateTable("Tweets".to_string()),
        ),
        load_script("create_table_tweets"),
    );
    scripts.insert(
        scriptify(
            Document::GENERAL,
            SQLEvent::CreateTable("Follows".to_string()),
        ),
        load_script("create_table_follows"),
    );

    // Tweets script
    scripts.insert(
        scriptify(Document::Tweets, SQLEvent::Insert),
        load_script("insert_tweet"),
    );
    scripts.insert(
        scriptify(Document::Tweets, SQLEvent::BatchInsert),
        load_script("batch_insert_tweets"),
    );
    scripts.insert(
        scriptify(Document::Tweets, SQLEvent::Select("user_tweets")),
        load_script("select_user_tweets"),
    );
    // Follows script
    scripts.insert(
        scriptify(Document::Follows, SQLEvent::Select("user_random_followee")),
        load_script("select_random_followee"),
    );
    scripts.insert(
        scriptify(Document::Follows, SQLEvent::Insert),
        load_script("insert_follow"),
    );
    scripts.insert(
        scriptify(Document::Follows, SQLEvent::Select("user_followers")),
        load_script("select_user_followers"),
    );
    scripts.insert(
        scriptify(Document::Follows, SQLEvent::Select("user_followees")),
        load_script("select_user_followees"),
    );
    scripts
});
