pub static INPUT: &str = include_str!("../input/day15.txt");
pub static TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

pub type Grid = Vec<Vec<u32>>;
pub type Pos = (usize, usize);

pub fn grid(input: &str) -> Grid {
    let mut grid = Vec::new();

    for l in input.lines() {
        let row = l
            .chars()
            .map(|d| d.to_digit(10).expect("dec digit"))
            .collect();
        grid.push(row);
    }

    grid
}

pub fn neighbors(grid: &Grid, (x, y): Pos) -> Vec<(Pos, u32)> {
    [
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
        (x + 1, y),
        (x, y + 1),
    ]
    .into_iter()
    .filter_map(|p @ (x, y)| {
        grid.get(y)
            .and_then(|row| row.get(x))
            .copied()
            .map(|cost| (p, cost))
    })
    .collect()
}
