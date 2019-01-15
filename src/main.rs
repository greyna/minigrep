use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    println!("");
    
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("\nProblem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("\nApplication error: {}", e);
        process::exit(1);
    }
}