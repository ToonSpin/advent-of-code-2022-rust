use std::io;
use std::io::prelude::*;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

type Range = (u32, u32);

fn contained_in(a: Range, b: Range) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn disjoint(a: Range, b: Range) -> bool {
    a.0 > b.1 || b.0 > a.1
}

fn overlaps(a: Range, b: Range) -> bool {
    !disjoint(a, b)
}

#[derive(Debug)]
struct RangePair {
    a: Range,
    b: Range,
}

impl RangePair {
    fn is_fully_contained(&self) -> bool {
        contained_in(self.a, self.b) || contained_in(self.b, self.a)
    }
    fn overlaps(&self) -> bool {
        overlaps(self.a, self.b)
    }
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    separated_pair(parse_u32, tag("-"), parse_u32)(input)
}

fn parse_range_pair_raw(input: &str) -> IResult<&str, (Range, Range)> {
    separated_pair(parse_range, tag(","), parse_range)(input)
}

fn parse_range_pair(input: &str) -> IResult<&str, RangePair> {
    let make_range = |(a, b)| RangePair { a, b };
    map(parse_range_pair_raw, make_range)(input)
}

fn parse_range_pairs(input: &str) -> IResult<&str, Vec<RangePair>> {
    separated_list1(line_ending, parse_range_pair)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_, input) = parse_range_pairs(&input[..]).unwrap();

    let count = input.iter().filter(|&p| p.is_fully_contained()).count();
    println!("Ranges where one fully contains the other: {}", count);

    let count = input.iter().filter(|&p| p.overlaps()).count();
    println!("Ranges where one overlaps the other: {}", count);

    Ok(())
}
