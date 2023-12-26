use std::io::stdin;

use itertools::Itertools;

fn is_abba(s: &str) -> bool {
    let bytes = s.as_bytes();
    return s.len() == 4
        && s.chars().all(|c| c.is_ascii_lowercase())
        && bytes[0] == bytes[3]
        && bytes[1] == bytes[2]
        && bytes[0] != bytes[1];
}

fn supports_tls(s: &String) -> bool {
    let mut inside = false;
    let mut have_outside = false;
    let mut have_inside = false;
    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            inside = true;
        } else if c == ']' {
            inside = false;
        }

        if i >= 3 && is_abba(&s[i - 3..=i]) {
            let flag = if inside {
                &mut have_inside
            } else {
                &mut have_outside
            };
            *flag = true;
        }
    }

    have_outside && !have_inside
}

fn part_one() {
    let lines = stdin().lines().map(|l| l.unwrap()).collect_vec();
    let result = lines.iter().filter(|s| supports_tls(s)).count();

    println!("{result}");
}

fn is_aba(s: &str) -> bool {
    let bytes = s.as_bytes();
    return s.len() == 3
        && s.chars().all(|c| c.is_ascii_lowercase())
        && bytes[0] == bytes[2]
        && bytes[0] != bytes[1];
}

fn are_matching(aba: &str, bab: &str) -> bool {
    let (bytes1, bytes2) = (aba.as_bytes(), bab.as_bytes());

    return bytes1[2] == bytes2[1] && bytes1[1] == bytes2[0];
}

fn supports_ssl(s: &String) -> bool {
    let mut inside = false;
    let mut abas = Vec::new();
    let mut babs = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            inside = true;
        } else if c == ']' {
            inside = false;
        }

        if i >= 2 {
            let substr = &s[i - 2..=i];
            if is_aba(substr) {
                let to_insert = if inside { &mut babs } else { &mut abas };
                to_insert.push(substr.to_string());
            }
        }
    }

    for aba in &abas {
        if babs
            .iter()
            .any(|bab| are_matching(aba.as_str(), bab.as_str()))
        {
            return true;
        }
    }

    false
}

fn part_two() {
    let lines = stdin().lines().map(|l| l.unwrap()).collect_vec();
    let result = lines.iter().filter(|s| supports_ssl(s)).count();

    println!("{result}");
}

fn main() {
    part_two();
}
