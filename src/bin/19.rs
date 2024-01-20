use std::collections::VecDeque;

fn simulate_1(n: usize) -> usize {
    let mut cnts = vec![0; n];
    for i in 0..n {
        cnts[i] = i + 1;
    }

    while cnts.len() > 1 {
        let mut to_remove = vec![0; cnts.len()];
        for i in (0..cnts.len()).step_by(2) {
            to_remove[(i + 1) % cnts.len()] = 1;
        }

        let mut next_cnts = Vec::new();
        for i in 0..cnts.len() {
            if to_remove[i] == 0 {
                next_cnts.push(cnts[i]);
            }
        }
        cnts = next_cnts;
    }

    cnts[0]
}

fn J(n: usize) -> usize {
    if n == 1 || n == 2 {
        return 1;
    }
    match n % 2 {
        0 => 2 * J(n / 2) - 1,
        1 => 2 * J(n / 2) + 1,
        _ => todo!(),
    }
}

fn part_one() {
    let input = 3014387;
    let result = J(input);
    println!("{result}");
}

fn simulate_2(n: usize) -> usize {
    let mut first_half = (1..=n / 2).collect::<VecDeque<_>>();
    let mut second_half = (n / 2 + 1..=n).collect::<VecDeque<_>>();

    for _ in 0..n - 1 {
        if second_half.len() >= first_half.len() {
            second_half.pop_front();
        } else {
            first_half.pop_back();
        }

        second_half.push_back(first_half.pop_front().unwrap());
        first_half.push_back(second_half.pop_front().unwrap());
    }

    first_half[0]
}

fn calculate_2(n: usize) -> usize {
    let mut result = vec![0; n + 1];
    result[2] = 1;

    let mut to = 3;
    let mut cnt = 1;

    'out: while to <= n {
        for _ in 0..cnt {
            result[to] = result[to - 1] + 2;
            to += 1;

            if to > n {
                break 'out;
            }
        }

        cnt = result[to - 1];
        for i in 1..=cnt {
            result[to] = i;
            to += 1;

            if to > n {
                break 'out;
            }
        }
    }

    result[n]
}

fn calculate_2_1(n: usize) -> usize {
    let mut pow = 1;
    while 3 * pow <= n {
        pow *= 3;
    }
    if n == pow {
        return n;
    }
    if n - pow <= pow {
        return n - pow;
    }
    2 * n - 3 * pow
}

fn part_two() {
    let input = 3014387;
    let result = calculate_2(input);

    println!("{result}");
}

fn main() {
    part_two();
}
