use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn compare_guess_to_secret(guess: &String, secret: &String) -> String {
   match guess.cmp(secret) {
        Ordering::Less => String::from("Too small"),
        Ordering::Greater => String::from("Too big"),
        Ordering::Equal => String::from("You WIN!")
   }
}
fn main() {
   println!("Guess the number!");

   let secret_number: String = rand::thread_rng().gen_range(1..=99).to_string();

   println!("The secret number is: {}", secret_number);

   println!("Please input your guess...");

   let mut guess = String::new();

   io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

   compare_guess_to_secret(&guess, &secret_number);

   println!("You guessed: {}", guess);
}
