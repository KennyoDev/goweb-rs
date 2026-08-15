use std::process::Command;
use serde_json::Value;
use crate::fileOps;


pub fn getUrl(name: String) -> String {
    let json = fileOps::readJson();

    if json == "-1"{
        return (-1).to_string();
    }
    
    //Get the json obj from the file
    let jsonObj: Value = serde_json::from_str(&json).unwrap();

    //get the value from the key as a string
    let url = &jsonObj[name].as_str().unwrap().to_string();

    if url == "" {
        return (-1).to_string();
    }

    return url.to_string();

}

pub fn getCommand(browser: String, url: String) -> Command {

    let mut cmd = Command::new(getBrowserName(browser));
    cmd.arg(&url);

    return cmd;

}

pub fn getBrowserName(browser: String) -> String {

    match browser.as_str() {
        "-f" => return "firefox".to_string(),
        "-c" => return "chromium".to_string(),
        "-z" => return "zen-browser".to_string(),
        _ => "-1".to_string()
    }

}
