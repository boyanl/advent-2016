mod util;
use std::io::stdin;

use util::vec2::*;

fn get_code(grid: &Vec<Vec<char>>, directions: &Vec<String>, start: Point2) -> String {
    let mut current = start;
    let mut code = String::new();

    for line in directions {
        for dir_c in line.chars() {
            let next = current
                + match dir_c {
                    'U' => UP,
                    'D' => DOWN,
                    'L' => LEFT,
                    'R' => RIGHT,
                    _ => todo!(),
                };

            if next.y >= 0
                && next.y < grid.len() as i32
                && next.x >= 0
                && next.x < grid[next.y as usize].len() as i32
                && grid[next.y as usize][next.x as usize] != ' '
            {
                current = next;
            }
        }
        code.push(grid[current.y as usize][current.x as usize]);
    }

    code
}

fn part_one() {
    let grid = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    let instructions = stdin().lines().map(|l| l.unwrap()).collect();

    let result = get_code(&grid, &instructions, Point2 { x: 1, y: 1 });
    println!("{result}");
}

fn part_two() {
    let grid = vec![
        vec![' ', ' ', '1', ' ', ' '],
        vec![' ', '2', '3', '4', ' '],
        vec!['5', '6', '7', '8', '9'],
        vec![' ', 'A', 'B', 'C', ' '],
        vec![' ', ' ', 'D', ' ', ' '],
    ];
    let instructions = stdin().lines().map(|l| l.unwrap()).collect();

    let result = get_code(&grid, &instructions, Point2 { x: 0, y: 2 });
    println!("{result}");
}

fn main() {
    part_two();
}
