use std::process::Command;
use std::collections::HashMap;
use serde_json::Value;
use crate::fileOps;


pub fn getUrl(name: String) -> String {
    
    let jsonObj : Value = getJsonObj();

    //get the value from the key as a string
    let url = match &jsonObj[name.clone()].as_str() {
        Some(url) => url.to_string(),
        None => {
            println!("Couldn't find url to entered key {}", name);
            return "-1".to_string();
        }
    };

    return url.to_string();

}

fn getJsonObj() -> Value {

    let json = fileOps::readJson();

    if json == "-1"{
        return serde_json::Value::String((-1).to_string());
    }

    return serde_json::from_str(&json).unwrap();

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

pub fn printAllSites() {

    let jsonObj = getJsonObj();

    for (key, value) in jsonObj.as_object().unwrap(){
        println!("Key: {} Url: {}", key, value);
    }

}

pub fn editJson(op: String, action: Vec<String>) -> bool {

   if !checkOp(op.clone()) {
        return false;
   }

   let mut jsonObj = getJsonObj();

   if jsonObj == "-1" {
       return false;
   }

   if op == "-n" {

       if let Some(obj) = jsonObj.as_object_mut() {
           obj.insert(
               action[0].clone(),
               serde_json::Value::String(action[1].clone()),
           );

       }

        fileOps::writeJson(jsonObj);

        return true;

   }else if op == "-d" {

        if let Some(obj) = jsonObj.as_object_mut() {
            obj.remove(&action[0]);
        }

        fileOps::writeJson(jsonObj);

        return true;

   }

   return false;

}

fn checkOp(op: String) -> bool {

    match op.as_str() {
        "-n" => { return true; },
        "-d" => { return true; },
        _    => { return false; }
    }

}

