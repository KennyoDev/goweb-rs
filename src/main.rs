use std::env;
use std::process::Command;
mod config;
mod fileOps;
mod process;


fn main() {

    //get user args
    let args: Vec<String> = env::args().collect();

    //check if args is not empty
    if args.len() <= 1 {
        println!("Please enter arguments!");
        return;
    }

    //handle everything other than opening a site
    match args[1].as_str() {
        "-a" => { return process::printAllSites(); },

        "-n" => { 
            let arr = vec!{args[2].clone(), args[3].clone()};
            let resp = process::editJson(args[1].to_string().clone(), arr.clone()); 

            if !resp {


            println!("Someting went wrong updating the JSON");
                
            }else {
                println!("Updated the JSON");
            }

            return;

        },

        "-d" => {

            let arr = vec!{args[2].clone()};
            let resp = process::editJson(args[1].to_string().clone(), arr.clone());

            if !resp {
                println!("Someting went wrong updating the JSON");
            }else {
                println!("Updated the JSON");
            }

            return;

        }
        _ => { }
    }

    //logic to open website starts here

    let mut getUrlIndex: usize = 1;

    let mut browserNameCMD = process::getBrowserName(args[1].clone()).to_string();

    //the standard is firefox
    let mut browserKey: String = "-f".to_string();

    //when it doesnt return an error, then the browser is specified
    if browserNameCMD != "-1"{
        browserKey = args[1].clone();
        getUrlIndex = 2;
    }

    let url: String = process::getUrl(args[getUrlIndex].clone());

    println!("{}", url);

    if url == "-1" {
        println!("This URL does not exist, consider adding it with '-n'");
        return;
    }
       
    let mut command = process::getCommand(browserKey, url);

    println!("Command: {:?}", command);

    match command.spawn() {
        Ok(_) => println!("Launched successfully!"),
        Err(e) => eprintln!("Failed to launch: {}", e),
    }


}

