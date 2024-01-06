use std::{ffi::IntoStringError, io::stdin};

use itertools::Itertools;
use Instruction::*;
use Operand::*;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(usize),
    Int(i32),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Copy(Operand, usize),
    Increment(usize),
    Decrement(usize),
    JumpNotZero(Operand, i32),
}

fn register_idx(c: char) -> usize {
    return (c as usize) - ('a' as usize);
}

fn parse_register(s: &str) -> Option<usize> {
    if s.starts_with(|c: char| c.is_alphabetic() && c.is_lowercase() && s.len() == 1) {
        return Some(register_idx(s.chars().nth(0).unwrap()));
    }
    None
}

fn parse_operand(s: &str) -> Option<Operand> {
    if s.starts_with(|c: char| c.is_digit(10)) {
        return s.parse::<i32>().ok().map(|v| Int(v));
    } else if let Some(idx) = parse_register(s) {
        return Some(Register(idx));
    }
    None
}

fn read_input() -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let parts = line.split_ascii_whitespace().collect_vec();
        let instr = match parts[0] {
            "cpy" => Copy(
                parse_operand(parts[1]).expect(format!("invalid operand: {}", parts[1]).as_str()),
                parse_register(parts[2]).expect(format!("invalid regiser: {}", parts[2]).as_str()),
            ),
            "inc" => Increment(
                parse_register(parts[1]).expect(format!("invalid register: {}", parts[1]).as_str()),
            ),
            "dec" => Decrement(
                parse_register(parts[1]).expect(format!("invalid register: {}", parts[1]).as_str()),
            ),
            "jnz" => JumpNotZero(
                parse_operand(parts[1]).expect(format!("invalid operand: {}", parts[1]).as_str()),
                parts[2].parse::<i32>().unwrap(),
            ),
            _ => todo!(),
        };

        result.push(instr);
    }

    result
}

const REGISTERS_CNT: usize = 4;
type State = [i32; REGISTERS_CNT];

fn execute(program: &Vec<Instruction>, start: State) -> State {
    let mut current = start;
    let mut ip: i32 = 0;

    while ip >= 0 && ip < program.len() as i32 {
        let instr = program[ip as usize];
        match instr {
            Copy(Register(idx), target) => current[target] = current[idx],
            Copy(Int(v), target) => current[target] = v,
            Increment(target) => current[target] += 1,
            Decrement(target) => current[target] -= 1,
            JumpNotZero(operand, amount) => {
                let to_check = match operand {
                    Register(idx) => current[idx],
                    Int(v) => v,
                };
                if to_check != 0 {
                    ip += amount - 1;
                }
            }
        }

        ip += 1;
    }

    current
}

fn part_one() {
    let instructions = read_input();
    let final_state = execute(&instructions, [0; REGISTERS_CNT]);
    let result = final_state[register_idx('a')];
    println!("{result}");
}

fn part_two() {
    let instructions = read_input();
    let mut start_state = [0; REGISTERS_CNT];
    start_state[register_idx('c')] = 1;
    let final_state = execute(&instructions, start_state);
    let result = final_state[register_idx('a')];
    println!("{result}");
}

fn main() {
    part_two();
}
