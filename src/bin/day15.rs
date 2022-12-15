use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::many1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

fn min<T: Ord>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}

fn max<T: Ord>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

type Point = (i64, i64);
type Interval = (i64, i64);

fn rotate((x, y): Point) -> Point {
    (x - y, x + y)
}

fn unrotate((x, y): Point) -> Point {
    ((x + y) / 2, (y - x) / 2)
}

struct Rect {
    ge: Point,
    lt: Point,
}

#[derive(Debug)]
struct Pair {
    sensor: Point,
    beacon: Point,
}

impl Pair {
    fn dist(&self) -> i64 {
        let dist_x = self.sensor.0 - self.beacon.0;
        let dist_y = self.sensor.1 - self.beacon.1;
        dist_x.abs() + dist_y.abs()
    }

    fn possible_beacons_at_y(&self, y: i64) -> Option<Interval> {
        let dist = self.dist();
        let d_y = (self.sensor.1 - y).abs();
        if d_y <= dist {
            Some((
                self.sensor.0 - (dist - d_y),
                self.sensor.0 + (dist - d_y) + 1,
            ))
        } else {
            None
        }
    }

    fn rotated_rect(&self) -> Rect {
        let dist = self.dist();
        let (x, y) = rotate(self.sensor);
        let ge = (x - dist, y - dist);
        let lt = (x + dist + 1, y + dist + 1);
        Rect { ge, lt }
    }
}

fn rects_to_slices(rects: &Vec<Rect>) -> (Vec<i64>, Vec<i64>) {
    let mut horiz = HashSet::new();
    let mut vert = HashSet::new();
    for r in rects.iter() {
        horiz.insert(r.ge.0);
        horiz.insert(r.lt.0);
        vert.insert(r.ge.1);
        vert.insert(r.lt.1);
    }
    let mut horiz: Vec<i64> = horiz.into_iter().collect();
    let mut vert: Vec<i64> = vert.into_iter().collect();
    horiz.sort();
    vert.sort();
    (horiz, vert)
}

fn slice_to_units(slice: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 1..slice.len() {
        let (x1, x2) = (slice[i - 1], slice[i]);
        if x2 - x1 == 1 {
            result.push(x1);
        }
    }
    result
}

fn slices_to_unit_squares(slices: (Vec<i64>, Vec<i64>)) -> Vec<Point> {
    let mut result = Vec::new();
    for x in slice_to_units(&slices.0).iter() {
        for y in slice_to_units(&slices.1).iter() {
            result.push((*x, *y));
        }
    }
    result
}

fn disjoint(a: Interval, b: Interval) -> bool {
    a.0 > b.1 || b.0 > a.1
}

fn subtract_coord(intervals: &Vec<Interval>, coord: i64) -> Vec<Interval> {
    let mut result = Vec::new();
    for (x1, x2) in intervals.iter() {
        if *x2 - *x1 == 1 && coord == *x1 {
            continue;
        } else if coord < *x1 {
            result.push((*x1, *x2));
        } else if coord >= *x2 {
            result.push((*x1, *x2));
        } else if *x1 == coord {
            result.push((x1 + 1, *x2));
        } else if coord + 1 == *x2 {
            result.push((*x1, (x2 - 1)));
        } else {
            result.push((*x1, coord));
            result.push((coord + 1, *x2));
        }
    }
    result
}

fn combine_nondisjoint(intervals: &Vec<Interval>) -> Vec<Interval> {
    let mut intervals = intervals.clone();
    intervals.sort();
    let mut retry = true;
    while retry {
        retry = false;
        let mut result = Vec::new();
        let mut current = intervals[0];
        for i in 1..intervals.len() {
            let interval = intervals[i];
            if disjoint(current, interval) {
                result.push(current);
                current = interval;
            } else {
                retry = true;
                let newx1 = min(current.0, interval.0);
                let newx2 = max(current.1, interval.1);
                current = (newx1, newx2);
            }
        }
        result.push(current);
        intervals = result;
    }
    intervals
}

fn possible_beacons_at_y(pairs: &Vec<Pair>, y: i64) -> i64 {
    let intervals = pairs
        .iter()
        .filter_map(|p| p.possible_beacons_at_y(y))
        .collect();
    let mut intervals = combine_nondisjoint(&intervals);
    for p in pairs.iter() {
        if p.beacon.1 == y {
            intervals = subtract_coord(&intervals, p.beacon.0);
        }
    }
    intervals.iter().map(|(x1, x2)| x2 - x1).sum()
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let raw_parser = recognize(pair(opt(tag("-")), digit1));
    map_res(raw_parser, |s: &str| s.parse::<i64>())(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    preceded(tag("x="), separated_pair(parse_i64, tag(", y="), parse_i64))(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let inner = separated_pair(parse_point, tag(": closest beacon is at "), parse_point);
    let mapper = |(sensor, beacon)| Pair { sensor, beacon };
    let outer = delimited(tag("Sensor at "), inner, line_ending);
    map(outer, mapper)(input)
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    many1(parse_pair)(input)
}

fn filter_part_two((x, y): &Point) -> bool {
    if *x < 0 || *x > 4000000 {
        false
    } else if *y < 0 || *y > 4000000 {
        false
    } else {
        true
    }
}

fn point_not_in((x, y): &Point, rects: &Vec<Rect>) -> bool {
    for r in rects.iter() {
        let ((a, b), (c, d)) = (r.ge, r.lt);
        if *x >= a && *y >= b && *x < c && *y < d {
            return false;
        }
    }
    true
}

fn tuning_freq((x, y): Point) -> i64 {
    x * 4000000 + y
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_, input) = parse_pairs(&input[..]).unwrap();

    let possible_beacons = possible_beacons_at_y(&input, 2000000);
    println!("Possible beacons at y = 2000000: {}", possible_beacons);

    let rects: Vec<Rect> = input.iter().map(|p| p.rotated_rect()).collect();
    let squares = slices_to_unit_squares(rects_to_slices(&rects));
    let squares: Vec<Point> = squares
        .into_iter()
        .filter(|p| point_not_in(p, &rects))
        .map(unrotate)
        .filter(filter_part_two)
        .collect();
    let freq = tuning_freq(squares[0]);
    println!("The tuning frequency of the beacon: {}", freq);

    Ok(())
}
