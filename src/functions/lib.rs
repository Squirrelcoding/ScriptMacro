use std::collections::HashMap;
use std::env;
use std::fs::{read_to_string, write};

use serde_json::{json, Value};

pub fn get_macros() -> HashMap<String, String> {
    let path = format!("{}/.mcrmng/macros.json", env::var("HOME").unwrap());
    let mut hashmap: HashMap<String, String> = HashMap::new();
    let macros: Value = serde_json::from_str(&read_to_string(path).unwrap()).unwrap();
    for (k, v) in macros.as_object().unwrap().to_owned() {
        hashmap.insert(k, v.as_str().unwrap().to_string());
    }
    hashmap
}

pub fn write_back(hm: HashMap<String, String>) {
    let json = json!(hm).to_string();
    let path = format!("{}/.mcrmng/macros.json", env::var("HOME").unwrap());
    write(path, json).unwrap();
}
