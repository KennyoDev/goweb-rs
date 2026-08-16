use std::fs;
use serde_json::Value;
use crate::config;

pub fn readJson() -> String {
    let jsonStr = fs::read_to_string(config::getFilePath()).expect("-1");

    return jsonStr;

}

pub fn writeJson(json: Value) -> bool {

    let jsonStr = serde_json::to_string_pretty(&json).unwrap();

    fs::write(config::getFilePath(), jsonStr).unwrap();

    return true;

}
