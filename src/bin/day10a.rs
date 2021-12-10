fn score(s: &str) -> u32 {
    let mut stack = Vec::new();

    for c in s.chars() {
        let mut is_paired = |r| stack.pop().map(|l| l == r).unwrap_or(false);
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if !is_paired('(') {
                    return 3;
                }
            }
            ']' => {
                if !is_paired('[') {
                    return 57;
                }
            }
            '}' => {
                if !is_paired('{') {
                    return 1197;
                }
            }
            '>' => {
                if !is_paired('<') {
                    return 25137;
                }
            }
            _ => unreachable!("Invalid input"),
        }
    }

    0
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day10.txt")?;

    let total_score: u32 = input.lines().map(|l| score(l)).sum();

    println!("{}", total_score);
    Ok(())
}
