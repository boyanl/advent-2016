use std::{cmp, io::stdin};

use itertools::Itertools;
use sscanf::sscanf;

fn ensure_size(v: &mut Vec<Vec<usize>>, sz: usize) -> &mut Vec<Vec<usize>> {
    if v.len() <= sz {
        v.resize(sz + 1, Vec::new());
    }

    v
}

fn main() {
    let magic = (17, 61);

    let mut bot_chips = Vec::new();
    let mut outputs = Vec::new();

    let instructions = stdin().lines().map(|l| l.unwrap()).collect_vec();

    for line in &instructions {
        if let Ok((value, bot)) = sscanf!(&line, "value {usize} goes to bot {usize}") {
            ensure_size(&mut bot_chips, bot)[bot].push(value);
        }
    }

    loop {
        for line in &instructions {
            if let Ok((bot, low_dest, high_dest)) = sscanf!(
                &line,
                "bot {usize} gives low to {String} and high to {String}"
            ) {
                if ensure_size(&mut bot_chips, bot)[bot].len() < 2 {
                    continue;
                }

                let (chip1, chip2) = (bot_chips[bot][0], bot_chips[bot][1]);
                let (low, high) = (cmp::min(chip1, chip2), cmp::max(chip1, chip2));

                if (low, high) == magic {
                    println!("Bot {bot} compared values {:?}", magic);
                }

                let (low_dest_type, low_dest_num) = low_dest
                    .split_ascii_whitespace()
                    .collect_tuple()
                    .map(|(t, n)| (t, n.parse::<usize>().unwrap()))
                    .unwrap();

                match low_dest_type {
                    "bot" => ensure_size(&mut bot_chips, low_dest_num)[low_dest_num].push(low),
                    "output" => ensure_size(&mut outputs, low_dest_num)[low_dest_num].push(low),
                    _ => todo!(),
                }

                let (high_dest_type, high_dest_num) = high_dest
                    .split_ascii_whitespace()
                    .collect_tuple()
                    .map(|(t, n)| (t, n.parse::<usize>().unwrap()))
                    .unwrap();

                match high_dest_type {
                    "bot" => ensure_size(&mut bot_chips, high_dest_num)[high_dest_num].push(high),
                    "output" => ensure_size(&mut outputs, high_dest_num)[high_dest_num].push(high),
                    _ => todo!(),
                }

                bot_chips[bot].clear();
            }
        }
        if bot_chips.iter().all(|chips| chips.len() < 2) {
            break;
        }
    }
    println!(
        "Output values [0, 1, 2] multiplied = {}",
        outputs[0][0] * outputs[1][0] * outputs[2][0]
    );
}
