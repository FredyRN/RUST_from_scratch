use std::io;

pub fn read_text() -> String {
    println!("Enter the text to analyze: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input.");
    text.trim().to_string()
}