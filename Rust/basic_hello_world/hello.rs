
/*
to compile: rustc file.rs
to execute: ./file

using Cargo
to start new project: cargo new hello_cargo
                      cd hello_cargo
to execute: cargo run
check codes compile but bot produce executable: cargo check
*/

fn main() {
    let answer = 42; //declare var
    println!("Hello {}", answer); //the ! in println! is a macro call
    assert_eq! (answer, 42) //assert that two things are equal, if not, panic
}
