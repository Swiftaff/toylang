//#![windows_subsystem = "windows"]
//needed for debug_window.rs

extern crate toylang_macros;

use clap::Parser;
use toylang::compiler_runner;
use toylang::debug_window_derive;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// input filepath relative to the toylang executable
    #[arg(short, long)]
    input: String,

    /// optional - if true, the input is expected to be the raw toylang code, encoded as base64, instead of a filename. Primarily for VS Code Extension to use
    #[arg(short, long)]
    code: bool,

    /// optional - turn debugging information on
    #[arg(short, long)]
    debug: bool,

    /// optional - output directory. Default is current directory
    #[arg(short, long)]
    output: Option<String>,

    /// optional - nosave flag. Avoid saving output, useful if compiling an in progress toylang file causes an invalid rust output file, which then won't allow compilation next time. Default is false, i.e. it will save
    #[arg(short, long)]
    nosave: bool,

    /// optional - lines of tokens flag. If true it will print the "lines of tokens" containing positional info as JSON to stdout. Experimental for use with Toylang VS Code extension. Default is false.
    #[arg(short, long)]
    tokens: bool,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
    let code = cli.code;
    let debug = cli.debug;
    let output = cli.output;
    let nosave = cli.nosave;
    let tokens = cli.tokens;

    if debug {
        debug_window_derive::run(input, debug, output);
    } else {
        compiler_runner::main(input, debug, output, nosave, tokens, code);
    }
}
