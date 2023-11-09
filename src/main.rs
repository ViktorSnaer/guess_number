// bring io input/output lib into scope. Comes from rust standard lib (std)
// by default rust has set of items defined in std lib that it brings into scope called (prelude)
// if type needed is not in the prelude it needs to be added to scope by use statement
use std::io;
use std::cmp::Ordering;
// rand is from crate
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut number_of_guesses = 8;

    loop {
        if number_of_guesses == 0 {
            println!("Game over");
            break;
        }
    println!("Number of guesses left: {number_of_guesses}");
    println!("Please input your guess.");

    // mutable var that is bound to a new empty instance of a String
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // & indicates that its a reference to a mutable var
        .expect("Failed to read line"); // if input causes a program crash "Failed to read line" will print
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                number_of_guesses -= 1;
            },
            Ordering::Greater => {
                println!("Too big");
                number_of_guesses -= 1;
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
