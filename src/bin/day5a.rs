use std::fs;

use advent_of_code_2021::day5::Line;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day5.txt")?;

    let lines: Vec<Line> = input.lines().map(|l| l.parse()).try_collect()?;

    let lines = lines
        .into_iter()
        .filter(|l| l.is_horizontal_or_vertical())
        .collect_vec();

    let xmax = lines.iter().map(|l| l.xmax()).max().unwrap();
    let ymax = lines.iter().map(|l| l.ymax()).max().unwrap();

    let mut board = vec![vec![0; xmax + 1]; ymax + 1];

    for l in lines {
        for (x, y) in l.points() {
            board[y][x] += 1;
        }
    }

    let result = board.into_iter().flatten().filter(|&x| x > 1).count();
    println!("{}", result);

    Ok(())
}
