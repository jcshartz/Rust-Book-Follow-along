# Rust Programming Book

This repo is where I will go through the Rust programming language book and maybe a little bit farther here and there. 

This README serves as central location for my notes going through the book, in general on Rust, and where I will have links for further resources.

# General Notes

## Building from exact dependency versions

## Building offline

## Misc

# Book Notes by Chapter

I used `rustup doc --book` which opens up the Rust book in a browser window. This does not require an internet connection

## Introduction

- Cargo manages dependencies and is the main build tool 
- `rustfmt` is the formatting tool
- There's a Rust Language Server that I'm assuming is a general LSP for Rust. The book mentions IDEs specifically

## Chapter 1

Overview on how to install rust, a simple "Hello World", and how to use Cargo

### Projects 
- hello_world
  - located in ./projects/hello_world
- hello_cargo
  - located in ./projects/hello_cargo

- Book recommends installing from a curl script. Windows has a special method
- You also need a linker which can be xcode, gcc, clang. 
- `rustc --version`  should show a version number if installed correctly
  - If not, check the PATH
- `rustup update` to update rust
- `rustup self uninstall` to install
- `rustup doc` has the local documentation that you can view in a browser offline
- The book will use dependencies beyond the standard library, which you will need to download. To download ahead of time (for offline build) do the following
```bash
cargo new get-dependencies
cd get-dependencies 
cargo add rand@0.8.5 trpl@0.2.0
```
```
```
- Offline builds can be done using the `--offline` flag with all `cargo` commands covered in the book
- `main` function will always be the first code that runs in every executable Rust program. If there are parameters, it would go inside the parentheses.
- `rustc main.rs` compiles the rust program starting at `main.rs`
- Windows compilation would include a `main.exe` and a `main.pdb` instead of an executable `main` as in Linux and Mac
- `cargo --version` to get the version. If it doesn't work then cargo broke
- `cargo new hello_cargo` to create a new project named `hello_cargo`
- `cargo init` to initialize a cargo project in a folder that was already made
- `cargo build` to build a cargo project 
  - A `Cargo.lock` file will generate to lock in the exact versions of dependencies that the app uses
- `cargo run` to build then run the application
  - easier than `cargo build` then finding the binary
- `cargo check` to check the code and ensures it compiles without producing an executable. Faster than `cargo build` so helps with testing/debugging
- cargo will store the binary in target/debug directory
- `cargo build --release` produces a release build in `target/release`. This build will have optimizations to run faster but won't have debug symbols
  - benchmarking should be done with a release build though

## Chapter 2

Project chapter to build up a number-guessing game

Located in ./projects/guessing_game

- an associated function is a function that is implemented on a type 
  - For example, in`String::new()` the function `new` is directly tied to the type `String`
- The `&` on an argument indicates a reference
  - which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times
  - for example, `&mut guess`
- `.expect("err msg");` is a method of `Result` and it allows you to have an error message
  - `Result`'s variants are `Ok` and `Err`
  - `expect` returns the `Ok` result or crashes the program on `Err` and prints the error message
- String placeholders are done with encapsulating curly brackets like `"You guessed: {guess};`  
  - When printing the result of an expression, place empty curly brackets in the format string, then follow the format string with a comma-separated list of expressions to print in each empty curly bracket of the same order. For example:
```rust
let x = 5;
let y = 10;

println!("x = {x}, y + 2 = {}, x + y = {}", y + 2, x + y);
```
```
```

- Some other things were touched. I added notes in the guessing_game main source

## Chapter 3

Rust programming features that mirror other programming languages

- Examples in ./projects/variables/

- `mut` is required to make a variable mutable else it can be considered "final" 
  - Conveys that the variable can be changed
- `const` also signals that values are bound to a name and are not allowed to change, similar to a `let` variable without `mut`. However, there are a few differences:
  - You declare constants with `const` rather than `let`
  - The type of the value must be annotated
  - Constants can be declared in any scope, including the global scope
  - Constants may be set only to a constant expression and not to a value that can only be computed at runtime
    - See the Rust Reference's section on constant evaluation for more information on what operations can be used when declaring constants.
- Shadows are only valid within the scope of the shadow. Else the name refers to the original declaration scope
- You can change the type of what is shadowed

## Chapter 4

Covers Rust ownership system

## Chapter 5

Covers structs and methods

## Chapter 6

Covers enums; `match`; and `if`, `let`, and `let...else` control flow. Structs and enums are used to make custom types.

## Chapter 7

Covers the module system and privacy rules. Used for organizing code and for creating the public application programming interface.

## Chapter 8

Covers some common collection data structures: vectors, strings, and hash maps.

## Chapter 9

Covers error-handling.

## Chapter 10

Covers generics, traits, and lifetimes. 


## Chapter 11

Covers testing.

## Chapter 12

Project chapter to build a subset of `grep` functionality. Covers things discussed in previous chapters.

## Chapter 13

Covers closures and iterators. Derived from the functional programming paradigm.

## Chapter 14

Covers a deeper look into Cargo with best practices for sharing libraries with others.

## Chapter 15

Covers smart pointers.

## Chapter 16

Covers some Rust specific models of concurrent programming. 

## Chapter 17

Covers Rust async and await, along with tasks, futures, and streams. 

## Chapter 18

Covers Rust comparisons with more traditional OOP principles.

## Chapter 19

Covers patterns and pattern matching.

## Chapter 20

Covers a variety of advanced Rust topics, like unsafe Rust, macros, more about lifetimes, traits, types, functions, and closures. 

## Chapter 21

Project chapter to implement a low-level multithreaded web server. 

## Appendix A

Covers Rust keywords

## Appendix B

Covers Rust operators and symbols

## Appendix C

Covers derivable traits provided by the standard library.

## Appendix D

Covers some useful development tools

## Appendix E

Explains Rust editions

## Appendix F

Has links to translations of the book

## Appendix G

Covers how Rust is made and nightly editions

