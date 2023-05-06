/*!
    ## Instructions for using toylang CLI

    ### -i --input
    Create a file e.g. `test.toy` containing the toy language in the same directory (or elsewhere).

    Pass the required input filepath using the input arg (-i or --input)

    ### -o --output
    Pass the optional output directory using the output arg (-o or --output). Otherwise by default `output.rs` is saved into the current directory.
    For development you may wish to output to `src\\bin` by convention to make use of `cargo run --bin output` below.

    ```bash
    toylang -i test.toy
    ```

    or

    ```bash
    toylang.exe --input ..\\..\\somewhere\\else\\test.toy --output src\\bin
    ```

    ## Compile errors
    Compile errors will appear in the console.

    ```bash
    TOYLANG COMPILE ERROR:
    ----------
    = monkey monkeys
            ^^^^^^^ is not a valid expression: must be either an: integer, e.g. 12345, float, e.g. 123.45, existing constant, e.g. x, string, e.g. "string", function, e.g. + 1 2
    ----------
    = monkey monkeys
    ^ No valid expression was found
    ----------
    ```

    ### Output file
    Or on success a compiled `output.rs` file will be saved to the output directory.

    ### Compiling the output file
    You can then build THAT file with cargo as needed, i.e. output to `src\\bin` and add this to the "Cargo.toml"

    ```cargo
    [[bin]]
    name = "output"
    ```

    Then compile and run the output file

    ```bash
    cargo run --bin output
    ```

    or

    ```bash
    cargo run --release --bin output
    ```

    And your final compiled `output.exe` will be saved to, and run from `\\target\\debug` or `\\target\\release`
*/

use crate::Compiler;
use std::process;

/// Only function for compiler_runner
pub fn main(input: String, debug: bool, output: Option<String>, nosave: bool) {
    let mut compiler = Compiler::new(input, debug, output, nosave).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = compiler.run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
