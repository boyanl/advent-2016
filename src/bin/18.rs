use std::io::stdin;

use bit_set::BitSet;

type Traps = (BitSet, usize);

fn read_input() -> Traps {
    let line = stdin().lines().last().unwrap().unwrap();
    let mut result = BitSet::new();

    for (i, c) in line.chars().enumerate() {
        if c == '^' {
            result.insert(i + 1);
        }
    }

    (result, line.len())
}

fn is_trap_on_next(idx: usize, traps: &Traps) -> bool {
    let (left, center, right) = (idx - 1, idx, idx + 1);
    let traps = &traps.0;

    return (traps.contains(left) && traps.contains(center) && !traps.contains(right))
        || (traps.contains(center) && traps.contains(right) && !traps.contains(left))
        || (traps.contains(left) && !traps.contains(center) && !traps.contains(right))
        || (traps.contains(right) && !traps.contains(center) && !traps.contains(left));
}

fn next_row(t: &Traps) -> Traps {
    let mut result = BitSet::new();
    let len = t.1;

    for idx in 1..=len {
        if is_trap_on_next(idx, t) {
            result.insert(idx);
        }
    }

    (result, len)
}

fn print_row(t: &Traps) {
    for i in 1..=t.1 {
        let c = if t.0.contains(i) { '^' } else { '.' };
        print!("{c}");
    }
    println!()
}

fn total_safe_tiles(start: Traps, rows: usize) -> usize {
    let mut curr = start;
    let mut total_traps = 0;

    for _ in 0..rows {
        total_traps += curr.1 - curr.0.len();
        curr = next_row(&curr);
    }

    total_traps
}

fn part_one() {
    let start = read_input();
    let result = total_safe_tiles(start, 40);

    println!("{result}");
}

fn part_two() {
    let start = read_input();
    let result = total_safe_tiles(start, 400_000);

    println!("{result}");
}

fn main() {
    part_two();
}
