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
    let n: usize = 3000;
    println!("Fibonacci({n}) = {}", fibo(n));
}
