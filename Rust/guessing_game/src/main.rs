/*
Guessing game
The program will generate random integer between 1 and 100. Then it will prompt
the player to enter a guess. Then the program will indicate wheter the guess
is too low or too high. If correct, the game will print congralulatory message
and exit
*/

/*first part:
1. ask user input
2. process that output
3. check that the input is in the expected form
*/

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}!", secret_number);

    println!("Please input your guess:");

    let mut guess = String::new(); //let var mutable = new instance of string
    
    //& = arg is reference-> give a way to let multiple parts of my code access
    // one piece of data without cpying the data into memory multiple times
    // read_line = thae user input and place into a string (take string as arg)
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); // best practice to introduce new line for .foo()

    println!("You guessed: {}", guess);


/*
Second partthread::spawn(move || {
    generate secret number

Upadate Cargo.toml to include rand library (rand = "0.3.14")
});
*/


}
