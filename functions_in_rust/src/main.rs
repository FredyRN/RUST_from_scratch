fn main() {
    println!("Hello, world!");
    say_my_name("Fredy Rodriguez");
    // Function return a value
    let num: u32 = 220;
    let x: u32 = 23;
    println!("{} divided by {} = {}", num, x, divide_by_x(num, x));
    println!("{} divided by {} = {}", num, x, divide_by_n(num, x));
}

// Functions may or may not have input arguments
// the function's input arguments are specified as a comma-separated list
// of data types inside parentheses.

fn say_my_name(name: &str){
    println!("Your name is {}", name);
}

// Functions may or may not return some value
// When a function returns a value, we add the syntax "-> <type>" after
// the list of arguments and before the opening curly bracket
fn divide_by_x(num: u32, x: u32) -> u32 {
    num / x
}

// We can use the "return" keyword at any point in the function to halt execution 
// and send a value back to the caller.
// Usually, the use of the "return" keyword is used in combination with a conditional test
fn divide_by_n(num: u32, n: u32) -> u32 {
    if num == 0{
        return 0;
    }
    num / n
}