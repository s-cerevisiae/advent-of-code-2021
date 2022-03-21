use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent_of_code_2021::{
    day15::*,
    utils::abs_diff,
};

#[derive(PartialEq, Eq)]
struct Vertex {
    pos: Pos,
    dist: u32,
    esti: u32,
}

impl Vertex {
    fn new(pos: Pos, dist: u32, esti: u32) -> Self {
        Self { pos, dist, esti }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let other_f = other.dist + other.esti;
        let self_f = self.dist + self.esti;
        other_f.cmp(&self_f)
    }
}

fn astar(start: Pos, target: Pos, grid: &Grid) -> Option<u32> {
    let esti = |pos: Pos| {
        let x_dist = abs_diff(pos.0, target.0);
        let y_dist = abs_diff(pos.1, target.1);
        x_dist + y_dist
    } as u32;

    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut visited = HashSet::new();

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Vertex::new(start, 0, esti(start)));

    while let Some(Vertex {
        pos: cur_pos,
        dist: cur_dist,
        ..
    }) = to_visit.pop()
    {
        if cur_pos == target {
            return Some(cur_dist);
        }
        if !visited.insert(cur_pos) {
            continue;
        }
        for (nb, cost) in neighbors(grid, cur_pos) {
            let new_dist = cost + cur_dist;
            let is_shorter = distances
                .get(&nb)
                .map_or(true, |&old_dist| new_dist < old_dist);
            if is_shorter {
                distances.insert(nb, new_dist);
                to_visit.push(Vertex::new(nb, new_dist, esti(nb)));
            }
        }
    }

    None
}

fn expand_grid(grid: &Grid) -> Grid {
    let (w, h) = (grid[0].len(), grid.len());
    let wrap_add = |x, y| {
        let r = x + y;
        if r > 9 {
            r - 9
        } else {
            r
        }
    };
    let mut expanded = vec![vec![0; 5 * w]; 5 * h];
    for y in 0..5 * h {
        for x in 0..5 * w {
            expanded[y][x] = wrap_add(grid[y % h][x % w], (x / w + y / h) as u32);
        }
    }

    expanded
}

fn main() {
    let grid = grid(INPUT);
    let full_grid = expand_grid(&grid);
    let target = (full_grid[0].len() - 1, full_grid.len() - 1);
    let result = astar((0, 0), target, &full_grid);
    println!("{result:?}");
}
