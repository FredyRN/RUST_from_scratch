use std::io;

fn main() {
    println!("Welcome to my simple Rust calculator");
    println!("Write operations in format: number1 operator number 2");
    println!("Operators: +, -, *, /");

    loop {
        println!("Please, write your operation or 'exit' to finish.");
        let mut opr = String::new();
        io::stdin().read_line(&mut opr).expect("Error use soported operators.");
        let opr = opr.trim();
        if opr.eq_ignore_ascii_case("exit") {
            println!("Thank you for use the calculator.");
            break;
        }
        let parts: Vec<&str> = opr.split_whitespace().collect();
        if parts.len() !=3 {
            println!("Error, use the correct format.");
            continue;
        }
        let mut n1: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: {} is not a valid number.", parts[0]);
                continue;
            }
        };
        let mut operator = parts[1];
        let mut n2: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error {} is not a valid number.", parts[2]);
                continue;
            }
        };
        let result = match operator {
            "+" => n1 + n2,
            "-" => n1 - n2,
            "*" => n1 * n2,
            "/" => {
                if n2 == 0.0 {
                    println!("Error division by zero");
                    continue;
                }
                n1 / n2
            },
            _ => {
                println!("Invalid operator. Use valid one only [+, -, *, /]");
                continue;
            }
        };
        println!("{n1} {operator} {n2} = {result}");
    }
}