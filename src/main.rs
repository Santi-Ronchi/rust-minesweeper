use std::process;

fn main() {
    if let Err(e) = guessing_game::run() {
        println!("app error: {}", e);
        process::exit(1);
    }
}
