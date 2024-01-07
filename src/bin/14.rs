fn consecutive(data: &str, amount: usize) -> Option<char> {
    for i in 0..data.len() - amount + 1 {
        let c = data.chars().nth(i).unwrap();
        let mut ok = true;
        for j in i + 1..i + amount {
            if data.chars().nth(j).unwrap() != c {
                ok = false;
                break;
            }
        }

        if ok {
            return Some(c);
        }
    }
    None
}

fn nth_key<F: Fn(i32) -> String>(n: usize, iter_fn: F) -> (i32, String) {
    let mut potential_keys = Vec::new();
    let mut keys = Vec::new();
    for i in 0.. {
        let key = iter_fn(i);

        if let Some(c) = consecutive(key.as_str(), 3) {
            potential_keys.push((i, c, key.clone()));
        }

        if potential_keys.len() >= 1 && i - potential_keys[0].0 > 1000 {
            potential_keys.remove(0);
        }

        if let Some(c) = consecutive(key.as_str(), 5) {
            let mut to_remove = Vec::new();
            for (idx, (i1, rep, k)) in potential_keys.iter().enumerate() {
                if i == *i1 {
                    continue;
                }

                if c == *rep {
                    keys.push((*i1, k.clone()));
                    to_remove.push(idx);
                }
            }

            to_remove.reverse();
            for idx in &to_remove {
                potential_keys.remove(*idx);
            }

            if keys.len() >= n {
                break;
            }
        }
    }

    keys.sort_by_key(|(idx, _)| *idx);
    keys[n - 1].clone()
}

fn part_one() {
    let salt = "yjdafjpo";

    let result = nth_key(
        64,
        &Box::new(|i| format!("{:x}", md5::compute(format!("{salt}{i}")))),
    )
    .0;
    println!("{result}");
}

fn part_two() {
    let salt = "yjdafjpo";
    let iter_fn = |i: i32| {
        let data = format!("{salt}{i}");
        let mut curr = md5::compute(data);
        for _ in 0..2016 {
            curr = md5::compute(format!("{:x}", curr));
        }
        format!("{:x}", curr)
    };
    let result = nth_key(64, iter_fn).0;
    println!("{result}");
}

fn main() {
    part_two();
}
