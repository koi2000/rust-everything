use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    let contents = fs::read_to_string(&filename).expect("Something went wrong reading the file");
    Config { query, filename }
}

// fn run(config: Config) {
//     let contents =
//         fs::read_to_string(config.filename).expect("Something went wrong reading the file");

//     println!("With text:\n{}", contents);
// }

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}