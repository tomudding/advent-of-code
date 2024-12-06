use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref FUNCTION_REGISTRY: Mutex<HashMap<String, fn() -> String>> = Mutex::new(HashMap::new());
}

pub fn get_registry() -> &'static Mutex<HashMap<String, fn() -> String>> {
    &FUNCTION_REGISTRY
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}