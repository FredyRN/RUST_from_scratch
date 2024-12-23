use std::io;
use rand::seq::SliceRandom;

fn new_password(length:u8) ->  String {
    const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGITS: &str = "0123456789";
    const PUNCTUATION: &str = "!@#$%&*()_+=[]{}\\/;:.";
    let mut letters: Vec<char> = format!("{}{}{}{}", LOWER, UPPER, DIGITS, PUNCTUATION)
        .chars()
        .collect();
    let mut rand_number = rand::thread_rng();
    (0..length)
        .map(|_| *letters.choose_mut(&mut rand_number).unwrap())
        .collect()
}

fn main() {
    println!("Welcome to password generator.");
    println!("Write password length or '0' to finish program.");
    println!("Write the length of password to generate: ");
    loop {
        let mut pswd = String::new();
        io::stdin()
            .read_line(&mut pswd)
            .expect("Error is not a number.");
        let pswd = pswd.trim();
        if pswd.eq_ignore_ascii_case("0") {
            println!("Thank you for use password generator.");
            break;
        }
        let length = match pswd.parse::<u8>() {
            Ok(lng) => lng,
            Err(_) => {
                println!("Please write a valid number 0-128");
                continue;
            }
        };
        if length == 0 {
            println!("This value is required.");
            continue;
        } else if length > 128 {
            println!("Max length is 128 characters.");
            continue;
        }
        println!("{}", new_password(length));
    }
}
