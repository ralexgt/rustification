// some things are not included in the prelude of the standard library,
// we must include them explicitly
use std::io; // io from std
use std::cmp::Ordering; // Ordering from cmp from std
use rand::Rng; // Rng from rand


fn main() {
    // initialize the game and generate a random number based on a seed given by the OS
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // main game loop that breaks when the correct answer was given
    loop {
        // prompt the user to give a guess
        println!("Input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // if the guess is not a positive number it goes to the next iteration of the loop
        // and prompts him again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess must be a positive number!");
                continue
            },
        };

        // show the user his guess and compare it with the secret number, if correct
        // the loop ends and so does the program
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            },
        }
    }
}