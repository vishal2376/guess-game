use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        println!("\nPlease input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
