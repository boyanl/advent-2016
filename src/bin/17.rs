use std::{
    collections::{HashSet, VecDeque},
    fmt::format,
};

use md5::Digest;

mod util;

use util::vec2::{Direction, Point2, DOWN, LEFT, RIGHT, UP};

fn is_open(c: char) -> bool {
    "bcdef".contains(c)
}

fn possible_directions(digest: Digest) -> Vec<Direction> {
    let s = &format!("{:x}", digest)[..4];
    let mut result = Vec::new();
    for (i, &dir) in [UP, DOWN, LEFT, RIGHT].iter().enumerate() {
        if is_open(s.chars().nth(i).unwrap()) {
            result.push(dir);
        }
    }

    result
}

fn direction_str(d: Direction) -> &'static str {
    match d {
        UP => "U",
        DOWN => "D",
        LEFT => "L",
        RIGHT => "R",
        _ => todo!(),
    }
}

fn shortest_path_to_exit(maze_passcode: &str) -> String {
    let mut q = VecDeque::new();
    let start = Point2 { x: 0, y: 0 };
    q.push_back((start, "".to_string()));

    while !q.is_empty() {
        let (pos, directions) = q.pop_front().unwrap();
        if pos.x == 3 && pos.y == 3 {
            return directions;
        }
        let digest = md5::compute(maze_passcode.to_string() + directions.as_str());

        for dir in possible_directions(digest) {
            let new_pos = pos + dir;
            if new_pos.x >= 0 && new_pos.y >= 0 && new_pos.x <= 3 && new_pos.y <= 3 {
                q.push_back((new_pos, directions.to_string() + direction_str(dir)));
            }
        }
    }

    "No path something is wrong".to_string()
}

fn part_one() {
    let passcode = "veumntbg";
    let path = shortest_path_to_exit(passcode);

    println!("{path}");
}

fn longest_path_to_exit(maze_passcode: &str) -> String {
    let mut q = VecDeque::new();
    let start = Point2 { x: 0, y: 0 };
    q.push_back((start, "".to_string()));

    let mut result = "".to_owned();
    while !q.is_empty() {
        let (pos, directions) = q.pop_front().unwrap();
        if pos.x == 3 && pos.y == 3 {
            if directions.len() > result.len() {
                result = directions;
            }
            continue;
        }
        let digest = md5::compute(maze_passcode.to_string() + directions.as_str());

        for dir in possible_directions(digest) {
            let new_pos = pos + dir;
            if new_pos.x >= 0 && new_pos.y >= 0 && new_pos.x <= 3 && new_pos.y <= 3 {
                q.push_back((new_pos, directions.to_string() + direction_str(dir)));
            }
        }
    }

    result
}

fn part_two() {
    let passcode = "veumntbg";
    let path = longest_path_to_exit(passcode);

    println!("{}", path.len());
}

fn main() {
    part_two();
}
