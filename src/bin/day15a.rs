use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent_of_code_2021::day15::*;

struct Vertex {
    pos: Pos,
    dist: u32,
}

impl Vertex {
    fn new(pos: Pos, dist: u32) -> Self {
        Self { pos, dist }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for Vertex {}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

fn dijkstra(start: Pos, target: Pos, grid: &Grid) -> Option<u32> {
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut visited = HashSet::new();

    let mut pq = BinaryHeap::new();
    pq.push(Vertex::new(start, 0));

    while let Some(Vertex {
        pos: cur_pos,
        dist: cur_dist,
    }) = pq.pop()
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
                pq.push(Vertex::new(nb, new_dist));
            }
        }
    }

    None
}

fn main() {
    let grid = grid(INPUT);

    let target = (grid[0].len() - 1, grid.len() - 1);
    let result = dijkstra((0, 0), target, &grid);
    println!("{result:?}");
}
