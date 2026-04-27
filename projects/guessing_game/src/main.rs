use std::cmp::Ordering;
// from standard library, pull the Ordering type
use std::io;
// from standard library, use the io library

// Pulls in Rng trait 
use rand::Rng;

fn main() {
    println!("Guess the number:");

    // thread_rng is a thread local random member generator
    // 1..=100 is 1 to 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // create a mutable variable `guess`
        // call the `new` function of String for `guess` to be equal to
        // The `::` indicates that `new` is associated to the `String` type 
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        // matching the ouput of parse, the Result type, into its parts
        // shadows guess and converts to u32 if Ok 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
