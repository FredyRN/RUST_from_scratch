use std::io;

use num_bigint::BigUint;
use num_traits::One;

// fn fibo(n: u32) -> u32 {
//     if n < 2 {
//         return n;
//     } else {
//         return fibo(n - 1) + fibo(n - 2);
//     }
// }

// fn fibo(n: u32) -> u128 {
//     let mut a: u128 = 0;
//     let mut b: u128 = 1;
//     for _ in 0..n {
//         let temp = a;
//         a = b;
//         b = temp + b;
//     }
//     a
// }

fn fibo(n: usize) -> BigUint {
    let mut f0 = BigUint::ZERO;
    let mut f1 = BigUint::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}

fn main() {
    println!("Welcome to fibonacci generator.");
    println!("Write a number or '0' to finish program.");
    println!("Write the number to get fibonacci: ");
    loop {
        let mut xn = String::new();
        io::stdin()
            .read_line(&mut xn)
            .expect("Error is not a number.");
        let xn = xn.trim();
        if xn.eq_ignore_ascii_case("0") {
            println!("Thank you for use fibonacci generator.");
            break;
        }
        let number = match xn.parse::<usize>() {
            Ok(x)  => x,
            Err(_) => {
                println!("Please write a valid number.");
                continue;
            }
        };
        println!("Fibonacci({number}) = {}", fibo(number));
        println!("====+====+====+====+====");
    }
}
