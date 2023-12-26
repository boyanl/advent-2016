use std::{collections::HashMap, io::stdin};

use itertools::Itertools;

fn part_one() {
    let messages = stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect_vec();

    let m = messages[0].len();

    let mut result = String::new();
    for i in 0..m {
        let mut freqs = HashMap::new();
        for msg in &messages {
            *freqs.entry(msg[i]).or_insert(0) += 1;
        }

        let c = *freqs.iter().max_by_key(|(_, freq)| *freq).unwrap().0;
        result.push(c);
    }

    println!("{result}");
}

fn part_two() {
    let messages = stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect_vec();

    let m = messages[0].len();

    let mut result = String::new();
    for i in 0..m {
        let mut freqs = HashMap::new();
        for msg in &messages {
            *freqs.entry(msg[i]).or_insert(0) += 1;
        }

        let c = *freqs.iter().min_by_key(|(_, freq)| *freq).unwrap().0;
        result.push(c);
    }

    println!("{result}");
}

fn main() {
    part_two();
}
