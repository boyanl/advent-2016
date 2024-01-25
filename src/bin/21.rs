use std::{collections::VecDeque, io::stdin, mem::swap};

use itertools::Itertools;
use sscanf::sscanf;
use Op::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    SwapPos(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateRightBasedOn(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn read_input() -> Vec<Op> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if let Ok((i, j)) = sscanf!(&line, "swap position {usize} with position {usize}") {
            result.push(SwapPos(i, j));
        } else if let Ok((i, j)) = sscanf!(&line, "reverse positions {usize} through {usize}") {
            result.push(Reverse(i, j));
        } else if let Ok(()) = sscanf!(&line, "rotate left 1 step") {
            result.push(RotateLeft(1));
        } else if let Ok(n) = sscanf!(&line, "rotate left {usize} steps") {
            result.push(RotateLeft(n));
        } else if let Ok(()) = sscanf!(&line, "rotate right 1 step") {
            result.push(RotateRight(1));
        } else if let Ok(n) = sscanf!(&line, "rotate right {usize} steps") {
            result.push(RotateRight(n));
        } else if let Ok((i, j)) = sscanf!(&line, "move position {usize} to position {usize}") {
            result.push(Move(i, j));
        } else if let Ok(c) = sscanf!(&line, "rotate based on position of letter {char}") {
            result.push(RotateRightBasedOn(c));
        } else if let Ok((c1, c2)) = sscanf!(&line, "swap letter {char} with letter {char}") {
            result.push(SwapLetters(c1, c2));
        } else {
            println!("Unrecognized instruction: {line}");
        }
    }

    result
}

fn scramble(s: &str, operations: &Vec<Op>) -> String {
    let mut v = s.chars().collect::<VecDeque<_>>();
    for &op in operations {
        match op {
            SwapPos(i, j) => v.swap(i, j),
            SwapLetters(c1, c2) => {
                let i = v.iter().position(|c| *c == c1).unwrap();
                let j = v.iter().position(|c| *c == c2).unwrap();
                v.swap(i, j);
            }
            RotateLeft(n) => v.rotate_left(n),
            RotateRight(n) => v.rotate_right(n),
            RotateRightBasedOn(c) => {
                let pos = v.iter().position(|x| *x == c).unwrap();
                let amount = 1 + pos + if pos >= 4 { 1 } else { 0 };
                v.rotate_right(amount % v.len());
            }
            Reverse(i, j) => {
                for di in 0..=(j - i) / 2 {
                    v.swap(i + di, j - di);
                }
            }
            Move(i, j) => {
                let el = v[i];
                v.remove(i);
                v.insert(j, el);
            }
        }
    }
    v.iter().collect::<String>()
}

// much brute-force
fn unscramble(s: &str, operations: &Vec<Op>) -> String {
    let perms = s.chars().permutations(s.len());
    for perm in perms {
        let str = perm.iter().collect::<String>();
        if scramble(&str, operations) == s {
            return str;
        }
    }
    return "bad".to_string();
}

fn part_one() {
    let input = "abcdefgh";
    let operations = read_input();
    let result = scramble(input, &operations);

    println!("{result}");
}

fn part_two() {
    let input = "fbgdceah";
    let operations = read_input();
    let result = unscramble(input, &operations);
    println!("{result}");
}

fn main() {
    part_two();
}
