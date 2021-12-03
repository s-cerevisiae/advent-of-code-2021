use std::cmp::Ordering::*;

#[derive(Debug)]
pub struct Count {
    zeros: usize,
    ones: usize,
}

impl Count {
    pub fn at_digit(xs: &[u32], digit: u32) -> Count {
        let ones = xs.into_iter().filter(|&x| x & (1 << digit) != 0).count();
        let zeros = xs.len() - ones;
        Self { zeros, ones }
    }

    pub fn most_common(&self) -> u32 {
        match self.zeros.cmp(&self.ones) {
            Greater => 0,
            _ => 1,
        }
    }

    pub fn least_common(&self) -> u32 {
        match self.zeros.cmp(&self.ones) {
            Greater if self.ones > 0 => 1,
            _ => 0,
        }
    }
}
