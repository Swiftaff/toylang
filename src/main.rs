use std::env;
use std::process;

use toylang::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut compiler = Compiler::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = compiler.run(&args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
