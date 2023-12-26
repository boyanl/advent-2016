use std::{io::stdin, result, vec};

use sscanf::sscanf;
use Command::*;

#[derive(Clone, Copy)]
enum Command {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

fn read_input() -> Vec<Command> {
    let mut commands = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if let Ok((a, b)) = sscanf!(&line, "rect {usize}x{usize}") {
            commands.push(Command::Rect(a, b));
        } else if let Ok((row, amount)) = sscanf!(&line, "rotate row y={usize} by {usize}") {
            commands.push(Command::RotateRow(row, amount));
        } else if let Ok((col, amount)) = sscanf!(&line, "rotate column x={usize} by {usize}") {
            commands.push(Command::RotateColumn(col, amount));
        }
    }

    commands
}

fn fill_rect(grid: &mut Vec<Vec<i32>>, a: usize, b: usize) {
    for i in 0..b {
        for j in 0..a {
            grid[i][j] = 1;
        }
    }
}

fn rotate_row(grid: &mut Vec<Vec<i32>>, row: usize, amount: usize) {
    let m = grid[row].len();
    let mut new_row = vec![0; m];
    for j in 0..m {
        new_row[(j + amount) % m] = grid[row][j]
    }

    grid[row] = new_row;
}

fn rotate_col(grid: &mut Vec<Vec<i32>>, col: usize, amount: usize) {
    let n = grid.len();
    let mut new_col = vec![0; n];
    for i in 0..n {
        new_col[(i + amount) % n] = grid[i][col];
    }

    for i in 0..n {
        grid[i][col] = new_col[i];
    }
}

fn visualize_grid(grid: &Vec<Vec<i32>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = if grid[i][j] == 0 { ' ' } else { '█' };
            print!("{c}");
        }
        println!();
    }
}

fn simulate(grid_size: (usize, usize), commands: &Vec<Command>) -> Vec<Vec<i32>> {
    let (n, m) = grid_size;
    let mut grid = vec![vec![0; m]; n];

    for command in commands {
        match *command {
            Rect(a, b) => fill_rect(&mut grid, a, b),
            RotateRow(row, amount) => rotate_row(&mut grid, row, amount),
            RotateColumn(col, amount) => rotate_col(&mut grid, col, amount),
        }
    }

    grid
}

fn part_one() {
    let commands = read_input();
    let size = (6, 50);
    let grid = simulate(size, &commands);

    let result = grid
        .iter()
        .map(|row| row.iter().filter(|&x| *x == 1).count())
        .sum::<usize>();

    println!("{result}");
}

fn part_two() {
    let commands = read_input();
    let size = (6, 50);
    let grid = simulate(size, &commands);

    visualize_grid(&grid);
}

fn main() {
    part_two();
}
