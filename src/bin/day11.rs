use std::io;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;

type Item = u64;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Old,
    Scalar(Item),
}

fn operand_value(operand: &Operand, old: Item) -> Item {
    match operand {
        Operand::Old => old,
        Operand::Scalar(n) => *n,
    }
}

#[derive(Clone, Copy, Debug)]
struct Operation {
    operator: Operator,
    operands: (Operand, Operand),
}

impl Operation {
    fn evaluate(&self, old: Item) -> Item {
        let a = operand_value(&self.operands.0, old);
        let b = operand_value(&self.operands.1, old);
        match self.operator {
            Operator::Add => a + b,
            Operator::Mul => a * b,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<Item>,
    op: Operation,
    modulus: u64,
    monkey_true: usize,
    monkey_false: usize,
    inspection_count: u64,
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    map_res(digit1, |s: &str| s.parse::<Item>())(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_add(input: &str) -> IResult<&str, Operator> {
    value(Operator::Add, tag("+"))(input)
}

fn parse_mul(input: &str) -> IResult<&str, Operator> {
    value(Operator::Mul, tag("*"))(input)
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((parse_add, parse_mul))(input)
}

fn parse_operand_old(input: &str) -> IResult<&str, Operand> {
    value(Operand::Old, tag("old"))(input)
}

fn parse_operand_scalar(input: &str) -> IResult<&str, Operand> {
    let wrap_item = |n| Operand::Scalar(n);
    map(parse_item, wrap_item)(input)
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((parse_operand_old, parse_operand_scalar))(input)
}

fn parse_opening_line(input: &str) -> IResult<&str, usize> {
    delimited(tag("Monkey "), parse_usize, pair(tag(":"), line_ending))(input)
}

fn parse_starting_items_line(input: &str) -> IResult<&str, Vec<Item>> {
    let items_parser = separated_list1(tag(", "), parse_item);
    delimited(tag("  Starting items: "), items_parser, line_ending)(input)
}

fn parse_operation_line(input: &str) -> IResult<&str, Operation> {
    let (rest, (_, operand1, _, operator, _, operand2, _)) = tuple((
        tag("  Operation: new = "),
        parse_operand,
        tag(" "),
        parse_operator,
        tag(" "),
        parse_operand,
        line_ending,
    ))(input)?;
    let operation = Operation {
        operator,
        operands: (operand1, operand2),
    };
    Ok((rest, operation))
}

fn parse_modulus_line(input: &str) -> IResult<&str, Item> {
    delimited(tag("  Test: divisible by "), parse_item, line_ending)(input)
}

fn parse_monkey_true_line(input: &str) -> IResult<&str, usize> {
    delimited(
        tag("    If true: throw to monkey "),
        parse_usize,
        line_ending,
    )(input)
}

fn parse_monkey_false_line(input: &str) -> IResult<&str, usize> {
    delimited(
        tag("    If false: throw to monkey "),
        parse_usize,
        line_ending,
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (rest, (_, mut items, op, modulus, monkey_true, monkey_false)) = tuple((
        parse_opening_line,
        parse_starting_items_line,
        parse_operation_line,
        parse_modulus_line,
        parse_monkey_true_line,
        parse_monkey_false_line,
    ))(input)?;
    items.reserve(2000);
    let monkey = Monkey {
        items,
        op,
        modulus,
        monkey_true,
        monkey_false,
        inspection_count: 0,
    };
    Ok((rest, monkey))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(line_ending, parse_monkey)(input)
}

fn play_turn(
    mut monkeys: Vec<Monkey>,
    thrower: usize,
    modulus: Option<Item>,
    buffer: &mut Vec<(usize, Item)>,
) -> Vec<Monkey> {
    let m = &monkeys[thrower];
    let mut count = m.inspection_count;

    for item in m.items.iter() {
        let item = m.op.evaluate(*item);
        let item = if let Some(part2) = modulus {
            item % part2
        } else {
            item / 3
        };
        let catcher = if item % m.modulus == 0 {
            m.monkey_true
        } else {
            m.monkey_false
        };
        buffer.push((catcher, item));
        count += 1;
    }

    for (catcher, item) in buffer.iter() {
        monkeys[*catcher].items.push(*item);
    }
    buffer.clear();

    monkeys[thrower].items.clear();
    monkeys[thrower].inspection_count = count;
    monkeys
}

fn play_round(
    mut monkeys: Vec<Monkey>,
    modulus: Option<Item>,
    buffer: &mut Vec<(usize, Item)>,
) -> Vec<Monkey> {
    let l = monkeys.len();
    for i in 0..l {
        monkeys = play_turn(monkeys, i, modulus, buffer);
    }
    monkeys
}

fn play_rounds(mut input: Vec<Monkey>, count: u64, modulus: Option<Item>) -> u64 {
    let mut buffer = Vec::new();
    for _ in 0..count {
        input = play_round(input, modulus, &mut buffer);
    }
    let mut counts: Vec<u64> = input.iter().map(|m| m.inspection_count).collect();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_, input) = parse_monkeys(&input[..]).unwrap();

    let part1 = play_rounds(input.clone(), 20, None);
    println!("After the first 20 rounds: {:?}", part1);

    let mut modulus = 1;
    for m in input.iter() {
        modulus *= m.modulus;
    }
    let part2 = play_rounds(input.clone(), 10000, Some(modulus));
    println!("Without worry decreasing after 1000 rounds: {:?}", part2);

    Ok(())
}
