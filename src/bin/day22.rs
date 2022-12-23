use std::io;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, value};
use nom::multi::many1;
use nom::IResult;

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum TurnDir {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Forward(u64),
    Turn(TurnDir),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, turn_dir: TurnDir) -> Self {
        match turn_dir {
            TurnDir::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            TurnDir::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
        }
    }

    fn facing(&self) -> u64 {
        match self {
            Direction::Up => 3,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
        }
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Position {
    fn warp_edge_part_two((x, y): Pos, dir: Direction) -> (Pos, Direction) {
        if x >= 100 && y == 50 {
            // B to C
            let (x, y) = (99, x - 50);
            ((x, y), dir.turn(TurnDir::Right))
        } else if x == 100 && y >= 50 && y < 100 {
            // C to B
            let (x, y) = (y + 50, 49);
            ((x, y), dir.turn(TurnDir::Left))
        } else if x == 0 && y < 50 {
            // B to E
            let dir = dir.turn(TurnDir::Right).turn(TurnDir::Right);
            ((99, 149 - y), dir)
        } else if x == 100 && y >= 100 && y < 150 {
            // E to B
            let dir = dir.turn(TurnDir::Right).turn(TurnDir::Right);
            ((149, 149 - y), dir)
        } else if x >= 100 && y == 199 {
            // B to F
            ((x - 100, y), dir)
        } else if x < 50 && y == 0 {
            // F to B
            ((x + 100, y), dir)
        } else if x >= 50 && x < 100 && y == 199 {
            // A to F
            let (x, y) = (0, x + 100);
            ((x, y), dir.turn(TurnDir::Right))
        } else if x == 149 && y >= 150 {
            // F to A
            let (x, y) = (y - 100, 0);
            ((x, y), dir.turn(TurnDir::Left))
        } else if x == 49 && y < 50 {
            // A to D
            ((0, 149 - y), dir.turn(TurnDir::Right).turn(TurnDir::Right))
        } else if x == 149 && y >= 100 && y < 150 {
            // D to A
            ((50, 149 - y), dir.turn(TurnDir::Right).turn(TurnDir::Right))
        } else if x == 49 && y >= 50 && y < 100 {
            // C to D
            let (x, y) = (y - 50, 100);
            ((x, y), dir.turn(TurnDir::Left))
        } else if x < 50 && y == 99 {
            // D to C
            let (x, y) = (50, x + 50);
            ((x, y), dir.turn(TurnDir::Right))
        } else if x >= 50 && x < 100 && y == 150 {
            // E to F
            let (x, y) = (49, x + 100);
            ((x, y), dir.turn(TurnDir::Right))
        } else if x == 50 && y >= 150 {
            // F to E
            let (x, y) = (y - 100, 149);
            ((x, y), dir.turn(TurnDir::Left))
        } else {
            ((x, y), dir)
        }
    }

    fn next_cell_forward(&self, grid: &Grid, (x, y): Pos) -> Pos {
        let width = grid[y].len();
        let height = grid.len();

        let p = x + width;
        let q = y + height;

        let (p, q) = match self.dir {
            Direction::Up => (p, q - 1),
            Direction::Right => (p + 1, q),
            Direction::Down => (p, q + 1),
            Direction::Left => (p - 1, q),
        };

        (p % width, q % height)
    }

    fn traverse_grid(&self, grid: &Grid, part_two: bool) -> (Pos, Direction) {
        let (mut p, mut q) = self.next_cell_forward(&grid, (self.x, self.y));
        let mut dir = self.dir;
        if grid[q][p] == ' ' {
            if part_two {
                ((p, q), dir) = Self::warp_edge_part_two((p, q), dir);
            } else {
                while grid[q][p] == ' ' {
                    (p, q) = self.next_cell_forward(&grid, (p, q));
                }
            }
        }
        ((p, q), dir)
    }

    fn follow_instr(&mut self, grid: &mut Grid, instr: Instruction, part_two: bool) {
        match instr {
            Instruction::Forward(mut n) => {
                while n > 0 {
                    let ((p, q), dir) = self.traverse_grid(&grid, part_two);
                    if grid[q][p] == '#' {
                        break;
                    }

                    self.x = p;
                    self.y = q;
                    self.dir = dir;

                    n -= 1;
                }
            }
            Instruction::Turn(turn_dir) => {
                self.dir = self.dir.turn(turn_dir);
            }
        }
    }

    fn password(&self) -> u64 {
        let row = (self.y + 1) as u64;
        let col = (self.x + 1) as u64;
        let facing = self.dir.facing();
        row * 1000 + col * 4 + facing
    }
}

fn parse_turn_left(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Turn(TurnDir::Left), tag("L"))(input)
}

fn parse_turn_right(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Turn(TurnDir::Right), tag("R"))(input)
}

fn parse_forward(input: &str) -> IResult<&str, Instruction> {
    let wrap = |n: &str| Instruction::Forward(n.parse().unwrap());
    map(digit1, wrap)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_turn_left, parse_turn_right, parse_forward))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_instruction)(input)
}

fn initial_position(grid: &Grid) -> Position {
    let mut x = 0;
    while grid[0][x] != '.' {
        x += 1;
    }
    Position {
        x,
        y: 0,
        dir: Direction::Right,
    }
}

fn parse_input(input: String) -> (Grid, Vec<Instruction>) {
    let mut grid: Grid = Vec::new();
    let mut instructions_index = 0;
    let input: Vec<&str> = input.lines().collect();
    for (i, line) in input.iter().enumerate() {
        if line.len() == 0 {
            instructions_index = i + 1;
            break;
        }
        grid.push(line.chars().collect());
    }
    let instr_line = input[instructions_index];
    let (_, instructions) = parse_instructions(instr_line).unwrap();
    (grid, instructions)
}

fn pad(mut grid: Grid) -> Grid {
    let width: usize = grid.iter().map(|v| v.len()).max().unwrap();
    for row in grid.iter_mut() {
        if row.len() < width {
            row.append(&mut vec![' '; width - row.len()]);
        }
    }
    grid
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (grid, instructions) = parse_input(input);
    let mut grid = pad(grid);

    let mut position = initial_position(&grid);
    for instr in instructions.iter() {
        position.follow_instr(&mut grid, *instr, false);
    }
    println!("The password when using the map: {}", position.password());

    let mut position = initial_position(&grid);
    for instr in instructions.iter() {
        position.follow_instr(&mut grid, *instr, true);
    }
    println!("The password when using the cube: {}", position.password());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_a_d() {
        assert_eq!(
            Position::warp_edge_part_two((49, 10), Direction::Left),
            ((0, 139), Direction::Right)
        );
        assert_eq!(
            Position::warp_edge_part_two((149, 139), Direction::Left),
            ((50, 10), Direction::Right)
        );
    }

    #[test]
    fn test_a_f() {
        assert_eq!(
            Position::warp_edge_part_two((149, 160), Direction::Left),
            ((60, 0), Direction::Down)
        );
        assert_eq!(
            Position::warp_edge_part_two((60, 199), Direction::Up),
            ((0, 160), Direction::Right)
        );
    }

    #[test]
    fn test_b_c() {
        assert_eq!(
            Position::warp_edge_part_two((110, 50), Direction::Down),
            ((99, 60), Direction::Left)
        );
        assert_eq!(
            Position::warp_edge_part_two((100, 60), Direction::Right),
            ((110, 49), Direction::Up)
        );
    }

    #[test]
    fn test_b_e() {
        assert_eq!(
            Position::warp_edge_part_two((0, 10), Direction::Right),
            ((99, 139), Direction::Left)
        );
        assert_eq!(
            Position::warp_edge_part_two((100, 110), Direction::Right),
            ((149, 39), Direction::Left)
        );
    }

    #[test]
    fn test_b_f() {
        assert_eq!(
            Position::warp_edge_part_two((110, 199), Direction::Up),
            ((10, 199), Direction::Up)
        );
        assert_eq!(
            Position::warp_edge_part_two((10, 0), Direction::Up),
            ((110, 0), Direction::Up)
        );
    }

    #[test]
    fn test_c_d() {
        assert_eq!(
            Position::warp_edge_part_two((49, 60), Direction::Left),
            ((10, 100), Direction::Down)
        );
        assert_eq!(
            Position::warp_edge_part_two((10, 99), Direction::Up),
            ((50, 60), Direction::Right)
        );
    }

    #[test]
    fn test_e_f() {
        assert_eq!(
            Position::warp_edge_part_two((60, 150), Direction::Down),
            ((49, 160), Direction::Left)
        );
        assert_eq!(
            Position::warp_edge_part_two((50, 160), Direction::Right),
            ((60, 149), Direction::Up)
        );
    }
}
