use std::io::{stdin, Read};

fn find_password(door_id: &str) -> String {
    let mut password = String::new();
    let start = &[0u8, 0u8];

    for i in 0.. {
        let to_hash = door_id.to_owned() + i.to_string().as_str();
        let hash = md5::compute(to_hash.as_bytes());

        let byte_3 = hash.bytes().nth(2).unwrap().unwrap();
        if hash.starts_with(start) && (byte_3 & 0xf0 == 0) {
            let c = char::from_digit((byte_3 % 16) as u32, 16).unwrap();
            password.push(c);

            if password.len() == 8 {
                break;
            }
        }
    }

    password
}

fn part_one() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let door_id = line;
        let password = find_password(door_id.as_str());

        println!("{password}");
    }
}

fn find_password_2(door_id: &str) -> String {
    let mut password: [u8; 8] = [0; 8];
    let mut remaining = 8;
    let start = &[0u8, 0u8];

    for i in 0.. {
        let to_hash = door_id.to_owned() + i.to_string().as_str();
        let hash = md5::compute(to_hash.as_bytes());

        let bytes = hash.0;
        if hash.starts_with(start) && (bytes[2] & 0xf0 == 0) {
            let c1 = char::from_digit((bytes[2] % 16) as u32, 16).unwrap();
            let c2 = char::from_digit(((bytes[3] / 16) % 16) as u32, 16).unwrap();

            if let Some(idx) = c1.to_digit(8) {
                if password[idx as usize] == 0 {
                    password[idx as usize] = c2 as u8;
                    remaining -= 1;

                    if remaining == 0 {
                        break;
                    }
                }
            }
        }
    }

    String::from_utf8_lossy(&password).to_string()
}

fn part_two() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let door_id = line;
        let password = find_password_2(door_id.as_str());

        println!("{password}");
    }
}

fn main() {
    part_two();
}
