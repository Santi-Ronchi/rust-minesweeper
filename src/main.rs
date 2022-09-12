use guessing_game::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = guessing_game::run(config) {
        println!("app error: {}", e);
        process::exit(1);
    }
}
