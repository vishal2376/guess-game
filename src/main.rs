use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();

    loop {
        println!("Please input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        let guess: u32 = guess.trim().parse().expect("Please enter Number");

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
