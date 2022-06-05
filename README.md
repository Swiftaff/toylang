# RUSTLANG

A toy language which compiles to rust for fun and experimentation.

## Build

Builds the executable at `/target/debug/rustlang.exe`

```
cargo build
```

## Usage

Create a file e.g. "test.y" containing the toy language in the same directory (or elsewhere). Pass the filepath to the exe to compile it:

```
rustlang.exe test.y
```

or

```
rustlang.exe ../../somewhere/else/test.y
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
x = 2
END
```

</td><td>

```rust
fn main() {
    let x = 2;
}
```

</td></tr></table>
