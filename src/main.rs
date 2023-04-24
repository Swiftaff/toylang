//#![windows_subsystem = "windows"]
//needed for debug_window.rs

use clap::Parser;
use toylang::compiler_runner;
use toylang::debug_window_derive;

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

    /// optional - nosave flag. Avoid saving output, useful if compiling an in progress toylang file causes an invalid rust output file, which then won't allow compilation next time. Default is false, i.e. it will save
    #[arg(short, long)]
    nosave: bool,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
    let debug = cli.debug;
    let output = cli.output;
    let nosave = cli.nosave;
    if debug {
        debug_window_derive::run(input, debug, output);
    } else {
        compiler_runner::main(input, debug, output, nosave);
    }
}
