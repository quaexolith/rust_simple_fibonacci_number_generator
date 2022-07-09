fn main() {
    for n in 0..10 {
        print_fibonacci_number(n);
    }
}

fn print_fibonacci_number(number: u32) {
    let index = number + 1;

    println!("Fibonacci number {index}: {}", fibonacci(number));
}

fn fibonacci(number: u32) -> u32 {
    match number {
        0 => 1,
        1 => 1,
        _ => fibonacci(number - 1) + fibonacci(number - 2),
    }
}
