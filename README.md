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
= a 123
= b 123.45
= c a
= d "string"

// basic arithmetic
= e + 1 2
= f - 5.4 3.2
END
```

</td><td>

```rust
fn main() {
    // single line comments
    let a = 123;
    let b = 123.45;
    let c = a;
    let d = "string";

    // basic arithmetic
    let e = 1 + 2;
    let f = 5.4 - 3.2;
}
```

</td></tr></table>
