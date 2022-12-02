use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut input: Vec<u64> = input
        .split("\n\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.split('\n').map(|s| s.parse().unwrap()).collect())
        .map(|v: Vec<u64>| v.iter().sum::<u64>())
        .collect();
    input.sort();
    input.reverse();

    println!(
        "The elf carrying the most calories is carrying: {}",
        input[0]
    );
    println!(
        "The three elves carrying the most calories are carrying:{}",
        input[0] + input[1] + input[2]
    );

    Ok(())
}
