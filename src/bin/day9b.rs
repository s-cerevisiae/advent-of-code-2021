use std::collections::HashSet;

use itertools::Itertools;

type Pos = (usize, usize);

fn surroundings((x, y): Pos, (h, w): Pos) -> Vec<Pos> {
    let mut s = Vec::new();
    if 0 < x {
        s.push((x - 1, y));
    }
    if 0 < y {
        s.push((x, y - 1))
    }
    if x < w - 1 {
        s.push((x + 1, y))
    }
    if y < h - 1 {
        s.push((x, y + 1))
    }
    s
}

fn search_from(p: Pos, m: Pos, map: &[Vec<u32>], travelled: &mut HashSet<Pos>) -> u32 {
    let neighbors = surroundings(p, m);
    let mut size = 0;
    for np@(ny, nx) in neighbors {
        if !travelled.insert(np) {
            continue;
        }
        if map[ny][nx] < 9 {
            size += 1 + search_from(np, m, map, travelled);
        }
    }
    size
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day9.txt")?;

    let heightmap: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let h = heightmap.len();
    let w = heightmap.first().map_or(0, |r| r.len());

    let points = (0..w)
        .cartesian_product(0..h)
        .filter(|&(x, y)| heightmap[y][x] < 9);

    let mut basins = Vec::new();
    let mut travelled = HashSet::new();
    for p in points {
        if travelled.contains(&p) {
            continue;
        }

        let size = search_from(p, (h, w), &heightmap, &mut travelled);
        basins.push(size);
    }
    basins.sort_unstable();

    let result: u32 = basins.iter().rev().take(3).product();
    println!("{:?}", result);

    Ok(())
}
