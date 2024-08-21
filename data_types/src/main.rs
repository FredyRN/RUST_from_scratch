fn main() {
    let mut a_number = 16;
    let a_word = "CEVICHE";
    println!("The stored number is {}", a_number);
    a_number=23;
    println!("The new stored number is {}", a_number);
    println!("The stored word is {}", a_word);
    println!("*-------------*");
    let shadow = 8;
    let shadow = shadow - 2;
    let shadow = shadow * 2;
    println!("The shadow number is {}", shadow);
    println!("*---------------*");
    println!("Data types: numbers");
    println!("*---------------*");
    println!("Signed numbers");
    let s_number: i32 = -21;
    println!("The signed <<i32>> number is {}", s_number);
    println!("*---------------*");
    println!("Unsigned numbers");
    let number: u32 = 22;
    println!("The usigned <<u32>> number is {}", number);
    println!("*---------------*");
    println!("Floating-point numbers");
    let number_64f: f64 = 3.1;
    let number_32f: f32 = 4.6;
    println!("Floating point number <<64f>> is {}", number_64f);
    println!("Floating point number <<32f>> is {}", number_32f);
}
