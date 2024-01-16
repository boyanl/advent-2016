use std::{collections::VecDeque, result};

fn simulate_1(n: usize) -> usize {
    let mut cnts = vec![0; n];
    for i in 0..n {
        cnts[i] = i + 1;
    }

    while cnts.len() > 1 {
        let mut to_remove = vec![0; cnts.len()];
        for i in (0..cnts.len()).step_by(2) {
            to_remove[(i + 1) % cnts.len()] = 1;
        }

        let mut next_cnts = Vec::new();
        for i in 0..cnts.len() {
            if to_remove[i] == 0 {
                next_cnts.push(cnts[i]);
            }
        }
        cnts = next_cnts;
    }

    cnts[0]
}

fn part_one() {
    let input = 3014387;
    let result = simulate_1(input);
    println!("{result}");
}

fn simulate_2(n: usize) -> usize {
    let mut indices = VecDeque::new();
    for i in 0..n {
        indices.push_back(i + 1);
    }

    while indices.len() > 1 {
        let to_remove = indices.len() / 2;
        indices.remove(to_remove);
        indices.rotate_left(1);
    }

    indices[0]
}

fn calculate_2(n: usize) -> usize {
    let mut result = vec![0; n + 1];
    result[2] = 1;

    let mut to = 3;
    let mut cnt = 1;

    'out: while to <= n {
        for _ in 0..cnt {
            result[to] = result[to - 1] + 2;
            to += 1;

            if to > n {
                break 'out;
            }
        }

        cnt = result[to - 1];
        for i in 1..=cnt {
            result[to] = i;
            to += 1;

            if to > n {
                break 'out;
            }
        }
    }

    result[n]
}

fn part_two() {
    let input = 3014387;
    let result = calculate_2(input);

    println!("{result}");
}

fn main() {
    part_two();
}
