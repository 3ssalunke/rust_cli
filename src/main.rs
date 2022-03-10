use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("{} {}", config.query, config.filename);

    if let Err(e) = run(config){
        println!("Appliation Error: {}", e);
        process::exit(1)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);

    Ok(())
}

struct Config{
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("Not Enough Arguments.")
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config{
            query,
            filename
        })
    }    
}