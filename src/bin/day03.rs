use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn prio(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        (c as u8 - ('a' as u8) + 1) as u64
    } else {
        (c as u8 - ('A' as u8) + 27) as u64
    }
}

fn common_char(v: &Vec<String>) -> char {
    let mut common: HashSet<char> = v[0].chars().collect();
    for s in v[1..].iter() {
        let t: HashSet<char> = s.chars().collect();
        common = common.intersection(&t).copied().collect();
    }
    for c in common.iter() {
        return *c;
    }
    unreachable!()
}

fn lines_to_vecs_p1(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    for line in lines.iter() {
        let half_length = line.len() / 2;
        let a = String::from(&line[..half_length]);
        let b = String::from(&line[half_length..]);
        result.push(vec![a, b])
    }
    result
}

fn lines_to_vecs_p2(lines: &Vec<String>) -> Vec<Vec<String>> {
    lines.chunks(3).map(|slice| Vec::from(slice)).collect()
}

fn common_prio(v: &Vec<String>, f: fn(&Vec<String>) -> Vec<Vec<String>>) -> u64 {
    f(&v).iter().map(common_char).map(prio).sum()
}

fn main() -> io::Result<()> {
    let lines = io::stdin().lock().lines().map(|r| r.unwrap()).collect();

    let p1: u64 = common_prio(&lines, lines_to_vecs_p1);
    println!("The sum of priorities by rucksack: {}", p1);

    let p2: u64 = common_prio(&lines, lines_to_vecs_p2);
    println!("The sum of priorities by Elf trio: {}", p2);

    Ok(())
}
