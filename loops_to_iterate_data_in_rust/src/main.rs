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
    /* Programs often have blocks of code that need to be repeated in place.
     * We can use loop expressions to  tell the program how to do the repetitions.
     * To print all entries in a phonebook, we can use a loop expression to tell the
     * program how to print starting from the first entry all the way through to the last entry.
     * RUST offers three loop expressions to make a program repeat a block of code.
    
     * loop: Repeat, unless a manual stops occurs.
     * while: Repeat while a condition remains true.
     * for: Repeat for all values in a collection.
     */
    loop {
        println!("This is an infinite loop expression!");
        break;
    }
    let mut counter: u32 = 1;
    // stop_loop is set when loop stops
    // let stop_loop = loop {
    //     counter *= 2;
    //     println!("Counter is {counter}");
    //     if counter > 100 {
    //         // Stop loop, eturn counter value
    //         break counter;
    //     }
    // };
    // println!("Break the loop at counter = {stop_loop}");
    /* Loop a while
     *  The while loop uses a condotional expression.
     *  Te loop repeats as long as the conditional expression remains true.
     *  This keyword lets us execute the actions in the expression body until
     *  the conditonal expression is false. 
     */
    while counter < 5 {
        println!("We loop a while {counter} times...");
        counter = counter + 1;
    }
    /*
     * The for loop uses an iterator to process a collection of items.
     * The loop repeats the actions in the expression body for each item in the collection.
     * In Rust, we can iterate oer any collection type, such as an array, vector, or hash map.
     * Rust uses an iterator to move through each item in the collection from first to last.
     * The for loop uses a temporry variable as the iterator.
     * The variable is implicitly declared at the start of the loop expression, and the 
     * current value is set with each iteration.
     */
    let big_dirds = ["ostrich", "peacock", "stork"];
    for bird in big_dirds.iter() {
        println!("The {} is a big bird.", bird);
    }
    
}
