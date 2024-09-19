/* Rust Fibers Runtime aka Green Stackful Coroutines Threads  */

/*
* Also known as Green Threads in Java
* and Green process in Erlang */

/* Rust functions normally have prologue and epilogue
* proceedures to setup/tear down certain CPU registers
* (setting up the stack frames)
*
* `naked_function` simply means that we want to do
* the prologue and epilogue proceedures ourselves*/
#![feature(naked_functions)]

mod runtime;
mod thread;

fn main() {
    println!("Hello, world!");
}
