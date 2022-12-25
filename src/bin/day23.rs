use ahash::{AHashMap, AHashSet};
use std::io;
use std::io::prelude::*;

type Pos = (i64, i64);

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn vicinity_contains_elf((x, y): Pos, elves_set: &AHashSet<Pos>) -> bool {
    for p in x - 1..=x + 1 {
        for q in y - 1..=y + 1 {
            if p == x && q == y {
                continue;
            }
            if elves_set.contains(&(p, q)) {
                return true;
            }
        }
    }
    false
}

fn check_for_proposal((x, y): Pos, elves_set: &AHashSet<Pos>, dir: Direction) -> Option<Pos> {
    match dir {
        Direction::North => {
            for p in x - 1..=x + 1 {
                if elves_set.contains(&(p, y - 1)) {
                    return None;
                }
            }
            return Some((x, y - 1));
        }
        Direction::South => {
            for p in x - 1..=x + 1 {
                if elves_set.contains(&(p, y + 1)) {
                    return None;
                }
            }
            return Some((x, y + 1));
        }
        Direction::West => {
            for q in y - 1..=y + 1 {
                if elves_set.contains(&(x - 1, q)) {
                    return None;
                }
            }
            return Some((x - 1, y));
        }
        Direction::East => {
            for q in y - 1..=y + 1 {
                if elves_set.contains(&(x + 1, q)) {
                    return None;
                }
            }
            return Some((x + 1, y));
        }
    }
}

fn round(mut elves: Vec<Pos>, round_number: usize) -> (u32, Vec<Pos>) {
    let mut proposals: AHashMap<Pos, Vec<usize>> = AHashMap::new();
    let elves_set: AHashSet<Pos> = AHashSet::from_iter(elves.iter().copied());
    let mut dirs: Vec<Direction> = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    dirs = dirs[round_number % 4..(round_number % 4) + 4]
        .iter()
        .copied()
        .collect();

    for (i, elf) in elves.iter().enumerate() {
        if !vicinity_contains_elf(*elf, &elves_set) {
            continue;
        }
        for dir in dirs.iter() {
            if let Some(p) = check_for_proposal(*elf, &elves_set, *dir) {
                let v = proposals.entry(p).or_insert(Vec::new());
                v.push(i);
                break;
            }
        }
    }

    let mut elves_moved = 0;
    for (p, v) in proposals.iter() {
        if v.len() == 1 {
            elves_moved += 1;
            elves[v[0]] = *p;
        }
    }
    (elves_moved, elves)
}

fn bounding_box(elves: &Vec<Pos>) -> (Pos, Pos) {
    let mut min_x = elves[0].0;
    let mut min_y = elves[0].1;
    let mut max_x = min_x;
    let mut max_y = min_y;

    for (x, y) in elves.iter() {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    ((min_x, min_y), (max_x, max_y))
}

fn empty_within_bb(elves: &Vec<Pos>) -> usize {
    let ((min_x, min_y), (max_x, max_y)) = bounding_box(&elves);
    let w = (max_x - min_x + 1) as usize;
    let h = (max_y - min_y + 1) as usize;
    w * h - elves.len()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let mut elves: Vec<Pos> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push((x as i64, y as i64));
            }
        }
    }

    let mut i = 0;
    let mut num_moved;
    loop {
        (num_moved, elves) = round(elves, i);
        if i == 9 {
            let empty_in_bb = empty_within_bb(&elves);
            println!("Empty tiles within bounding box: {}", empty_in_bb);
        }
        if num_moved == 0 {
            println!("First round with no movement: {}", i + 1);
            break;
        }
        i += 1;
    }

    Ok(())
}
