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
    * Thus we need to wrap all code we want to do error handling on, which returns a Results, which in return either contains an error or the result we are after. More on this can be read here https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html 
    * With `match` we can handle errors. Basically it works like an if-else statement with different syntax 

### Key Takeaways for Rust

* Packages can be imported using the `use` declaration 
* Ownership, Borrowing and Shadowing are important concepts in Rust, regarding the scope of variables. 
* using an *"_"* is a convention telling the compiler that the variable will be assigned but not used later
* There is not `try ... catch` statement in Rust. A `Result` needs to be returned from a function and then checked at the highest level using the `match` Statement. More on error handling in rust: https://doc.rust-lang.org/book/ch09-00-error-handling.html


## 3. Raw input and output

**Idiomatic Rust code**

* Instead of reading bytes, we should read keys (Because reading bytes usually means you are to low level solving a problem someone else already solved in a library) &Rightarrow; In this case `termion`
* `_` as tree in a match statement handley every case that is not matched so far
* In Rust (as in every other programming language) the main function should only be the entry point of the app and should contain the least code possible. All other things are handled in different files, so that the programm is easier maintainable
* A `struct` is a collection of variables and eventually functions forming an entity (Similiar to pythons `class`). A big advantage of structs is that we can use several declarations like `pub, private, etc` to say where the struct should be accessible
* Implementing functions for a struct is done using the `impl` keyword
* `&self` is a reference to the struct a function was called upon &Rightarrow; `&` is the keyword for references

### Key Takeaways for Rust

* **Structs** are similiar to python classes and are initiated f.e `pub struct Example {...}`. They have accessor keywords, telling the compiler where the struct should be accessible from
