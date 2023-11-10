use std::time::Instant;

fn main() {
    let start = Instant::now();

    let number = 10000000;
    let vec: Vec<u32> = (2..=number).filter(|&n| is_prime(n)).collect();
    println!("{:?}", vec);
    let end = start.elapsed();

    println!("{:.2?}", end);
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 || n == 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;

    let mut i = 5;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}
