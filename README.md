# TOYLANG

A toy language which compiles to rust for fun and experimentation.

## Build

Builds the `'/src/main.rs` into an executable at `/target/debug/toylang.exe` or `/target/release/toylang.exe`

```
cargo build
```

or

```
cargo build --release
```

## Usage

Create a file e.g. `test.toy` containing the toy language in the same directory (or elsewhere). Pass the filepath to the exe to compile it:

```
toylang test.toy
```

or

```
toylang.exe ../../somewhere/else/test.toy
```

Compile errors will appear in the console.

```
TOYLANG COMPILE ERROR:
----------
= monkey monkeys
         ^^^^^^^ is not a valid expression: must be either an: integer, e.g. 12345, float, e.g. 123.45, existing constant, e.g. x, string, e.g. "string", function, e.g. + 1 2
----------
= monkey monkeys
^ No valid expression was found
----------
```

Or on success a compiled file will be saved.
By default we assume the toylang.exe is in the `/target/debug` or /`target/release` directory, so it will save the output into `../../src/bin/output.rs`

You can then build THAT file with cargo as needed, i.e. add this to the "Cargo.toml"

```
[[bin]]
name = "output"
```

Then compile and run the output file

```
cargo run --bin output
```

or

```
cargo run --release --bin output
```

And your final compiled `output.exe` will be run from `/target/debug` or `/target/release`

## Toy language Syntax Examples

### Basic syntax

First line must be RUN

Anything in between as per examples below

Last line must be END

### Variable assignment

<table><tr><th>Toy</th><th>Rust</th></tr><tr><td>

```
RUN
// single line comments
= integer 123
= float 123.45
= reference a
= string "string"

// basic arithmetic
= addition + 1 2
= subtraction - 5.4 3.2
= multiplication * 3 4
= division / 1.0 2.0
= modulus % 42 3.14
END
```

</td><td>

```rust
fn main() {
    // single line comments
    let an_integer = 123;
    let a_float = 123.45;
    let a_reference = a;
    let a_string = "string";

    // basic arithmetic
    let addition = 1 + 2;
    let subtraction = 5.4 - 3.2;
    let multiplication = 3 * 4;
    let division = 1.0 / 2.0;
    let modulus = 42 % 3.14;
}
```

</td></tr></table>
