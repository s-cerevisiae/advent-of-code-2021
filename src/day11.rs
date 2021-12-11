use std::collections::HashSet;

use itertools::iproduct;

pub type Pos = (usize, usize);
pub type Grid = Vec<Vec<u32>>;

pub fn step(grid: &mut Grid) -> u32 {
    let mut flash_count = 0;

    grid.iter_mut().flatten().for_each(|x| *x += 1);

    let mut flashed = HashSet::new();
    for p in iproduct!(0..10, 0..10) {
        chain_flash(p, grid, &mut flashed);
    }

    for x in grid.iter_mut().flatten() {
        if *x > 9 {
            flash_count += 1;
            *x = 0;
        }
    }

    flash_count
}

fn adjacent(x: usize, y: usize) -> Vec<Pos> {
    let x0 = x as isize;
    let y0 = y as isize;

    let xs = [x0, x0 + 1, x0 - 1];
    let ys = [y0, y0 + 1, y0 - 1];

    iproduct!(xs, ys)
        .filter(|(x, y)| (0..10).contains(x) && (0..10).contains(y) && (*x, *y) != (x0, y0))
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn chain_flash((x, y): Pos, grid: &mut Grid, flashed: &mut HashSet<Pos>) {
    if grid[y][x] > 9 && flashed.insert((x, y)) {
        for ap @ (ax, ay) in adjacent(x, y) {
            grid[ay][ax] += 1;
            if !flashed.contains(&ap) {
                chain_flash(ap, grid, flashed);
            }
        }
    }
}
