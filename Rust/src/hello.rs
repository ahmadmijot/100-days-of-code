
/*
to compile: rustc file.rs
to execute: ./file
*/

fn main() {
    let answer = 42; //declare var
    println!("Hello {}", answer); //the ! in println! is a macro call
    assert_eq! (answer, 42) //assert that two things are equal, if not, panic
}
