use advent_of_code_2021::day13::{Instruction, Points};
use anyhow::Context;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day13.txt")?;

    let (coords, inst) = input.split_once("\n\n").context("Invalid input")?;

    let mut points = Points::new();
    for line in coords.lines() {
        let (x, y) = scan_fmt!(line, "{d},{d}", i32, i32)?;
        points.insert((x, y));
    }

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in inst.lines() {
        instructions.push(line.parse()?);
    }

    let folded = instructions
        .into_iter()
        .fold(points, |acc, inst| inst.fold_points(&acc));

    let (&xmin, &xmax) = folded
        .iter()
        .map(|(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (&ymin, &ymax) = folded
        .iter()
        .map(|(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if folded.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    Ok(())
}
