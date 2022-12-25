use std::io;
use std::io::prelude::*;

fn to_snafu(mut n: i64) -> Vec<i64> {
    let mut result = Vec::new();
    if n == 0 {
        return vec![0];
    }
    while n > 0 {
        let digit = (n + 2) % 5 - 2;
        result.push(digit);
        n -= digit;
        n /= 5;
    }
    result
}

fn from_snafu(snafu: &str) -> i64 {
    let mut result = 0;
    for c in snafu.chars() {
        result *= 5;
        let digit: i64 = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!("Got unknown digit {}", c),
        };
        result += digit;
    }
    result
}

fn print_digits(digits: Vec<i64>) {
    for digit in digits.iter().rev() {
        match *digit {
            -2 => print!("="),
            -1 => print!("-"),
            _ => print!("{}", *digit),
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = input.lines();

    let mut sum = 0;
    for l in input {
        sum += from_snafu(l);
    }

    print!("The SNAFU number to input into Bob's console: ");
    print_digits(to_snafu(sum));
    println!();
    Ok(())
}
