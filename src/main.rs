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

    let url: String = process::getUrl(args[1].clone());

    println!("{}", url);

    if url == "-1" {
        println!("This URL does not exist, consider adding it with '-n'");
    }
       
    let mut command = process::getCommand("-f".to_string(), url);

    println!("Command: {:?}", command);

    match command.spawn() {
        Ok(_) => println!("Launched successfully!"),
        Err(e) => eprintln!("Failed to launch: {}", e),
    }


}
