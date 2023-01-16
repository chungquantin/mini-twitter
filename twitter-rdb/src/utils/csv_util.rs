use super::get_absolute_path;
use csv::{ReaderBuilder, StringRecord};

pub fn load_from_csv(filepath: &'static str) -> Vec<StringRecord> {
    // Build the CSV reader and iterate over each record.
    let abs_path = get_absolute_path(filepath);
    let mut rdr = ReaderBuilder::new().from_path(abs_path).unwrap();
    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()
        .unwrap();

    records
}
