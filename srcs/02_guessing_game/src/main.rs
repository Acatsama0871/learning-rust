use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secrete_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please enter your guess:");
        
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            },
            Ordering::Greater => println!("Too large"),
        }
    }
}
