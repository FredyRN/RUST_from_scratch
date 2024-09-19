// Another common collection type in Rust is the hash map. 
// The HashMap<K, V> type stores data by mapping each jey K
// with its value V. While data in a vector is accessed by using an 
// integer index, data in a hash is accessed by using a key.
// To use a hash map this need to be imported by using the std::collecctions::HashMap

use std::collections::HashMap;


fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("The Adventures of Sherlock Holmes"), String::from("Eye lyked it alot."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));
    // We can iterate over everything
    for (book, review) in &reviews {
        println!("{book}: \"{review}\"");
    }
    // Or we can look for a specific review
    let book: &str = "Programming in Rust";
    println!("Review for \"{book}\": {:?}", reviews.get(book));
    // To remove an specific key
    let obsolete: &str = "Ancient Roman History";
    println!("\"{obsolete}\" removed.");
    reviews.remove(obsolete);
    println!("Review for {obsolete}: {:?}", reviews.get(obsolete));
}
