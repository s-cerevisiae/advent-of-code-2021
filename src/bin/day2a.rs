use std::fs;

use advent_of_code_2021::day2::Command;

struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day2.txt")?;

    let result = input
        .lines()
        .filter_map(|l| l.parse::<Command>().ok())
        .fold(Position::new(), |mut pos, c| {
            match c {
                Command::Forward(d) => pos.horizontal += d,
                Command::Down(d) => pos.depth += d,
                Command::Up(d) => pos.depth -= d,
            }
            pos
        });

    println!("{}", result.horizontal * result.depth);
    Ok(())
}
