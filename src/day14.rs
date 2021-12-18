use std::str::FromStr;

use anyhow::Context;
use defaultmap::DefaultHashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct Polymer {
    pairs: DefaultHashMap<(char, char), usize>,
    pub elements: DefaultHashMap<char, usize>,
}

impl From<&str> for Polymer {
    fn from(s: &str) -> Self {
        let mut pairs = DefaultHashMap::new(0);
        for p in s.chars().tuple_windows() {
            pairs[p] += 1;
        }

        let mut elements = DefaultHashMap::new(0);
        for e in s.chars() {
            elements[e] += 1;
        }

        Self { pairs, elements }
    }
}

#[derive(Debug)]
pub struct Rule {
    from: (char, char),
    to: char,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once(" -> ").context("Invalid rule")?;
        let from = from.chars().collect_tuple().context("Invalid rule lhs")?;
        let to = to.chars().next().context("Invalid rule rhs")?;

        Ok(Self { from, to })
    }
}

pub fn step(polymer: &mut Polymer, rules: &[Rule]) {
    let mut new_pairs = DefaultHashMap::new(0);
    for &Rule { from, to } in rules {
        if polymer.pairs.contains_key(&from) {
            let times = polymer.pairs[from];
            polymer.elements[to] += times;
            new_pairs[(from.0, to)] += times;
            new_pairs[(to, from.1)] += times;
        }
    }
    polymer.pairs = new_pairs
}
