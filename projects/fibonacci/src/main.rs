// fn fibo(n: u32) -> u32 {
//     if n < 2 {
//         return n;
//     } else {
//         return fibo(n - 1) + fibo(n - 2);
//     }
// }

fn fibo(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

fn main() {
    let n: u32 = 50;
    println!("Fibonacci({n}) = {}", fibo(n));
}
