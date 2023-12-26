use std::io::stdin;

use sscanf::sscanf;

fn decompressed_once(s: &str) -> String {
    let mut remaining = s;
    let mut result = String::new();

    while remaining.len() > 0 {
        let c = remaining.chars().nth(0).unwrap();
        if c == '(' {
            if let Some(end) = remaining.find(")") {
                let block = &remaining[..=end];
                if let Ok((amount, times)) = sscanf!(block, "({usize}x{usize})") {
                    // println!(
                    //     "End = {end}, len = {}, remaining = {remaining}",
                    //     remaining.len()
                    // );
                    let to_repeat = &remaining[end + 1..end + amount + 1];

                    for _ in 0..times {
                        result.push_str(to_repeat);
                    }

                    remaining = &remaining[end + amount + 1..];
                }
            }
        } else {
            result.push(c);
            remaining = &remaining[1..];
        }
    }

    result
}

fn part_one() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let result = decompressed_once(line.as_str());
        println!("{}", result.len());
    }
}

fn decompressed_fully_len(s: &str) -> usize {
    if !s.contains("(") {
        return s.len();
    }

    let start = s.find("(").unwrap();
    let end = s.find(")").unwrap();
    let (amount, times) = sscanf!(&s[start..=end], "({usize}x{usize})").unwrap();
    let repeated_block = &s[end + 1..end + amount + 1];
    let rest = &s[end + amount + 1..];

    start + times * decompressed_fully_len(repeated_block) + decompressed_fully_len(rest)
}

fn part_two() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let result = decompressed_fully_len(line.as_str());
        println!("{result}");
    }
}

fn main() {
    part_two();
}
