use std::io;

fn main() {
    // taking user input
    println!("Please input your guess: ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read user input");

    println!("You guessed {}", guess);

    // generating random number b/w 1-100
    
}