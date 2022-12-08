use anyhow::Result;
use take_until::TakeUntilExt;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/day08/input.txt")?;

    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    println!("Part 1: {}", count_visible_trees(&grid));
    println!("Part 2: {}", best_scenic_score(&grid));
    Ok(())
}

fn count_visible_trees(grid: &Vec<Vec<usize>>) -> usize {
    let mut visible_trees = 0;

    let height = grid.len();
    let width = grid[0].len();

    for x in 0..height {
        for y in 0..width {
            let current_tree = grid[x][y];
            if (0..x).all(|x1| grid[x1][y] < current_tree)
                || ((x + 1)..height).all(|x1| grid[x1][y] < current_tree)
                || (0..y).all(|y1| grid[x][y1] < current_tree)
                || ((y + 1)..width).all(|y1| grid[x][y1] < current_tree)
            {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn best_scenic_score(grid: &Vec<Vec<usize>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut best_score = 0;

    for x in 1..height {
        for y in 1..width {
            best_score = scenic_score(grid, x, y).max(best_score);
        }
    }
    best_score
}

fn scenic_score(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let tree = grid[x][y];

    let height = grid.len();
    let width = grid[0].len();

    let up_score = (0..x).rev().take_while(|&x1| grid[x1][y] < tree).count();

    let down_score = ((x + 1)..height)
        .take_until(|&x1| grid[x1][y] >= tree)
        .count();

    let left_score = (0..y).rev().take_until(|&y1| grid[x][y1] >= tree).count();

    let right_score = ((y + 1)..width)
        .take_until(|&y1| grid[x][y1] >= tree)
        .count();

    up_score * down_score * left_score * right_score
}
