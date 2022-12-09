use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: i32,
    visible: bool,
}

impl Tree {
    fn new(height: i32) -> Self {
        Tree {
            height,
            visible: false,
        }
    }

    fn mark_visibilities<'a, T: Iterator<Item = &'a mut Tree>>(iterator: T) {
        let mut max_height = -1;
        for mut tree in iterator {
            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }
}

fn char_to_height(c: char) -> i32 {
    c.to_digit(10).unwrap().try_into().unwrap()
}

fn line_to_vec_of_trees(line: &str) -> Vec<Tree> {
    line.chars().map(|c| Tree::new(char_to_height(c))).collect()
}

fn rotate_square(square: Vec<Vec<Tree>>, reverse: bool) -> Vec<Vec<Tree>> {
    let l = square.len();
    let mut new_square = Vec::new();

    for y in 0..l {
        let mut new_row = Vec::with_capacity(l);
        for x in 0..l {
            if reverse {
                new_row.push(square[x][(l - 1) - y]);
            } else {
                new_row.push(square[(l - 1) - x][y]);
            }
        }
        new_square.push(new_row);
    }

    new_square
}

fn count_square_visibility(square: &Vec<Vec<Tree>>) -> i32 {
    let visible_in_row = |row: &Vec<Tree>| row.iter().filter(|t| t.visible).count() as i32;
    square.iter().map(visible_in_row).sum()
}

fn mark_square_visibility(mut square: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    for row in square.iter_mut() {
        Tree::mark_visibilities(row.iter_mut());
        Tree::mark_visibilities(row.iter_mut().rev());
    }
    square = rotate_square(square, false);
    for row in square.iter_mut() {
        Tree::mark_visibilities(row.iter_mut());
        Tree::mark_visibilities(row.iter_mut().rev());
    }
    rotate_square(square, true)
}

fn get_viewing_distance(square: &Vec<Vec<Tree>>, origin: (i32, i32), d: (i32, i32)) -> i32 {
    let (mut x, mut y) = origin;
    let l = square.len() as i32;
    let threshold = square[y as usize][x as usize].height;
    let (dx, dy) = d;

    let mut distance = 0;
    loop {
        x += dx;
        y += dy;
        if x < 0 || x >= l || y < 0 || y >= l {
            break;
        }
        distance += 1;
        let height = square[y as usize][x as usize].height;
        if height >= threshold {
            break;
        }
    }
    distance
}

fn get_scenic_score(square: &Vec<Vec<Tree>>, origin: (i32, i32)) -> i32 {
    let mut score = 1;
    for d in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
        score *= get_viewing_distance(square, origin, *d);
    }
    score
}

fn get_max_scenic_score(square: &Vec<Vec<Tree>>) -> i32 {
    let mut max_score = 0;
    let l = square.len();
    for y in 0..l {
        for x in 0..l {
            let score = get_scenic_score(square, (x as i32, y as i32));
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input: Vec<Vec<Tree>> = input.lines().map(line_to_vec_of_trees).collect();
    let input = mark_square_visibility(input);

    let num_visible = count_square_visibility(&input);
    println!("The number of visible trees in the grid: {}", num_visible);

    let max_score = get_max_scenic_score(&input);
    println!("The best possible scenic score in the grid: {}", max_score);

    Ok(())
}
