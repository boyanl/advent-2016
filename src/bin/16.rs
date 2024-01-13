use bit_set::BitSet;
use itertools::Itertools;

fn to_bytes(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect_vec()
}

fn invert(set: &mut Vec<u8>) {
    for v in set {
        *v = 1 - *v;
    }
}

fn iter(set: &Vec<u8>) -> Vec<u8> {
    let mut res = set.clone();
    res.push(0);
    let mut second_half = set.clone();
    second_half.reverse();
    invert(&mut second_half);
    res.append(&mut second_half);

    res
}

fn checksum_iter(s: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let mut res = Vec::new();
    while i < s.len() {
        let (c1, c2) = (s[i], s[i + 1]);
        res.push(if c1 == c2 { 1 } else { 0 });
        i += 2;
    }

    res
}

fn checksum(set: &[u8]) -> Vec<u8> {
    let mut current = checksum_iter(set);

    while current.len() % 2 == 0 {
        current = checksum_iter(&current);
    }

    current
}

fn find_dragon_checksum(start: &str, desired_len: usize) -> String {
    let start_set = to_bytes(start);
    let mut current = start_set;

    while current.len() < desired_len {
        current = iter(&mut current);
    }

    let data = &current[..desired_len];

    let checksum = checksum(&data);
    checksum.iter().map(|x| x.to_string()).join("")
}

fn part_one() {
    let checksum = find_dragon_checksum("11101000110010100", 272);
    println!("{checksum}");
}

fn part_two() {
    let checksum = find_dragon_checksum("11101000110010100", 35651584);
    println!("{checksum}");
}

fn main() {
    part_one();
}
