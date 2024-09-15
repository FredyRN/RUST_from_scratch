fn main() {
    // Arrays data type
    // Array's signature in compile time is defined as [T, size]
    // T is the data type for all elements in the array
    // size is a nonnegative integer that represents the array lenght
    // Only one thing about an array can change over time: the values of the elements
    // in the array (data type and size both are constants)
    let days: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let bytes: [i32; 5] = [0; 5];
    println!("Days of week {:?}", days);
    println!("{:?}", bytes);
    println!("++++++++++++++++++++++++++++++++++++++");
    // Vector data type
    // As with arrays, vectors store multiple values that have the same data type.
    // Unlike arrays, th size or lenght of a vector can grow or shrink at any time.
    // The syntax <vector><T> declares a vector type composed of a generic (not yet known) data type
    // To create a vector, we use a concrete type like <vector>i32, a vector of type i32 or 
    // <vector>String, a vector of type String.
    let three_nums: Vec<i32> = vec![15, 2, 25];
    println!("Initial vector: {:?}", three_nums);
    let zeroes: Vec<i32> = vec![0; 5];
    println!("Zeroes {:?}", zeroes);
    // Vector can also be created by using the Vec::new() meethod. This method of vector creation lets us add
    // and remove values at the end of the vector. To support this behavior, we declare the vector as mutable
    // with the mut keyword.
    let mut fruit: Vec<&str> = Vec::new();
    // To add a value at the end of the vector, we use the push() method
    fruit.push("Banana");
    fruit.push("Apple");
    fruit.push("Avocado");
    println!("Fruits: {:?}", fruit);
    // To remove a value at the end of the vector, we use the pop() method
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);
    // index a vector
    println!("Vector: {:?}, two = {}", fruit, fruit[1]);
    // To change a value un place
    println!("Change fruit {} to Watermelon", fruit[1]);
    fruit[1] = "Watermelon";
    println!("Fruits: {:?}", fruit);
}
