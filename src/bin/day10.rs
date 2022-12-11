use std::io;
use std::io::prelude::*;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{recognize, map_res, opt, value, map};
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded};

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let raw_parser = recognize(pair(opt(tag("-")), digit1));
    map_res(raw_parser, |s: &str| s.parse::<i32>())(input)
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Noop, tag("noop"))(input)
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let raw_parser = preceded(tag("addx "), parse_i32);
    map(raw_parser, |i| Instruction::Addx(i))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_addx, parse_noop))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}

#[derive(Debug)]
struct State {
    x_register: i32,
    elapsed_cycles: i32,
}

impl State {
    fn new() -> Self {
        State { x_register: 1, elapsed_cycles: 0 }
    }

    fn iterate_part1(&mut self, instruction: &Instruction) -> i32 {
        match instruction {
            Instruction::Noop => {
                self.elapsed_cycles += 1;
                self.x_register
            },
            Instruction::Addx(i) => {
                self.x_register += i;
                self.elapsed_cycles += 2;
                self.x_register - i
            },
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_, input) = parse_instructions(&input[..]).unwrap();

    let mut state = State::new();
    let mut interesting_states = vec![220, 180, 140, 100, 60, 20];
    let mut total_signal_strength = 0;

    let mut next_evaluation = interesting_states.pop().unwrap();
    for instruction in input.iter() {
        let prev_value = state.iterate_part1(instruction);
        if state.elapsed_cycles >= next_evaluation {
            total_signal_strength += next_evaluation * prev_value;
            match interesting_states.pop() {
                Some(n) => {
                    next_evaluation = n
                },
                None => {
                    break;
                },
            }
        }
    }

    println!("{}", total_signal_strength);

    Ok(())
}
