use std::{cmp::Reverse, collections::HashMap, io::stdin};

use itertools::Itertools;

fn check_room(name_parts: &Vec<String>, checksum: &str) -> bool {
    let mut freqs = HashMap::new();
    for part in name_parts {
        for c in part.chars() {
            *freqs.entry(c).or_insert(0) += 1;
        }
    }
    let mut pairs = freqs.iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(c, freq)| (Reverse(*freq), *c));

    for (i, c) in checksum.chars().enumerate() {
        if *pairs[i].0 != c {
            return false;
        }
    }

    true
}

fn shift(letter: char, amount: i32) -> char {
    let n = ('z' as i32) - ('a' as i32) + 1;
    let a = 'a' as i32;
    return char::from_u32(((((letter as i32) - a + amount) % n) + a) as u32).unwrap();
}

fn decrypt_name(name_parts: &Vec<String>, sector_id: i32) -> String {
    let mut result = String::new();

    for part in name_parts {
        for c in part.chars() {
            result.push(shift(c, sector_id));
        }
        result.push(' ');
    }

    result
}

struct Room {
    name_parts: Vec<String>,
    sector_id: i32,
    checksum: String,
}

fn read_rooms() -> Vec<Room> {
    let mut result = Vec::new();

    for line in stdin().lines().map(|l| l.unwrap()) {
        let parts = line.split("-").collect::<Vec<_>>();
        let (number, checksum) = parts
            .last()
            .unwrap()
            .split("[")
            .collect_tuple::<(&str, &str)>()
            .map(|(p1, p2)| (p1.parse::<i32>().unwrap(), &p2[0..p2.len() - 1]))
            .unwrap();
        let name_parts = parts[0..parts.len() - 1]
            .iter()
            .map(|x| x.to_string())
            .collect_vec();

        result.push(Room {
            name_parts: name_parts,
            sector_id: number,
            checksum: checksum.to_string(),
        });
    }

    result
}

fn part_one() {
    let rooms = read_rooms();

    let result = rooms
        .iter()
        .filter(|room| check_room(&room.name_parts, &room.checksum))
        .map(|room| room.sector_id)
        .sum::<i32>();
    println!("{result}");
}

fn part_two() {
    let rooms = read_rooms();

    let room = rooms
        .iter()
        .filter(|room| {
            let decryped = decrypt_name(&room.name_parts, room.sector_id).to_ascii_lowercase();
            return decryped.contains("north") || decryped.contains("pole");
        })
        .next()
        .unwrap();

    println!("{}", room.sector_id);
}

fn main() {
    part_one();
}
