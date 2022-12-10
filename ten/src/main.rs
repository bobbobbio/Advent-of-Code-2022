#![feature(type_alias_impl_trait)]

use advent::prelude::*;
use std::fmt::Write as _;

#[derive(HasParser)]
enum Instruction {
    #[parse(string = "noop")]
    Noop,
    #[parse(before = "addx ")]
    Addx(i32),
}

struct Machine {
    signal_strengths: Vec<i32>,
    x_reg: i32,
    cycle: i32,
    pc: usize,
    mid_instruction: bool,
}

impl Machine {
    fn new() -> Self {
        Self {
            signal_strengths: vec![0; 300],
            x_reg: 1,
            cycle: 1,
            pc: 0,
            mid_instruction: false,
        }
    }

    fn execute(&mut self, instr: &[Instruction]) {
        if let Instruction::Addx(v) = instr[self.pc] {
            if self.mid_instruction {
                self.cycle += 1;
                self.x_reg += v;
                self.signal_strengths[self.cycle as usize] = self.cycle * self.x_reg;
                self.mid_instruction = false;
                self.pc += 1;
            } else {
                self.cycle += 1;
                self.signal_strengths[self.cycle as usize] = self.cycle * self.x_reg;
                self.mid_instruction = true;
            }
        } else {
            self.cycle += 1;
            self.signal_strengths[self.cycle as usize] = self.cycle * self.x_reg;
            self.pc += 1;
        }
    }
}

struct Crt {
    x: i32,
    s: String,
}

impl Crt {
    fn new() -> Self {
        Self {
            x: 1,
            s: String::from("\n"),
        }
    }

    fn draw(&mut self, x_reg: i32) {
        if self.x >= x_reg && self.x < x_reg + 3 {
            write!(&mut self.s, "#").unwrap();
        } else {
            write!(&mut self.s, ".").unwrap();
        }
        self.x += 1;

        if self.x == 41 {
            self.x = 1;
            writeln!(&mut self.s).unwrap();
        }
    }
}

#[part_one]
fn part_one(input: List<Instruction, TermWith<NewLine>>) -> i32 {
    let mut m = Machine::new();
    while m.pc < input.len() {
        m.execute(&input[..]);
    }

    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|i| m.signal_strengths[i])
        .sum()
}

#[part_two]
fn part_two(input: List<Instruction, TermWith<NewLine>>) -> String {
    let mut m = Machine::new();
    let mut crt = Crt::new();
    while m.pc < input.len() {
        crt.draw(m.x_reg);
        m.execute(&input[..]);
    }

    crt.s
}

#[cfg(test)]
const PART_2: &'static str = "
####..##..####.#..#.####..##..#....###..
#....#..#....#.#..#....#.#..#.#....#..#.
###..#......#..#..#...#..#..#.#....#..#.
#....#.....#...#..#..#...####.#....###..
#....#..#.#....#..#.#....#..#.#....#.#..
####..##..####..##..####.#..#.####.#..#.
";

harness!(part_1: 16020, part_2: PART_2);
