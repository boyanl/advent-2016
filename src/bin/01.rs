mod util;
use std::{
    collections::HashSet,
    io::{stdin, Cursor},
};

use util::vec2::*;

fn final_location(instructions: &str) -> Point2 {
    let mut current = (origin(), UP);

    for instruction in instructions.split(", ") {
        let dir = &instruction[0..1];
        let amount = instruction[1..].parse::<i32>().unwrap();

        let new_dir = match dir {
            "L" => rotate_left(current.1),
            "R" => rotate_right(current.1),
            _ => todo!(),
        };

        current.0 += new_dir * amount;
        current.1 = new_dir;
    }

    current.0
}

fn distance(p: Point2) -> i32 {
    return p.x.abs() + p.y.abs();
}

fn part_one() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        println!("{}", distance(final_location(line.as_str())));
    }
}

fn part_two() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut current = (origin(), UP);
        let mut visited = HashSet::new();
        visited.insert(current.0);

        for instruction in line.split(", ") {
            let dir = &instruction[0..1];
            let amount = instruction[1..].parse::<i32>().unwrap();

            let new_dir = match dir {
                "L" => rotate_left(current.1),
                "R" => rotate_right(current.1),
                _ => todo!(),
            };

            for i in 1..=amount {
                let new_pt = current.0 + new_dir * i;
                if visited.contains(&new_pt) {
                    println!("{}", distance(new_pt));
                    return;
                }
                visited.insert(new_pt);
            }

            current.0 += new_dir * amount;
            current.1 = new_dir;
        }
    }
}

fn main() {
    part_two();
}
