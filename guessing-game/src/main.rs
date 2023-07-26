use std::io; // Never forget the double colon or semi-colon!
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Input a number");

    let correct = rand::thread_rng().gen_range(1..=100);
    println!("Pssst! The correct answer is {correct}");

    loop {
        let mut guess = String::new(); // This is a mutable variable that is empty for now
        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong somehow lmao"); // note how on the .expect you need a semi
        
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Don't tell me you seriously guessed {guess}...");
        match guess.cmp(&correct) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}