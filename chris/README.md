# Christopher's solution for the 2025 Advent of Code challenge

For this year's challenge, I'm learning [Rust](https://rust-lang.org/).
So you need to have that installed to compile and run my solutions.
This README gives a quick overview of how to do that.

## Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### VS Code extensions and settings

I use the following extensions for Rust:

* [rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) for general language support
* [vadimcn.vscode-lldb](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) to enable debugging

The relevant user settings that I changed are the following:

```json
{
    "rust-analyzer.check.command": "clippy",
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true,
    },
}
```

## Compiling and running an existing solution

Each exercise will be a separate [Cargo](https://doc.rust-lang.org/cargo/) package.
As such, the easiest way to run the example is the following:

```bash
cd chris/dayXX
cargo run
```

## Creating a new exercise

```bash
cd chris
# --bin: project is a binary, not a library
# --vcs none: don't create a new git repo for this project
cargo new dayXX --bin --vcs none
```

## Need for speed

If you want to compare the speed of solutions, you might want to compile the solutions with compiler optimizations enabled:

```bash
cd chris/dayXX
cargo build --release
target/release/dayXX
```
