use std::collections::HashSet;

use advent_of_code_2021::day12::{Edge, Map, is_big};
use anyhow::Context;
use itertools::Itertools;

fn count_paths(graph: &Map) -> usize {
    let mut visited = HashSet::new();
    visited.insert("start");
    traverse("start", &graph, &mut visited)
}

fn traverse<'n>(node: &'n str, graph: &'n Map, visited: &mut HashSet<&'n str>) -> usize {
    let mut count = 0;

    for n in graph.neighbors(node) {
        if n == "end" {
            count += 1;
        } else if is_big(n) || visited.insert(n) {
            count += traverse(n, graph, visited);
            visited.remove(n);
        }
    }

    count
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day12.txt")?;

    let edges: Vec<Edge> = input
        .lines()
        .map(|l| l.split_once('-').context("Invalid edge"))
        .try_collect()?;
    let graph = Map::from_edges(edges);

    let paths = count_paths(&graph);

    println!("{:?}", paths);
    Ok(())
}
