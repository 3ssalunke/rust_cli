use std::env;
use std::process;
use grape::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("{} {}", config.query, config.filename);

    if let Err(e) = grape::run(config){
        eprintln!("Appliation Error: {}", e);
        process::exit(1)
    }
}
