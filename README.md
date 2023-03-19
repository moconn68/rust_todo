# rust_todo: A Simple Rust CLI To-Do List Tool 📝

## About ℹ️

This is a simple CLI application that acts as a To-Do list manager. I created this as my first Rust project in order to learn the basics of the language as well as work through best-practices such as project structure and testing.

## Features 🔔
- Add items to a to-do list which can be scrolled through and selected to toggle completion status
- Option to clear all items marked as complete
- Tasks are automatically saved to a local database for reference in subsequent progrma executions

## Usage 🛠️

### Pre-requisites 💾

In order to run this project, you will need the Rust programming language installed on your machine, which should include Cargo. If you need to install this software, you can do so here: https://www.rust-lang.org/tools/install

### Installation ⚙️

1. Download this project via Git: `git clone https://github.com/moconn68/rust_todo.git`
2. In the root directory, execute the command `cargo run` in your terminal.

- Alternatively, if you'd like to build a standalone executable to run anywhere without building with
  Cargo each time, you can run `cargo build --release` which should generate an executable at `src/target/release/rust_todo`.

3. That's it! Use the menu options in the CLI to view, add, toggle completion status, and clear completed tasks, or to quit the program. Enjoy!

## Software used 👨‍💻

This project leverages the Rust programming language including it's robust toolchain. In addition, the following open-source dependencies are utilized:
- [`dialoguer`](https://crates.io/crates/dialoguer): provides a simple API for CLI application development
- [`sqlite`](https://crates.io/crates/sqlite): Rust implementation of the sqlite3 file-based SQL database
