use rustc_serialize::json;
use std::fs;
const PATH: &str = "randoms.json";

pub fn create() -> Result<(Vec<i32>), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(PATH)?;
    let array: Vec<i32> = json::decode(&content).unwrap();
    Ok(array)
}
