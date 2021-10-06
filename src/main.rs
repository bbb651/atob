use std::{env, process};

use atob::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = atob::run(config) {
        eprintln!("Error running code: {}", err);
        process::exit(1);
    } 
}
