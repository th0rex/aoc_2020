#![feature(never_type)]

use std::{
    collections::HashSet,
    str::FromStr,
};

enum Insn {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Insn {
    fn can_flip(&self) -> bool {
        use Insn::*;

        match self {
            Jmp(_) | Nop(_) => true,
            _ => false,
        }
    }

    fn flip(&mut self) {
        use Insn::*;

        match self {
            Jmp(x) => *self = Nop(*x),
            Nop(x) => *self = Jmp(*x),
            _ => {},
        }
    }
}

impl FromStr for Insn {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Insn::*;

        let idx = s.find(" ").unwrap();
        let operand = s[idx + 1..].parse::<isize>().unwrap();
        Ok(match &s[..idx] {
            "acc" => Acc(operand),
            "jmp" => Jmp(operand),
            "nop" => Nop(operand),
            _ => unreachable!(),
        })
    }
}

fn emulate(code: &[Insn]) -> (isize, isize) {
    use Insn::*;

    let mut seen = HashSet::new();

    let mut acc = 0isize;
    let mut pc = 0isize;

    while seen.insert(pc) && pc as usize != code.len() {
        match code[pc as usize] {
            Acc(x) => {
                acc += x;
                pc += 1
            },
            Jmp(x) => pc += x,
            Nop(_) => pc += 1,
        }
    }

    (acc, pc)
}

fn main() {
    let mut code = include_str!("inputs/day_8.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<Insn>>();

    println!("{:?}", emulate(&code[..]));

    for i in 0..code.len() {
        if code[i].can_flip() {
            code[i].flip();
        }

        let (acc, pc) = emulate(&code[..]);

        if pc as usize == code.len() {
            println!("{}", acc); break;
        }

        if code[i].can_flip() {
            code[i].flip();
        }
    }
}
