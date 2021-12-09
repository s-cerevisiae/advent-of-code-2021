use std::fs;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day8.txt")?;

    let count: u32 = input
        .lines()
        .map(|l| {
            l.split_once('|')
                .unwrap()
                .1
                .trim()
                .split_ascii_whitespace()
                .map(|s| match s.len() {
                    2 | 4 | 3 | 7 => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum();

    println!("{}", count);

    Ok(())
}
