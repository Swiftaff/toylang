use std::env;
use std::process;

use toylang::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = config.run(&args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
