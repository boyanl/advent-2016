mod util;

use std::collections::{HashSet, VecDeque};

use util::vec2::*;

#[derive(Clone, Copy, Debug)]
struct Maze {
    fav_number: i32,
}

impl Maze {
    fn is_wall(self: &Self, pos: Point2) -> bool {
        let (x, y) = (pos.x, pos.y);
        let mut s = x * x + 3 * x + 2 * x * y + y + y * y;
        s += self.fav_number;

        s.count_ones() % 2 == 1
    }
}

fn neighbours(pos: Point2, maze: Maze) -> Vec<Point2> {
    let mut result = Vec::new();
    for dir in [UP, LEFT, RIGHT, DOWN] {
        let next = pos + dir;
        if next.x >= 0 && next.y >= 0 && !maze.is_wall(next) {
            result.push(next);
        }
    }

    result
}

fn shortest_path_len(maze: Maze, start: Point2, dest: Point2) -> Option<u32> {
    let mut q = VecDeque::new();
    q.push_back((0, start));

    let mut visited = HashSet::new();
    visited.insert(start);

    while !q.is_empty() {
        let (dist, pos) = q.pop_front().unwrap();
        if pos == dest {
            return Some(dist);
        }

        for next in neighbours(pos, maze) {
            if !visited.contains(&next) {
                q.push_back((dist + 1, next));
                visited.insert(next);
            }
        }
    }

    None
}

fn accessible_locations(maze: Maze, start: Point2, max_dist: u32) -> HashSet<Point2> {
    let mut q = VecDeque::new();
    q.push_back((0, start));

    let mut visited = HashSet::new();
    visited.insert(start);

    while !q.is_empty() {
        let (dist, pos) = q.pop_front().unwrap();

        for next in neighbours(pos, maze) {
            if !visited.contains(&next) && dist + 1 <= max_dist {
                q.push_back((dist + 1, next));
                visited.insert(next);
            }
        }
    }

    visited
}

fn part_one() {
    let number = 1362;
    let maze = Maze { fav_number: number };
    let start = Point2 { x: 1, y: 1 };
    let dest = Point2 { x: 31, y: 39 };
    let result = shortest_path_len(maze, start, dest);

    if let Some(d) = result {
        println!("{d}");
    } else {
        println!("No path");
    }
}

fn part_two() {
    let number = 1362;
    let maze = Maze { fav_number: number };
    let start = Point2 { x: 1, y: 1 };
    let result = accessible_locations(maze, start, 50).len();
    println!("{result}");
}

fn main() {
    part_two();
}
