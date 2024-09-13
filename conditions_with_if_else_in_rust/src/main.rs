fn main() {
    // Arrays: array's signature in compile time is defined as [T, size]
    // T is the data type for all elements in the array
    // size is a nonnegative integer that represents the array lenght
    // Only one thing about an array can change over time: the values of the elements
    // in the array (data type and size both are constants)
    let days: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let bytes: [i32; 5] = [0; 5];
    println!("Today is {}", days[len]);
    println!("{}", bytes[0]);
}
