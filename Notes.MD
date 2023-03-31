# Setup 

* See how to install rust and cargo (rust's build system) https://www.rust-lang.org/tools/install
* Run `cargo init hecto-clone`

## 1. Getting started

* To compile your code run `cargo build` + `./target/debug/hecto-clone` or `cargo run` (Which compiles and runs the programm, it does not recompile if no changes are recognised)

## 2. Reading user inputs

* We can `use std::io::{self, Read};` the terminals standard input stream to hook up the terminal input to our program. In default its connected in **cooked mode**, meaning the input is only send to the io stream after hitting enter
* However we want the terminal in **raw mode**, which passes the data directly to the porgram without any extra input &Rightarrow; To enter raw mode in rust we have to use external crates (Libraries)
* We use *stdout* to change the mode, because in terminals the writer determines the mode (bceause  it moves the cursor, draws on the screen etc.) 
* Using `let _stdout = ...` allows us to stay in raw mode. Because of rusts ownership we need to assign to a variable. Explained in short:
  * `into_raw_mode` sets the terminal into raw mode and returns a value
  * Once the value is removed the terminal will be reset to cooked mode
  * Rusts ownership system removes a function call after it is executed (if not bound to a variable)
  * Using the underscore "_" is just a convetion telling others we want to keep this variables even if not using it &Rightarrow; Rusts compiler will warn you if not using an underscore
* **Keypresses**:
  * We need the bytes to tranfer a key press to stoud 
  * For that purpose we use shadowing (Redeclaring a variable inside a loop with the same name as the loop). This is perfectly legal in rust
  * `\r` prints the output neatly line by line (Carriage return)
  * Functions need no return if the last expression should be returned, then the `;` is omitted.
  * `println!("{:#b}", b);` prints binary represenation of a character
  * Error handling in rust:
    * You cannot use `try .. catch` in Rust, instead we need to propagate errors along to the highest level

### Key Takeaways