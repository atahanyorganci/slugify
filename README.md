# `slugify` CLI

Slugify is a simple command-line interface (CLI) tool written in Rust that generates slugs from provided strings.

- UTF-8 compatible `slugify` uses [`slug` crate](https://crates.io/crates/slug) under the hood to handle conversion.
- It uses Rust btw.

## Installation

To use Slugify, you need to have Rust installed. If you do not have Rust, you can install it from the [official website](https://www.rust-lang.org/).

Once Rust is installed, you can clone this repository and build the project:

```bash
git clone https://github.com/atahanyorganci/slugify.git
cd slugify
cargo build --release
```

The executable will be in the target/release directory. You can add this directory to your PATH to use slugify from anywhere on your system.

### `just` Users

[`Justfile`](./Justfile) contains commands to install `x86_64-unknown-linux-musl` target, build statically linked binary and install to local executable directory.

```sh
just toolchain # installs x86_64-unknown-linux-musl
just install # or just build and just install
```

## Usage

```bash
slugify [OPTIONS] <INPUT>
```

### Arguments

- `<INPUT>`: The input string to slugify. Use '-' to read from standard input.

### Options

- `-d`, `--delim <DELIM>`: Specify a custom delimiter (default is '-').
- `-p`, `--path`: Treat the input as a path, preserving the file extension.

### Examples

```bash
$ slugify "Hello World"
hello-world
$ slugify "Hello World" --delim _
hello_world
$ slugify "path/to/my file.txt" --path
path/to/my-file.txt
$ ls -1 | slugify --path -
cargo.lock
cargo.toml
justfile
license
readme.md
src
target
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
