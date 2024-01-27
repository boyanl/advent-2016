use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    io::stdin,
};

use itertools::Itertools;
use sscanf::sscanf;
use util::vec2::Point2;

mod util;

// derive PartialOrd + Ord  as well so we can stick it as part of a tuple in a priority queue ...
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    used: usize,
    available: usize,
}

type Grid = Vec<Vec<Node>>;

fn read_input() -> Grid {
    let mut result: Vec<Vec<Node>> = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if let Ok((x, y, _, total, _, used, _, available, _, percent)) = sscanf!(
            &line,
            "/dev/grid/node-x{usize}-y{usize}{str:/\\s+/}{usize}T{str:/\\s+/}{usize}T{str:/\\s+/}{usize}T{str:/\\s+/}{usize}%"
        ) {
            if result.len() <= y {
                result.resize(y + 1, vec![]);
            }
            if result[y].len() <= x {
                result[y].resize(x + 1, Node { used: 0, available: 0});
            }
            result[y][x] = Node { used: used, available: available };
        }
    }

    result
}

fn part_one() {
    let mut result = 0;
    let grid = read_input();
    let nodes = grid.iter().flat_map(|v| v).collect_vec();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            if nodes[j].available >= nodes[i].used {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn neighbours(coords: Point2, limits: Point2) -> Vec<Point2> {
    let mut result = Vec::new();
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_pos = coords + Point2 { x: dx, y: dy };
        if new_pos.x >= 0 && new_pos.x < limits.x && new_pos.y >= 0 && new_pos.y < limits.y {
            result.push(new_pos)
        }
    }

    result
}

type Pos = Point2;

// Technically the immovable walls are part of the state too
// but they don't ever change, and it's easier to not include them so we can clone/copy/move out this struct
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct SimplifiedState {
    target: Pos,
    empty: Pos,
    limits: Point2,
}

fn convert_state(grid: &Grid, target: Point2) -> (SimplifiedState, HashSet<Pos>) {
    let (n, m) = (grid.len(), grid[0].len());
    let total_cnt = n * m; // assume rectangular grid
    let avg_total = grid
        .iter()
        .flat_map(|v| v)
        .map(|n| (n.available + n.used) as i32)
        .sum::<i32>()
        / (total_cnt as i32);

    let mut empty = Vec::new();
    let mut immovable = HashSet::new();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j].used == 0 {
                empty.push(Point2 {
                    x: j as i32,
                    y: i as i32,
                });
            } else if grid[i][j].used > (avg_total as usize) {
                immovable.insert(Point2 {
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
    }

    assert!(empty.len() == 1);
    (
        SimplifiedState {
            target: target,
            empty: empty[0],
            limits: Point2 {
                x: m as i32,
                y: n as i32,
            },
        },
        immovable,
    )
}

fn manhattan_dist(p1: Point2, p2: Point2) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn move_empty_cost(
    from: Pos,
    to: Pos,
    target: Pos,
    immovable: &HashSet<Pos>,
    limits: Point2,
) -> Option<i32> {
    let mut q = VecDeque::new();
    q.push_back((from, 0));

    let mut visited = HashSet::new();
    visited.insert(from);

    while !q.is_empty() {
        let (pos, d) = q.pop_front().unwrap();
        if pos == to {
            return Some(d);
        }

        for next in neighbours(pos, limits) {
            if !visited.contains(&next) && !immovable.contains(&next) && next != target {
                q.push_back((next, d + 1));
                visited.insert(next);
            }
        }
    }

    None
}

fn next_states(s: &SimplifiedState, immovable: &HashSet<Pos>) -> Vec<(SimplifiedState, i32)> {
    let mut res = Vec::new();
    for next in neighbours(s.target, s.limits) {
        if !immovable.contains(&next) {
            // Move empty node to the node adjacent to target
            // then move empty to target, in order to advance target to the next position
            if let Some(cost) = move_empty_cost(s.empty, next, s.target, immovable, s.limits) {
                let next_state = SimplifiedState {
                    empty: s.target,
                    target: next,
                    limits: s.limits,
                };
                res.push((next_state, cost + 1));
            }
        }
    }

    res
}

fn shortest_path(start_state: &Grid, source: Point2, target: Point2) -> i32 {
    let (simplified, immovable) = convert_state(start_state, target);
    let mut q = BinaryHeap::new();

    q.push(Reverse((manhattan_dist(source, target), 0, simplified)));

    let mut visited = HashSet::new();

    while !q.is_empty() {
        let (_, d, state) = q.pop().unwrap().0;

        if state.target == source {
            return d;
        }
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        for (next, cost) in next_states(&state, &immovable) {
            q.push(Reverse((
                d + cost + manhattan_dist(next.target, source),
                d + cost,
                next,
            )));
        }
    }

    -1
}

fn part_two() {
    let grid = read_input();
    let source = Point2 { x: 0, y: 0 };
    let target = Point2 {
        x: (grid[0].len() - 1) as i32,
        y: 0,
    };
    let result = shortest_path(&grid, source, target);

    println!("{result}");
}

fn main() {
    part_two();
}
