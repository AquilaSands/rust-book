use std::io;

fn main() {
    loop {
        let mut limit = String::new();

        println!("How many Fibonacci numbers?");

        io::stdin().read_line(&mut limit).expect("Failed to read.");

        match limit.trim().parse::<u32>() {
            Ok(num) => fib(num),
            Err(_) => continue
        };

        break
    }
}

fn fib(limit: u32) {
    let mut prev: u32 = 0;
    let mut fib: u32 = 0;

    for i in 1..limit + 1 {
        println!("fib-{}: {}", i, fib);

        let next = if fib != 0 { prev + fib } else { 1 };
        prev = fib;
        fib = next;
    }
}
