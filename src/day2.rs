use std::str::FromStr;

use anyhow::anyhow;

pub enum Command {
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
