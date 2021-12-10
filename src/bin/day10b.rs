use itertools::Itertools;

fn remaining_chars(s: &str) -> Option<Vec<char>> {
    let is_pair = |x, y| matches!((x, y), ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>'));

    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                if !stack.pop().map(|l| is_pair(l, c)).unwrap_or(false) {
                    return None;
                }
            }
        }
    }

    Some(stack)
}

fn completion_score(s: &[char]) -> u64 {
    let mut score = 0;

    for c in s.iter().rev() {
        score *= 5;
        score += match *c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        };
    }

    score
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day10.txt")?;

    let mut scores = input
        .lines()
        .filter_map(|l| remaining_chars(l))
        .map(|s| completion_score(&s))
        .collect_vec();

    scores.sort_unstable();

    let mid_score = scores[scores.len() / 2];

    println!("{}", mid_score);
    Ok(())
}
