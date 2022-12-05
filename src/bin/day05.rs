use std::io;
use std::io::prelude::*;

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, newline};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug)]
struct Move {
    quantity: u32,
    from: usize,
    to: usize,
}

impl Move {
    fn apply_p1(&self, mut stacks: Vec<Stack>) -> Vec<Stack> {
        for _ in 0..self.quantity {
            let c = stacks[self.from].pop().unwrap();
            stacks[self.to].push(c);
        }
        stacks
    }

    fn apply_p2(&self, mut stacks: Vec<Stack>) -> Vec<Stack> {
        let split_index = stacks[self.from].len() - (self.quantity as usize);
        let mut tail = stacks[self.from].split_off(split_index);
        stacks[self.to].append(&mut tail);
        stacks
    }
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (rest, (_, q, _, f, _, t)) = tuple((
        tag("move "),
        parse_u32,
        tag(" from "),
        parse_usize,
        tag(" to "),
        parse_usize,
    ))(input)?;
    Ok((
        rest,
        Move {
            quantity: q,
            from: f - 1,
            to: t - 1,
        },
    ))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, parse_move)(input)
}

fn get_stacks(input: &str) -> Vec<Stack> {
    let lines: Vec<&str> = input.lines().rev().filter(|s| s.len() > 0).collect();
    let num_stacks = (lines[0].len() + 1) / 4;
    let mut stacks = vec![Vec::new(); num_stacks];
    for s in lines[1..].iter() {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..num_stacks {
            let c = chars[i * 4 + 1];
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    stacks
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (raw_moves, raw_stacks) = take_until::<_, _, Error<_>>("move")(input).unwrap();
    let stacks = get_stacks(raw_stacks);
    let (_, moves) = parse_moves(raw_moves).unwrap();

    let mut stacks_new = stacks.clone();
    for m in moves.iter() {
        stacks_new = m.apply_p1(stacks_new);
    }

    print!("The message (CrateMover 9000) reads: ");
    for stack in stacks_new.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!();

    let mut stacks_new = stacks;
    for m in moves.iter() {
        stacks_new = m.apply_p2(stacks_new);
    }

    print!("The message (CrateMover 9001) reads: ");
    for stack in stacks_new.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!();

    Ok(())
}
