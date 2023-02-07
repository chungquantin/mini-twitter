use path_absolutize::*;
use std::{
    env::{self},
    path::Path,
};

pub fn read_file_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let abs_filepath = &get_absolute_path(path);
    let data = std::fs::read_to_string(abs_filepath)?;
    Ok(data)
}

pub fn get_absolute_path(path: &str) -> String {
    let p = Path::new(path);
    let cwd = env::current_dir().unwrap();

    p.absolutize_from(&cwd)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
