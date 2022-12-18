use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

type Point = (u32, u32);
type Polyline = Vec<Point>;

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(parse_u32, tag(","), parse_u32)(input)
}

fn parse_polyline(input: &str) -> IResult<&str, Polyline> {
    separated_list1(tag(" -> "), parse_point)(input)
}

fn parse_polylines(input: &str) -> IResult<&str, Vec<Polyline>> {
    separated_list1(tag("\n"), parse_polyline)(input)
}

fn min(a: u32, b: u32) -> u32 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

fn polyline_to_point_set(polyline: &Polyline) -> HashSet<Point> {
    let mut result = HashSet::new();
    for i in 1..polyline.len() {
        let ((a, b), (p, q)) = (polyline[i - 1], polyline[i]);
        for y in min(b, q)..=max(b, q) {
            for x in min(a, p)..=max(a, p) {
                result.insert((x, y));
            }
        }
    }
    result
}

fn add_poly(s: HashSet<Point>, v: &Polyline) -> HashSet<Point> {
    let t = polyline_to_point_set(v);
    s.union(&t).copied().collect()
}

fn simulate_sand_part_one(material: &mut HashSet<Point>, max_y: u32) -> () {
    let mut done = false;
    while !done {
        let (mut x, mut y) = (500, 0);
        loop {
            if y > max_y {
                done = true;
                break;
            }
            y += 1;
            if !material.contains(&(x, y)) {
                continue;
            }
            if !material.contains(&(x - 1, y)) {
                x -= 1;
                continue;
            }
            if !material.contains(&(x + 1, y)) {
                x += 1;
                continue;
            }
            material.insert((x, y - 1));
            break;
        }
    }
}

fn simulate_sand_part_two(material: &mut HashSet<Point>, max_y: u32) -> () {
    let mut queue = vec![(500, 0)];
    while let Some((x, y)) = queue.pop() {
        material.insert((x, y));
        if y == max_y {
            continue;
        }
        if !material.contains(&(x, y + 1)) {
            queue.push((x, y + 1));
        }
        if !material.contains(&(x - 1, y + 1)) {
            queue.push((x - 1, y + 1));
        }
        if !material.contains(&(x + 1, y + 1)) {
            queue.push((x + 1, y + 1));
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_, input) = parse_polylines(&input[..]).unwrap();
    let input = input.iter().fold(HashSet::new(), |a, e| add_poly(a, &e));

    let len_pre = input.len();
    let max_y = *input.iter().map(|(_, y)| y).max().unwrap();

    let mut part_one = input.clone();
    simulate_sand_part_one(&mut part_one, max_y);
    let units_to_rest = part_one.len() - len_pre;
    println!("Units before falling to the abyss: {}", units_to_rest);

    let mut part_two = input.clone();
    simulate_sand_part_two(&mut part_two, max_y + 1);
    let units_to_rest = part_two.len() - len_pre;
    println!("Units before equilibrium: {}", units_to_rest);
    Ok(())
}
