use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref FUNCTION_REGISTRY: Mutex<HashMap<String, fn() -> String>> = Mutex::new(HashMap::new());
}

pub fn get_registry() -> &'static Mutex<HashMap<String, fn() -> String>> {
    &FUNCTION_REGISTRY
}
