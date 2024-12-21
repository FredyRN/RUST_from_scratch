fn fibo(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fibo(n - 1) + fibo(n - 2);
    }
}

fn main() {
    let n: u32 = 300;
    println!("Fibonacci({n}) = {}", fibo(n));
}
