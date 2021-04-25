use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1].parse::<i128>().unwrap() {
        1 => question1(args[2].parse::<i128>().unwrap()),
        2 => question2(args[2].parse::<i128>().unwrap()),
        3 => question3(args[2].parse::<i128>().unwrap()),
        4 => question4(
            args[2].parse::<i128>().unwrap(),
            args[3].parse::<i128>().unwrap(),
        ),
        _ => println!("enter a correct parameter"),
    }
}

fn question1(number: i128) {
    for n in 2..number {
        if number % n == 0 {
            return println!("not prime");
        }
    }
    return println!("prime");
}

fn question2(number: i128) {
    let sqrt_number = (number as f64).sqrt();
    for n in 2..=sqrt_number as i128 {
        if number % n == 0 {
            return println!("not prime");
        }
    }
    return println!("prime");
}

fn sqrt_factor(number: i128) -> i128 {
    let sqrt_number = (number as f64).sqrt();
    for n in 2..=sqrt_number as i128 {
        if number % n == 0 {
            return n;
        }
    }
    return 1;
}

fn question3(number: i128) {
    let mut factors = Vec::new();
    let mut current_number = number;
    loop {
        let prime_factor = sqrt_factor(current_number);
        if prime_factor == 1 {
            factors.push(current_number);
            break;
        }
        factors.push(prime_factor);
        current_number = current_number / prime_factor;
    }
    println!("{:?}", factors);
}

// Euclidean algorithm
fn question4(number1: i128, number2: i128) {
    let mut x: i128 = number1;
    let mut y: i128 = number2;
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    println!("{}", x)
}
