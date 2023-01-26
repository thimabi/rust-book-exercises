use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Option<Guess> {
        if !(1..=100).contains(&value) {
            return None;
        }

        Some(Guess { value })
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess, then press Enter.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess: Guess = match Guess::new(guess) {
            Some(guess) => guess,
            None => {
                println!("Guess must be between 1 and 100, including both of them.");
                continue;
            }
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
