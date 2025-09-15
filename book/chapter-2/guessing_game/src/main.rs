use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    
    let mut guess = String::new(); // set guess as mutable
    
    io::stdin()
        .read_line(&mut guess)
        .except("Failed to read line");

    println!("You guessed: {guess}");
}
