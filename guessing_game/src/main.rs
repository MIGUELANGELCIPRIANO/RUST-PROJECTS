use std::io; // use the io input/output library from the std library

fn main() {
    println!("Guess the number!"); // println! is a macro that prints a string to the screen

    println!("Please input your guess.");

    let mut guess = String::new();  // let mut is a mutable variable and String::new() is a function that returns a new instance of string
                                    // this line has created a mutable variable that is currently bound to a new, empty instance of String 

    io::stdin() // call the stdin function from the io module, which will allow us to handle user input
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {guess}");
}
