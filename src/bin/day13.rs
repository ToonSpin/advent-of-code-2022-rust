use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    List(Vec<Value>),
    Integer(u32),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Value::List(left) => match other {
                Value::List(right) => {
                    if left.len() == 0 || right.len() == 0 {
                        left.len().cmp(&right.len())
                    } else {
                        let mut index = 0;
                        let mut result = Ordering::Equal;
                        while let Ordering::Equal = result {
                            result = left[index].cmp(&right[index]);
                            index += 1;
                            if left.len() == index || right.len() == index {
                                if let Ordering::Equal = result {
                                    result = left.len().cmp(&right.len());
                                    break;
                                }
                            }
                        }
                        result
                    }
                }
                Value::Integer(_) => {
                    let right = Value::List(vec![other.clone()]);
                    self.cmp(&right)
                }
            },
            Value::Integer(left) => match other {
                Value::List(_) => {
                    let left = Value::List(vec![Value::Integer(*left)]);
                    left.cmp(&other)
                }
                Value::Integer(right) => left.cmp(right),
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_integer(input: &str) -> IResult<&str, Value> {
    map(parse_u32, |i| Value::Integer(i))(input)
}

fn parse_list(input: &str) -> IResult<&str, Value> {
    let inner = separated_list0(tag(","), parse_value);
    let outer = delimited(tag("["), inner, tag("]"));
    map(outer, |v| Value::List(v))(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((parse_list, parse_integer))(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Value, Value)> {
    let parse_pair = separated_pair(parse_list, line_ending, parse_list);
    terminated(parse_pair, line_ending)(input)
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<(Value, Value)>> {
    separated_list1(line_ending, parse_pair)(input)
}

fn pairs_to_vec(v: &Vec<(Value, Value)>) -> Vec<Value> {
    let v: Vec<Vec<Value>> = v.iter().map(|(v, w)| vec![v.clone(), w.clone()]).collect();
    v.into_iter().flatten().collect()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let (_, input) = parse_pairs(input).unwrap();

    let mut index_sum = 0;
    for (i, (left, right)) in input.iter().enumerate() {
        if left < right {
            index_sum += i + 1;
        }
    }
    println!("The sum of indices of all in-order pairs: {}", index_sum);

    let divider_a = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    let divider_b = Value::List(vec![Value::List(vec![Value::Integer(6)])]);

    let mut part_two = pairs_to_vec(&input);
    part_two.push(divider_a.clone());
    part_two.push(divider_b.clone());
    part_two.sort();

    let a = part_two.binary_search(&divider_a).unwrap() + 1;
    let b = part_two.binary_search(&divider_b).unwrap() + 1;
    println!("The product of indices of the divider packets: {}", a * b);

    Ok(())
}
