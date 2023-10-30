fn main() {
    let n: i32 = 9;
    println!("{}\"th Fibonacci Number: {}", n, fib(n))
}

fn fib(n: i32) -> i32 {
    match n <= 1 {
        true => return n,
        false => return fib(n - 1) + fib(n - 2),
    }
}
