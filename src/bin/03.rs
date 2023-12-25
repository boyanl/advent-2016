use std::{arch::x86_64::_MM_SET_ROUNDING_MODE, fs::set_permissions, io::stdin};

use itertools::Itertools;

pub fn sort<T: Eq + Ord>(tpl: (T, T, T)) -> (T, T, T) {
    let (a, b, c) = tpl;
    let mut v = [a, b, c];
    v.sort();
    let [a, b, c] = v;
    (a, b, c)
}

fn parse_input() -> Vec<Vec<i32>> {
    stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_ascii_whitespace()
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_one() {
    let mut result = 0;
    let numbers = parse_input();

    for row in &numbers {
        if row.len() == 3 {
            let (a, b, c) = sort((row[0], row[1], row[2]));
            if a + b > c {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn part_two() {
    let mut result = 0;
    let numbers = parse_input();

    for i in (0..numbers.len() - 2).step_by(3) {
        for j in 0..3 {
            let (a, b, c) = sort((numbers[i][j], numbers[i + 1][j], numbers[i + 2][j]));
            if a + b > c {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn main() {
    part_two();
}
