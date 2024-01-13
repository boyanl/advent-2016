use std::cmp::{max, min};
use std::io::stdin;

use itertools::Itertools;
use sscanf::sscanf;

#[derive(Clone, Copy, Debug)]
struct Disk {
    pos: i32,
    n: i32,
}

// Returns (gcd, a1, a2); a1, a2 are such that a1*x + a2*y = gcd
fn extended_euclid(x: i64, y: i64) -> (i64, i64, i64) {
    let (mut s_prev, mut t_prev) = (1, 0);
    let (mut s, mut t) = (0, 1);

    let mut a = max(x, y);
    let mut b = min(x, y);
    let swap = a == y;

    while b > 0 {
        let q = a / b;
        (a, b) = (b, a % b);

        let s_next = s_prev - q * s;
        let t_next = t_prev - q * t;

        (s, s_prev) = (s_next, s);
        (t, t_prev) = (t_next, t);
    }

    if swap {
        return (a, t_prev, s_prev);
    }

    (a, s_prev, t_prev)
}

fn solve_crt_2x2(a1: i64, n1: i64, a2: i64, n2: i64) -> i64 {
    let (g, m1, m2) = extended_euclid(n1, n2);
    assert!(g == 1);

    (a1 * m2 * n2 + a2 * m1 * n1).rem_euclid(n1 * n2)
}

fn solve_crt(mods: &Vec<(i64, i64)>) -> i64 {
    assert!(mods.len() >= 2);
    let mut solution = solve_crt_2x2(mods[0].0, mods[0].1, mods[1].0, mods[1].1);
    let mut total_mod = mods[0].1 * mods[1].1;

    for &(r, m) in &mods[2..] {
        solution = solve_crt_2x2(solution, total_mod, r, m);
        total_mod *= m;
    }

    solution
}

fn read_input() -> Vec<Disk> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if let Ok((_, n, pos)) = sscanf!(
            &line,
            "Disc #{i32} has {i32} positions; at time=0, it is at position {i32}."
        ) {
            result.push(Disk { n: n, pos: pos });
        }
    }

    result
}

fn test_extended_gcd(n1: i64, n2: i64) {
    let (gcd, a1, a2) = extended_euclid(n1, n2);
    println!(
        "GCD = {gcd}, a1 = {a1}, a2 = {a2}, a1*n1 + a2*n2 = {}",
        a1 * n1 + a2 * n2
    );
}

fn part_one() {
    let disks = read_input();
    let coeffs = disks
        .iter()
        .enumerate()
        .map(|(idx, &disk)| {
            (
                (-disk.pos - (idx as i32) - 1).rem_euclid(disk.n) as i64,
                disk.n as i64,
            )
        })
        .collect_vec();

    let result = solve_crt(&coeffs);

    println!("{result}");
}

fn part_two() {
    let mut disks = read_input();
    disks.push(Disk { pos: 0, n: 11 });

    let coeffs = disks
        .iter()
        .enumerate()
        .map(|(idx, &disk)| {
            (
                (-disk.pos - (idx as i32) - 1).rem_euclid(disk.n) as i64,
                disk.n as i64,
            )
        })
        .collect_vec();

    let result = solve_crt(&coeffs);

    println!("{result}");
}

fn main() {
    part_two();
}
