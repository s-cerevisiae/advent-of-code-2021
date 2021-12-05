use std::str::FromStr;

use anyhow::Context;

type Pos = (usize, usize);

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
    Others,
}

use Direction::*;

#[derive(Debug)]
pub struct Line {
    start: Pos,
    end: Pos,
    direction: Direction,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" -> ").context("Invalid line")?;

        let parse_tuple = |t: &str| -> anyhow::Result<Pos> {
            let (x, y) = t.split_once(',').context("Invalid endpoints")?;
            Ok((x.parse()?, y.parse()?))
        };

        let (x1, y1) = parse_tuple(a)?;
        let (x2, y2) = parse_tuple(b)?;

        let abs_diff = |x, y| (x as i32 - y as i32).abs();
        let direction = if x1 == x2 {
            Horizontal
        } else if y1 == y2 {
            Vertical
        } else if abs_diff(x1, x2) == abs_diff(y1, y2) {
            Diagonal
        } else {
            Others
        };

        let line = Line {
            start: (x1, y1),
            end: (x2, y2),
            direction,
        };

        Ok(line)
    }
}

impl Line {
    pub fn is_horizontal_or_vertical(&self) -> bool {
        matches!(self.direction, Horizontal | Vertical)
    }

    pub fn is_diagonal(&self) -> bool {
        matches!(self.direction, Diagonal)
    }

    pub fn xmax(&self) -> usize {
        self.start.0.max(self.end.0)
    }

    pub fn ymax(&self) -> usize {
        self.start.1.max(self.end.1)
    }

    pub fn points(&self) -> Vec<(usize, usize)> {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;

        let towards = |x, y| if x < y { x..=y } else { y..=x };
        let downright = || (x1 as i32 - x2 as i32).signum() == (y1 as i32 - y2 as i32).signum();

        match self.direction {
            Horizontal => towards(y1, y2).map(|y| (x1, y)).collect(),
            Vertical => towards(x1, x2).map(|x| (x, y1)).collect(),
            Diagonal => {
                if downright() {
                    towards(x1, x2).zip(towards(y1, y2)).collect()
                } else {
                    towards(x1, x2).zip(towards(y1, y2).rev()).collect()
                }
            }
            Others => panic!("Not supposed to do this"),
        }
    }
}
