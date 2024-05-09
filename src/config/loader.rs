use std::{collections::HashMap, sync::OnceLock};

use simple_config_parser::Config;

pub static CONFIG_MAP:OnceLock<HashMap<String,String>> = OnceLock::new();

pub fn load(name:&String) {
    let path = format!("{}.config",name);
    let f = Config::new().file(&path).expect(&format!("{}",&path));
    let mut map = HashMap::new();
    map.insert("device".to_string(), name.to_string());
    for arr in f.data {
        map.insert(arr[0].clone(), arr[1].clone());
    }
    CONFIG_MAP.set(map).unwrap();
}

pub fn get(key:&str) -> Option<String> { 
    let map = CONFIG_MAP.get().unwrap();

    match map.get(key) {
        Some(v) => Some(v.to_string()),
        None => None,
    }
}