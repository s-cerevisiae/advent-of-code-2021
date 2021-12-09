type Pos = (usize, usize);

fn surroundings(v: &[Vec<u32>], (x, y): Pos, (h, w): Pos) -> Vec<u32> {
    let mut s = Vec::new();
    if 0 < x {
        s.push(v[y][x - 1]);
    }
    if 0 < y {
        s.push(v[y - 1][x])
    }
    if x < w - 1 {
        s.push(v[y][x + 1])
    }
    if y < h - 1 {
        s.push(v[y + 1][x])
    }
    s
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day9.txt")?;

    let heightmap: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let h = heightmap.len();
    let w = heightmap.first().map_or(0, |r| r.len());

    let mut risk_level = 0;
    for y in 0..h {
        for x in 0..w {
            let p = heightmap[y][x];
            let is_lowpoint = surroundings(&heightmap, (x, y), (h, w))
                .into_iter()
                .all(|v| v > p);
            if is_lowpoint {
                risk_level += p + 1;
            }
        }
    }

    println!("{:?}", risk_level);
    Ok(())
}
