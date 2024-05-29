// fn main() {
//     let name = "Uzair Rehman";

//     for c in name.chars() {
//         println!("{}", c);
//     }
// }

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Create a mutable String to store the user's guess
        let mut guess = String::new();

        // Read the user's input and store it in the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input from a string to a number
        // If the conversion fails, continue to the next iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
