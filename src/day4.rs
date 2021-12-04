use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Board {
    inner: Vec<Vec<u32>>,
}

impl Board {
    pub fn is_winning(&self, drawn: &HashSet<u32>) -> bool {
        let is_drawn = |x| drawn.contains(x);

        let row = self.inner.iter().any(|row| row.iter().all(is_drawn));
        let column = (0..5).any(|i| (0..5).all(|j| is_drawn(&self.inner[j][i])));

        row || column
    }

    pub fn score(&self, drawn: &HashSet<u32>, just_called: u32) -> u32 {
        self.inner
            .iter()
            .flatten()
            .filter(|x| !drawn.contains(x))
            .sum::<u32>()
            * just_called
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Vec<Vec<u32>> = s
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse::<u32>())
                    .try_collect()
            })
            .try_collect()?;
        Ok(Board { inner })
    }
}
