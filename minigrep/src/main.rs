use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config){
        
    }
}

