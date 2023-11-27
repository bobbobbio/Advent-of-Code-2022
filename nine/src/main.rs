#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, HasParser)]
enum Direction {
    #[parse(string = "U")]
    Up,
    #[parse(string = "D")]
    Down,
    #[parse(string = "L")]
    Left,
    #[parse(string = "R")]
    Right,
}

#[derive(HasParser)]
struct Step {
    direction: Direction,
    times: u32,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_(&mut self, d: Direction) {
        match d {
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
    }

    fn follow(&mut self, other: Self) {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        let diag_delta_abs = delta_x.abs() + delta_y.abs();

        if delta_x.abs() > 1 || diag_delta_abs > 2 {
            self.x += if delta_x < 0 { -1 } else { 1 };
        }

        if delta_y.abs() > 1 || diag_delta_abs > 2 {
            self.y += if delta_y < 0 { -1 } else { 1 };
        }
    }
}

#[part_one]
fn part_one(input: List<Step, TermWith<NewLine>>) -> usize {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    let mut tail_pos = HashSet::new();
    for i in input {
        for _ in 0..i.times {
            tail_pos.insert(tail);

            head.move_(i.direction);
            tail.follow(head);
        }
    }
    tail_pos.insert(tail);

    tail_pos.len()
}

#[part_two]
fn part_two(input: List<Step, TermWith<NewLine>>) -> usize {
    let mut chain = vec![Position { x: 0, y: 0 }; 10];

    let mut tail_pos = HashSet::new();
    for i in input {
        for _ in 0..i.times {
            tail_pos.insert(*chain.last().unwrap());

            chain[0].move_(i.direction);
            for i in 1..chain.len() {
                let prev = chain[i - 1];
                chain[i].follow(prev);
            }
        }
    }
    tail_pos.insert(*chain.last().unwrap());

    tail_pos.len()
}

harness!(part_1: 6212, part_2: 2522);
