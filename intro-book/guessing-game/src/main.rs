use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play the guessing game !!!!!");

    loop {
        let secret_number = rand::rng().random_range(1..=100);

        println!("Guess a number : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your number is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Way below, the random number is {secret_number}"),
            Ordering::Greater => println!("Way above, the random number is {secret_number}"),
            Ordering::Equal => {
                println!(" You Win ! ");
                break;
            }
        }
    }
    println!("Guess a number : ")
}
