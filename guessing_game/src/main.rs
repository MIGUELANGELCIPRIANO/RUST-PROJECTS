use std::io; // use the io input/output library from the std library
use rand::Rng; // Rng trait defines methods that random number generators implement

fn main() {
    println!("Guess the number!"); // println! is a macro that prints a string to the screen

    let secret_number = rand::thread_rng().gen_range(1..=100);  // rand::thread_rng function that gives us the particular random number generator we’re going to use
    // gen_range method defined by Rng that takes a range expression as argument and generates a random number

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();  // let mut is a mutable variable and String::new() is a function that returns a new instance of string
    // this line has created a mutable variable that is currently bound to a new, empty instance of String 

    io::stdin() // call the stdin function from the io module, which will allow us to handle user input
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {guess}");
}
