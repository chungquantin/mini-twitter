pub fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(filepath)?;
    Ok(data)
}
