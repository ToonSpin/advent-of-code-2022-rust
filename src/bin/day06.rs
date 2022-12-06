use std::io;
use std::io::prelude::*;

use nom::bytes::complete::take;
use nom::character::complete::anychar;
use nom::combinator::{recognize, verify};
use nom::multi::many_till;
use nom::IResult;

fn all_different(input: &str) -> bool {
    let l = input.len();
    let input = input.as_bytes();
    for i in 0..l {
        for j in i + 1..l {
            if &input[i] == &input[j] {
                return false;
            }
        }
    }
    true
}

fn marker_parser(marker_len: usize) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input: &str| verify(take(marker_len), all_different)(input)
}

fn length_before_marker(input: &str, marker_len: usize) -> usize {
    let (_, prefix) = recognize(many_till(anychar, marker_parser(marker_len)))(input).unwrap();
    return prefix.len();
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let l = length_before_marker(input, 4);
    println!("Length before the start-of-packet marker: {}", l);

    let l = length_before_marker(input, 14);
    println!("Length before the start-of-message marker: {}", l);

    Ok(())
}
