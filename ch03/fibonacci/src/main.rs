fn main() {
    let n = std::env::args().nth(1).expect("Missing argument");
    let n: u32 = n.parse().expect("Invalid argument");
    println!("{}", fib(n));
}

fn fib(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
