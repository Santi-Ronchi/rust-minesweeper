use std::env;
use std::process;
mod config;
use crate::config::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config.filename) {
        println!("app error: {}", e);
        process::exit(1);
    }
}
