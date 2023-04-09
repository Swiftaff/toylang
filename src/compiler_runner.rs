use crate::Compiler;
use std::process;

pub fn main(input: String, debug: bool, output: Option<String>) {
    let mut compiler = Compiler::new(input, debug, output).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = compiler.run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
