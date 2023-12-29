use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Binary,
    io::stdin,
};

use itertools::Itertools;
use regex::*;
use sscanf::sscanf;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Item {
    Microchip(i32),
    Generator(i32),
}

// Items residing on a floor, represented as a bitmask
// the lowest 8 bits are for microchips, the highest 8 for generators
type FloorItems = u16;

fn add_item(f: FloorItems, item: Item) -> FloorItems {
    let mask = match item {
        Item::Microchip(i) => 1 << i,
        Item::Generator(i) => 1 << (8 + i),
    };
    f | mask
}

fn contains_item(f: FloorItems, item: Item) -> bool {
    add_item(f, item) == f
}

fn add_items(f: FloorItems, items: &[Item]) -> FloorItems {
    let mut res = f;
    for item in items {
        res = add_item(res, *item);
    }
    res
}

fn remove_item(f: FloorItems, item: Item) -> FloorItems {
    let mask = match item {
        Item::Microchip(i) => 1 << i,
        Item::Generator(i) => 1 << (8 + i),
    };
    f & !mask
}

fn remove_items(f: FloorItems, items: &[Item]) -> FloorItems {
    let mut res = f;
    for item in items {
        res = remove_item(res, *item);
    }
    res
}

fn items_count(f: FloorItems) -> u32 {
    f.count_ones()
}

fn items(f: FloorItems) -> Vec<Item> {
    let mut result = Vec::new();
    for i in 0..8 {
        if f & (1 << i) != 0 {
            result.push(Item::Microchip(i));
        }

        if f & (1 << (8 + i)) != 0 {
            result.push(Item::Generator(i));
        }
    }

    result
}

fn from_items(items: &Vec<Item>) -> FloorItems {
    let mut res = 0;
    for item in items {
        res = add_item(res, *item);
    }

    res
}

type Floors = u64;

fn set_floor(floors: Floors, idx: usize, floor: FloorItems) -> Floors {
    let clear_floor_mask = !(65535u64 << (16 * idx));
    let mask = (floor as u64) << (16 * idx);
    (floors & clear_floor_mask) | mask
}

fn get_floor(floors: Floors, idx: usize) -> FloorItems {
    let mask = 65535u64 << (16 * idx);
    let res: u16 = u16::try_from((floors & mask) >> (16 * idx)).unwrap();
    res
}

fn total_items_count(floors: Floors) -> u32 {
    floors.count_ones()
}

fn print_state(f: Floors, name_reverse_mapping: &HashMap<i32, String>) {
    for floor in (0..4).rev() {
        let floor_items = get_floor(f, floor as usize);
        let items_str = items(floor_items)
            .iter()
            .map(|item| match item {
                Item::Generator(i) => format!(
                    "{} generator",
                    name_reverse_mapping.get(i).unwrap_or(&i.to_string())
                ),
                Item::Microchip(i) => format!(
                    "{}-compatible microchip",
                    name_reverse_mapping.get(i).unwrap_or(&i.to_string())
                ),
            })
            .join(", ");
        println!("{floor}: {}", items_str);
    }
}

fn get_or_add_mapping(name_mapping: &mut HashMap<String, i32>, name: &str) -> i32 {
    if name_mapping.contains_key(name) {
        return name_mapping[name];
    }

    let next_val = name_mapping.iter().map(|(_, v)| *v).max().unwrap_or(0) + 1;
    name_mapping.insert(name.to_string(), next_val);
    next_val
}

fn parse_items(items_str: &str, name_mapping: &mut HashMap<String, i32>) -> Vec<Item> {
    if items_str == "nothing relevant" {
        return Vec::new();
    }

    let mut result = Vec::new();
    for part in Regex::new(r" and |, and |, |\.").unwrap().split(items_str) {
        if part.is_empty() {
            continue;
        }
        if let Ok(what) = sscanf!(part, "a {String} generator") {
            let idx = get_or_add_mapping(name_mapping, what.as_str());
            result.push(Item::Generator(idx));
        } else if let Ok(what) = sscanf!(part, "a {String}-compatible microchip") {
            let idx = get_or_add_mapping(name_mapping, what.as_str());
            result.push(Item::Microchip(idx));
        } else if part == "nothing relevant" {
            continue;
        } else {
            println!("Can't parse \"{part}\"");
        }
    }

    result
}

fn read_input() -> (Floors, HashMap<String, i32>) {
    let mut result: Floors = 0;

    let mut name_mapping = HashMap::new();

    for line in stdin().lines().map(|l| l.unwrap()) {
        if let Ok((floor_str, items_str)) = sscanf!(&line, "The {String} floor contains {String}") {
            let floor_idx = match floor_str.as_str() {
                "first" => 0,
                "second" => 1,
                "third" => 2,
                "fourth" => 3,
                _ => todo!(),
            };

            let items = parse_items(items_str.as_str(), &mut name_mapping);
            result = set_floor(result, floor_idx as usize, from_items(&items));
        }
    }

    (result, name_mapping)
}

fn ok(floor: FloorItems) -> bool {
    let mut unmatched_chips = [-1; 16];
    let mut have_generator = false;

    for item in items(floor) {
        match item {
            Item::Microchip(i) => {
                if unmatched_chips[i as usize] == -1 {
                    unmatched_chips[i as usize] = 1;
                }
            }
            Item::Generator(i) => {
                have_generator = true;
                unmatched_chips[i as usize] = 0;
            }
        }
    }

    let any_unmatched = unmatched_chips.iter().any(|x| *x == 1);

    return !(any_unmatched && have_generator);
}

fn next_states(floor: i32, floors: Floors) -> Vec<(i32, Floors)> {
    let mut result = Vec::new();

    let floor_items = get_floor(floors, floor as usize);
    let items = items(floor_items);
    if items.len() >= 1 {
        // Take 1 item
        for &item in items.iter() {
            let remaining = remove_item(floor_items, item);

            if ok(remaining) {
                if floor < 3 {
                    let mut above = get_floor(floors, (floor + 1) as usize);
                    above = add_item(above, item);

                    if ok(above) {
                        let mut new_floors = floors;
                        new_floors = set_floor(new_floors, floor as usize, remaining);
                        new_floors = set_floor(new_floors, (floor + 1) as usize, above);
                        result.push((floor + 1, new_floors));
                    }
                }
                if floor > 0 {
                    let mut below = get_floor(floors, (floor - 1) as usize);
                    below = add_item(below, item);

                    if ok(below) {
                        let mut new_floors = floors;
                        new_floors = set_floor(new_floors, floor as usize, remaining);
                        new_floors = set_floor(new_floors, (floor - 1) as usize, below);
                        result.push((floor - 1, new_floors));
                    }
                }
            }
        }
    }

    if items.len() >= 2 {
        for (i, &item1) in items.iter().enumerate() {
            for j in i + 1..items.len() {
                let item2 = items[j];
                let remaining = remove_item(remove_item(floor_items, item1), item2);

                if ok(remaining) {
                    if floor < 3 {
                        let mut above = get_floor(floors, (floor + 1) as usize);
                        above = add_item(add_item(above, item1), item2);

                        if ok(above) {
                            let mut new_floors = floors;
                            new_floors = set_floor(new_floors, floor as usize, remaining);
                            new_floors = set_floor(new_floors, (floor + 1) as usize, above);
                            result.push((floor + 1, new_floors));
                        }
                    }
                    if floor > 0 {
                        let mut below = get_floor(floors, (floor - 1) as usize);
                        below = add_item(add_item(below, item1), item2);

                        if ok(below) {
                            let mut new_floors = floors;
                            new_floors = set_floor(new_floors, floor as usize, remaining);
                            new_floors = set_floor(new_floors, (floor - 1) as usize, below);
                            result.push((floor - 1, new_floors));
                        }
                    }
                }
            }
        }
    }

    result
}

fn pairs(floor: FloorItems) -> Vec<(Item, Item)> {
    let mut result = Vec::new();
    for item in items(floor) {
        match item {
            Item::Generator(i) => {
                if contains_item(floor, Item::Microchip(i)) {
                    result.push((item, Item::Microchip(i)));
                }
            }
            Item::Microchip(_) => {}
        }
    }
    result
}

const optimize_heuristic: bool = true;

fn steps_required(items_state: Floors, start_floor: i32) -> i32 {
    let mut q = BinaryHeap::new();
    let start = (Reverse(0), (start_floor, items_state));
    q.push(start);

    let mut visited = HashSet::new();

    while !q.is_empty() {
        let (time_rev, (floor, mut floors_state)) = q.pop().unwrap();
        let mut time = time_rev.0;

        if visited.contains(&(floor, floors_state)) {
            continue;
        }
        visited.insert((floor, floors_state));

        if floor == 3
            && items_count(get_floor(floors_state, floor as usize))
                == total_items_count(floors_state)
        {
            return time;
        }

        if optimize_heuristic {
            for (target_floor, &cost) in [12, 8].iter().enumerate() {
                if floor == target_floor as i32 {
                    let mut floor_items = get_floor(floors_state, target_floor);
                    let pairs = pairs(floor_items);

                    if pairs.len() > 1 {
                        for &(item1, item2) in &pairs[1..] {
                            floor_items = remove_items(floor_items, &[item1, item2]);
                        }

                        floors_state = set_floor(floors_state, target_floor, floor_items);
                        time += (pairs.len() as i32 - 1) * cost;
                    }
                }
            }
        }
        for next in next_states(floor, floors_state) {
            q.push((Reverse(time + 1), next));
        }
    }

    -1
}

fn part_one() {
    let (start_state, _) = read_input();

    let result = steps_required(start_state, 0);
    println!("{result}");
}

fn part_two() {
    let (mut start_state, mapping) = read_input();
    let m = *mapping.iter().map(|(_, v)| v).max().unwrap();

    // add "elerium" and "dilithium" generator + chip by hand
    let mut new_floor_0 = get_floor(start_state, 0);
    new_floor_0 = add_items(
        new_floor_0,
        &[
            Item::Generator(m + 1),
            Item::Microchip(m + 1),
            Item::Generator(m + 2),
            Item::Microchip(m + 2),
        ],
    );
    start_state = set_floor(start_state, 0, new_floor_0);

    let result = steps_required(start_state, 0);
    println!("{result}");
}

fn test_stuff() {
    for floor in 0..3 {
        let mut items = 0;
        for pairs_cnt in 1..=7 {
            items = add_items(
                items,
                &[Item::Microchip(pairs_cnt), Item::Generator(pairs_cnt)],
            );
            println!(
                "{pairs_cnt} pairs, start floor = {floor}, result = {}",
                steps_required(set_floor(0, floor as usize, items), floor as i32)
            );
        }
    }
}

fn main() {
    part_two();
}
