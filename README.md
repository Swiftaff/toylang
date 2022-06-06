# TOYLANG

A toy language which compiles to rust for fun and experimentation.

## Build

Builds the executable at `/target/debug/toylang.exe`

```
cargo build
```

## Usage

Create a file e.g. "test.toy" containing the toy language in the same directory (or elsewhere). Pass the filepath to the exe to compile it:

```
toylang test.toy
```

or

```
toylang.exe ../../somewhere/else/test.toy
```

Compile errors will appear in the console.

Or on success a compiled file will be saved in the same directory as "output.rs".
You can then build that file with rust as needed.

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
}

```

</td></tr></table>
