use std::fs;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day6.txt")?;

    let mut c = [0; 9];
    input
        .trim()
        .split(',')
        .filter_map(|d| d.parse().ok())
        .for_each(|d: usize| c[d] += 1);

    for _ in 0..256 {
        c.rotate_left(1);
        c[6] += c[8];
    }

    println!("{:?}", c.into_iter().sum::<u128>());

    Ok(())
}
