/*
Guessing game
The program will generate random integer between 1 and 100. Then it will prompt
the player to enter a guess. Then the program will indicate wheter the guess
is too low or too high. If correct, the game will print congralulatory message
and exit
*/

/*
First part:
1. ask user input
2. process that output
3. check that the input is in the expected form

Second part: 
Generate secret number
Upadate Cargo.toml to include rand library (rand = "0.3.14")

Third part: 
Compare guess to the secret number 

Fourth part: 
Create loop
Quit after correct guess (break statement)

Fifth: 
Handling invalid input

*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 10);

    // println!("The secret number is {}!", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); //let var mutable = new instance of string
    
        //& = arg is reference-> give a way to let multiple parts of my code access
        // one piece of data without cpying the data into memory multiple times
        // read_line = thae user input and place into a string (take string as arg)
        io::stdin().read_line(&mut guess)
          .expect("Failed to read line"); // best practice to introduce new line for .foo()

        let guess: u32 = match guess.trim().parse() { // "convert" (shadowing) String to int
            //let guess: u32 = guess.trim().parse()
            //.expect("please type a number!") // crash the program
            Ok(num) => num,
            Err(_) => continue, //ignore non-number
            };
    
        println!("You guessed: {}", guess);

    // compare
        match guess.cmp(&secret_number){
          Ordering::Less => println!("Too smol"), //mind the syntax! , not ;
          Ordering::Greater => println!("Too big"),
          Ordering::Equal => {
              println!("You win!");
              break}
        }
    
    }
}
