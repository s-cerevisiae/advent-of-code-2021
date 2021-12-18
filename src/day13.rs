use std::{str::FromStr, collections::HashSet};

use scan_fmt::scan_fmt;

pub type Point = (i32, i32);
pub type Points = HashSet<Point>;

pub enum Axis {
    X,
    Y,
}

pub struct Instruction {
    axis: Axis,
    pos: i32,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, pos) = scan_fmt!(s, "fold along {}={d}", String, i32)?;
        let axis = if a == "x" { Axis::X } else { Axis::Y };

        Ok(Self { axis, pos })
    }
}

impl Instruction {
    fn fold_point(&self, &(x, y): &Point) -> Point {
        let pos = self.pos;
        match self.axis {
            Axis::X => {
                if x < pos {
                    (x, y)
                } else {
                    (2 * pos - x, y)
                }
            }
            Axis::Y => {
                if y < pos {
                    (x, y)
                } else {
                    (x, 2 * pos - y)
                }
            }
        }
    }

    pub fn fold_points(&self, points: &Points) -> Points {
        let mut folded = Points::new();
        for p in points {
            folded.insert(self.fold_point(p));
        }
        folded
    }
}
