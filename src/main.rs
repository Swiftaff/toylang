use std::process;

use clap::Parser;
use toylang::Compiler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// input filepath relative to the toylang executable
    #[arg(short, long)]
    input: String,

    /// optional - turn debugging information on
    #[arg(short, long)]
    debug: bool,

    /// optional - output directory. Default is current directory
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
    let debug = cli.debug;
    let output = cli.output;

    let mut compiler = Compiler::new(input, debug, output).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = compiler.run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
