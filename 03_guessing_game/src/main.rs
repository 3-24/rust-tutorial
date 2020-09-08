use std::io;    // io is not in the prelude, so bring it into the scope explicitly.
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("The secret number is: {}", secret_number);

    // infinite loop
    loop {
        println!("Please input your guess.");
        
        // let - a statement which create a variable
        // mut - Set mutability. Rest variables are immutable by default.
        // :: - String::new idicates that new is an associated function of the String type.
        let mut guess = String::new();

        // & inidcates that this argument is a reference, therefore don't need to store the data into memory multiple times.
        // read_line returns io::Result. If this is an Err value, expect cause program to crash and display the message by expect().
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); 
        
        // u32: unsigned 32bit integer
        // trim removes whitespace
        // parse : string -> number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
