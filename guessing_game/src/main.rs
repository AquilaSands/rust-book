use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut keep_going = true;

    let answer = thread_rng().gen_range(1, 101);

    while keep_going {
        let mut guess = String::new();

        println!("What's your guess?");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a number!");

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!(
                    "You guessed {}, the answer was {}. You are correct.",
                    guess, answer
                );
                keep_going = false
            },
        }
    }
}
