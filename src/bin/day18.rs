use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

fn min<T: Ord + Copy>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn max<T: Ord + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn minmax<T: Ord + Copy>((mm_min, mm_max): (T, T), a: T) -> (T, T) {
    (min(mm_min, a), max(mm_max, a))
}

type Voxel = (i64, i64, i64);

fn parse_voxel(line: &str) -> Voxel {
    let mut split_line = line.split(',');
    let x = split_line.next().unwrap().parse().unwrap();
    let y = split_line.next().unwrap().parse().unwrap();
    let z = split_line.next().unwrap().parse().unwrap();
    (x, y, z)
}

fn parse_voxels(input: &str) -> Vec<Voxel> {
    input
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(parse_voxel)
        .collect()
}

#[derive(Debug)]
struct VoxelGraph {
    vertices: HashSet<Voxel>,
    edges: HashMap<Voxel, Vec<Voxel>>,
    bounding_box: (Voxel, Voxel),
}

impl VoxelGraph {
    fn get_bounding_box(vertices: &Vec<Voxel>) -> (Voxel, Voxel) {
        let (x, y, z) = vertices[0];
        let mut mmx = (x, x);
        let mut mmy = (y, y);
        let mut mmz = (z, z);
        for voxel in vertices.iter() {
            let (x, y, z) = *voxel;
            mmx = minmax(mmx, x);
            mmy = minmax(mmy, y);
            mmz = minmax(mmz, z);
        }
        (
            (mmx.0 - 1, mmy.0 - 1, mmz.0 - 1),
            (mmx.1 + 1, mmy.1 + 1, mmz.1 + 1),
        )
    }

    fn from(vertices: Vec<Voxel>) -> Self {
        let mut edges: HashMap<Voxel, Vec<Voxel>> = HashMap::new();

        let bounding_box = Self::get_bounding_box(&vertices);

        let vertices = HashSet::from_iter(vertices.into_iter());
        for (x, y, z) in vertices.iter() {
            let neighbors = [
                (1, 0, 0),
                (0, 1, 0),
                (0, 0, 1),
                (-1, 0, 0),
                (0, -1, 0),
                (0, 0, -1),
            ];
            let mut edges_current = Vec::new();
            for (p, q, r) in neighbors.iter() {
                let neighbor = (x + p, y + q, z + r);
                if vertices.contains(&neighbor) {
                    edges_current.push(neighbor);
                }
            }
            edges.insert((*x, *y, *z), edges_current);
        }

        VoxelGraph {
            vertices,
            edges,
            bounding_box,
        }
    }

    fn in_bounding_box(&self, (x, y, z): Voxel) -> bool {
        let (p, q, r) = self.bounding_box.0;
        if x < p || y < q || z < r {
            return false;
        }
        let (p, q, r) = self.bounding_box.1;
        if x > p || y > q || z > r {
            return false;
        }
        true
    }

    fn outer_surface_area(&self) -> usize {
        let mut side_count = 0;
        let mut visited: HashSet<Voxel> = HashSet::new();
        let mut queue = vec![self.bounding_box.0];
        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            let (x, y, z) = current;
            let neighbors = [
                (1, 0, 0),
                (0, 1, 0),
                (0, 0, 1),
                (-1, 0, 0),
                (0, -1, 0),
                (0, 0, -1),
            ];
            for (p, q, r) in neighbors.iter() {
                let neighbor = (x + p, y + q, z + r);
                if !self.in_bounding_box(neighbor) || visited.contains(&neighbor) {
                    continue;
                }
                if self.vertices.contains(&neighbor) {
                    side_count += 1;
                } else {
                    queue.push(neighbor);
                }
            }
            visited.insert(current);
        }
        side_count
    }

    fn surface_area(&self) -> usize {
        6 * self.vertices.len() - self.count_edges()
    }

    fn count_edges(&self) -> usize {
        self.edges.iter().map(|v| v.1.len()).sum()
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input = parse_voxels(&input[..]);
    let graph = VoxelGraph::from(input.clone());

    println!("{:?}", graph.surface_area());
    println!("{:?}", graph.outer_surface_area());

    Ok(())
}
