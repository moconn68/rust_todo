# rust_todo: A Simple Rust CLI To-Do List Tool 📝

## About ℹ️

This is a simple CLI application that acts as a To-Do list manager. I created this as my first Rust project in order to learn the basics of the language as well as work through best-practices such as project structure and testing.

## Usage 🛠️

### Pre-requisites 💾

In order to run this project, you will need the Rust programming language installed on your machine, which should include Cargo. If you need to install this software, you can do so here: https://www.rust-lang.org/tools/install

### Installation ⚙️

1. Download this project via Git: `TODO`
2. In the root directory, execute the command `cargo run` in your terminal.

- Alternatively, if you'd like to build a standalone executable to run anywhere without building with
  Cargo each time, you can run `cargo build --release` which should generate an executable at `src/target/release/rust_todo`.

3. That's it! Use the menu options in the CLI to view, add, toggle completion status, and clear completed tasks, or to quit the program. Enjoy!

## Software used 👨‍💻

This project leverages the Rust programming language including it's robust toolchain. In addition, the open-source [`dialoguer`](https://crates.io/crates/dialoguer) crate is used to provide a simple API for CLI application development, which this project uses for the main selection list and accepting user input for creating new tasks.
