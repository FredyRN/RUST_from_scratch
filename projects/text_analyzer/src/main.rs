mod analysis;
mod input;

fn main() {
    println!("Welcome to the Text Analyzer!");

    // Read input text
    let text = input::read_text();

    // Input analysis
    let word_count = analysis::count_words(&text);
    let char_count = analysis::count_characters(&text);

    // Print results
    println!("Word count: {}", word_count);
    println!("Character count: {}", char_count);
}
