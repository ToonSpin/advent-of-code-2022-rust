use std::io;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res, opt, recognize, value};
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded};
use nom::IResult;

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

struct Tube {
    instructions: Vec<Instruction>,
    next_instruction: usize,
    x_register: i32,
    queue: Vec<i32>,
}

impl Tube {
    fn new(instructions: Vec<Instruction>) -> Self {
        Tube {
            instructions,
            next_instruction: 0,
            x_register: 1,
            queue: Vec::new(),
        }
    }
}

impl Iterator for Tube {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop() {
            Some(val) => Some(val),
            None => {
                if self.instructions.len() <= self.next_instruction {
                    return None;
                }
                let result = match self.instructions[self.next_instruction] {
                    Instruction::Noop => Some(self.x_register),
                    Instruction::Addx(argument) => {
                        self.queue.push(self.x_register);
                        self.queue.push(self.x_register);
                        self.x_register += argument;
                        self.queue.pop()
                    }
                };
                self.next_instruction += 1;
                result
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_, input) = parse_instructions(&input[..]).unwrap();
    let tube = Tube::new(input.clone());

    let mut interesting_states = vec![220, 180, 140, 100, 60, 20];
    let mut total_signal_strength = 0;
    let mut next_eval_part_one = interesting_states.pop().unwrap();
    let mut crt_display = Vec::new();

    for (i, val) in tube.enumerate() {
        let cycle = i as i32 + 1;
        if cycle == next_eval_part_one {
            total_signal_strength += next_eval_part_one * val;
            if let Some(n) = interesting_states.pop() {
                next_eval_part_one = n;
            }
        }
        let column = (i as i32) % 40;
        let pixel = if (val - column).abs() <= 1 { '#' } else { ' ' };
        crt_display.push(pixel);
    }
    println!("The total signal strength: {}", total_signal_strength);

    for line in crt_display.chunks(40) {
        let line: String = line.iter().collect();
        println!("{}", line);
    }

    Ok(())
}
