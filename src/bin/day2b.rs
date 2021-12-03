use std::{fs, str::FromStr};

use anyhow::anyhow;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, dist) = s.split_once(' ').ok_or(anyhow!("Invalid Command Syntax"))?;

        let command = match cmd {
            "forward" => Command::Forward,
            "down" => Command::Down,
            "up" => Command::Up,
            _ => Err(anyhow!("Invalid Command Name"))?
        }(dist.parse()?);

        Ok(command)
    }
}
struct Position {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            aim: 0,
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
                Command::Forward(d) => {
                    pos.horizontal += d;
                    pos.depth += pos.aim * d;
                },
                Command::Down(d) => pos.aim += d,
                Command::Up(d) => pos.aim -= d,
            }
            pos
        });

    println!("{}", result.horizontal * result.depth);
    Ok(())
}
