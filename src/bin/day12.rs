use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io;
use std::io::prelude::*;

fn line_to_heights(line: &&str) -> Vec<u8> {
    let map_byte = |b: &u8| match *b {
        b'S' => 0,
        b'E' => 25,
        _ => b - b'a',
    };
    line.as_bytes().iter().map(map_byte).collect()
}

type Position = (usize, usize);

struct HeightMap {
    begin: Position,
    end: Position,
    width: usize,
    height: usize,
    map: Vec<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DijkstraCandidate {
    cost: u64,
    pos: Position,
    via: Position,
}

impl Ord for DijkstraCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .reverse()
            .then(self.via.cmp(&other.via))
            .then(self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for DijkstraCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn minmax(coord: usize, dim: usize) -> (usize, usize) {
    let min = if coord == 0 { 0 } else { coord - 1 };
    let max = if coord >= dim - 1 { dim - 1 } else { coord + 1 };
    (min, max)
}

fn dedup(queue: BinaryHeap<DijkstraCandidate>) -> BinaryHeap<DijkstraCandidate> {
    let set: HashSet<&DijkstraCandidate> = HashSet::from_iter(queue.iter().to_owned());
    BinaryHeap::from_iter(set.iter().map(|&&c| c))
}

impl HeightMap {
    fn height_at(&self, pos: Position) -> u8 {
        self.map[pos.1][pos.0]
    }
    fn find_shortest_path(&self, from: Position, to: Position, part_two: bool) -> Vec<Position> {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        queue.push(DijkstraCandidate {
            pos: from,
            via: from,
            cost: 0,
        });
        let mut path = HashMap::new();
        let mut num_iter = 0;
        let dest;

        loop {
            let candidate = queue.pop().unwrap();

            path.insert(candidate.pos, candidate);
            visited.insert(candidate.pos);

            if (part_two && self.height_at(candidate.pos) == 0) || candidate.pos == to {
                dest = candidate.pos;
                break;
            }

            for neighbor in self.get_neighbors(candidate.pos).iter() {
                if visited.contains(neighbor) {
                    continue;
                }
                if self.height_at(candidate.pos) > self.height_at(*neighbor) {
                    if self.height_at(candidate.pos) - self.height_at(*neighbor) > 1 {
                        continue;
                    }
                }
                queue.push(DijkstraCandidate {
                    cost: candidate.cost + 1,
                    pos: *neighbor,
                    via: candidate.pos,
                });
            }
            num_iter += 1;
            if num_iter % 50 == 0 {
                queue = dedup(queue);
            }
        }

        let mut result = Vec::new();
        let mut current_candidate = path.get(&dest).unwrap();
        loop {
            result.push(current_candidate.pos);
            if current_candidate.pos == from {
                break;
            }
            current_candidate = path.get(&current_candidate.via).unwrap();
        }

        result
    }

    fn find_byte(lines: &Vec<&str>, needle: u8) -> Position {
        for (y, line) in lines.iter().enumerate() {
            for (x, &b) in line.as_bytes().iter().enumerate() {
                if b == needle {
                    return (x, y);
                }
            }
        }
        unreachable!()
    }

    fn get_neighbors(&self, p: Position) -> Vec<Position> {
        let mut result = Vec::new();
        let (minx, maxx) = minmax(p.0, self.width);
        let (miny, maxy) = minmax(p.1, self.height);

        for y in miny..=maxy {
            for x in minx..=maxx {
                if x == p.0 || y == p.1 {
                    if (x, y) != p {
                        result.push((x, y));
                    }
                }
            }
        }
        result
    }

    fn new(input: String) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let map: Vec<Vec<u8>> = lines.iter().map(line_to_heights).collect();
        let begin = Self::find_byte(&lines, b'S');
        let end = Self::find_byte(&lines, b'E');
        let width = map[0].len();
        let height = map.len();
        HeightMap {
            begin,
            end,
            map,
            width,
            height,
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let map = HeightMap::new(input);

    let trip1 = map.find_shortest_path(map.end, map.begin, false).len() - 1;
    println!("The shortest path from start to end: {}", trip1);

    let trip2 = map.find_shortest_path(map.end, map.begin, true).len() - 1;
    println!("The shortest path from any groundlevel cell to end: {}", trip2);
    Ok(())
}
