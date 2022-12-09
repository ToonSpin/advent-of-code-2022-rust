use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Motion {
    dir: Direction,
    dist: i64,
}

impl Motion {
    fn apply(&self, pos: Position) -> Position {
        let (x, y) = pos;
        match self.dir {
            Direction::Up => (x, y + self.dist),
            Direction::Right => (x + self.dist, y),
            Direction::Down => (x, y - self.dist),
            Direction::Left => (x - self.dist, y),
        }
    }
}

type Position = (i64, i64);

fn touching(a: Position, b: Position) -> bool {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();
    dx <= 1 && dy <= 1
}

fn move_towards(from: Position, to: Position) -> Position {
    if from.0 == to.0 {
        if from.1 < to.1 {
            return (from.0, from.1 + 1);
        } else {
            return (from.0, from.1 - 1);
        }
    }

    if from.1 == to.1 {
        if from.0 < to.0 {
            return (from.0 + 1, from.1);
        } else {
            return (from.0 - 1, from.1);
        }
    }

    if (from.0 - to.0).abs() == 2 && (from.1 - to.1).abs() == 2 {
        return ((from.0 + to.0) / 2, (from.1 + to.1) / 2);
    }

    if (from.0 - to.0).abs() == 1 {
        if from.0 < to.0 {
            return move_towards((from.0 + 1, from.1), to);
        } else {
            return move_towards((from.0 - 1, from.1), to);
        }
    }

    if (from.1 - to.1).abs() == 1 {
        if from.1 < to.1 {
            return move_towards((from.0, from.1 + 1), to);
        } else {
            return move_towards((from.0, from.1 - 1), to);
        }
    }

    unreachable!()
}

struct State {
    visited: HashSet<Position>,
    rope: Vec<Position>,
}

impl State {
    fn new(rope_length: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        let rope = vec![(0, 0); rope_length];
        State { visited, rope }
    }

    fn apply_motion(mut self, motion: &Motion) -> Self {
        let new_head = motion.apply(self.rope[0]);
        let l = self.rope.len();
        while self.rope[0] != new_head {
            self.rope[0] = move_towards(self.rope[0], new_head);
            for i in 1..l {
                if !touching(self.rope[i - 1], self.rope[i]) {
                    self.rope[i] = move_towards(self.rope[i], self.rope[i - 1]);
                }
            }
            self.visited.insert(self.rope[l - 1]);
        }
        self
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Up, tag("U")),
        value(Direction::Right, tag("R")),
        value(Direction::Down, tag("D")),
        value(Direction::Left, tag("L")),
    ))(input)
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn parse_motion(input: &str) -> IResult<&str, Motion> {
    let make_motion = |(dir, dist)| Motion { dir, dist };
    let parse_raw_motion = separated_pair(parse_direction, tag(" "), parse_i64);
    map(parse_raw_motion, make_motion)(input)
}

fn parse_motions(input: &str) -> IResult<&str, Vec<Motion>> {
    separated_list1(line_ending, parse_motion)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_, input) = parse_motions(&input[..]).unwrap();

    let mut state = State::new(2);
    for motion in input.iter() {
        state = state.apply_motion(motion);
    }
    let visited = state.visited.len();
    println!("Visited positions for rope length 2: {:?}", visited);

    let mut state = State::new(10);
    for motion in input.iter() {
        state = state.apply_motion(motion);
    }
    let visited = state.visited.len();
    println!("Visited positions for rope length 10: {:?}", visited);

    Ok(())
}
