use std::fs;
use crate::config;

pub fn readJson() -> String {
    let jsonStr = fs::read_to_string(config::getFilePath()).expect("-1");

    return jsonStr;

}
