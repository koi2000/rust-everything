use std::env;
use std::fs;
struct Config{
    query:String,
    filename:String,
}

fn parse_config(args:&[String]) -> Config{
    let query = args[1].clone();
    let filename = args[2].clone();

    let contents = fs::read_to_string(config.filename)
    .expext("Something went wrong reading the file");
}

fn main() {
    let args:Vec<String> = env.args.collect();

    let config = parse_config(&args);
    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);
    println!("{:?}",args);
}