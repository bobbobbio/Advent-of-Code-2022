#![feature(type_alias_impl_trait)]

use advent::prelude::*;
use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Copy, Debug)]
struct Crate(char);

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl HasParser for Crate {
    #[into_parser]
    fn parser() -> _ {
        char('[')
            .with(char::parser())
            .skip(char(']'))
            .map(|v| Self(v))
    }
}

#[derive(Debug, HasParser)]
enum EmptyOrCrate {
    #[parse(string = "   ")]
    Empty,
    Crate(Crate),
}

impl fmt::Display for EmptyOrCrate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Self::Crate(c) = self {
            write!(f, "{c}")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
struct Move {
    move_: u32,
    from: u32,
    to: u32,
}

impl HasParser for Move {
    #[into_parser]
    fn parser() -> _ {
        (
            string("move ").with(u32::parser()),
            string(" from ").with(u32::parser()),
            string(" to ").with(u32::parser()),
        )
            .map(|(move_, from, to)| Self { move_, from, to })
    }
}

#[derive(HasParser, Debug)]
struct Rows(List<List<EmptyOrCrate, SepBy<Space>>, SepBy<NewLine>>);

#[derive(HasParser, Debug)]
struct Moves(List<Move, TermWith<NewLine>>);

#[derive(Debug)]
struct Input {
    rows: Rows,
    moves: Moves,
}

impl Input {
    fn parse(input: String) -> Self {
        let lines: Vec<_> = input.split("\n").collect();
        let split = lines.iter().position(|l| l.is_empty()).unwrap() as usize;
        let first = lines[..(split - 1)].join("\n");
        let second = lines[(split + 1)..].join("\n");

        let rows: Rows = parse::parse_str(&first).unwrap();
        let moves: Moves = parse::parse_str(&second).unwrap();
        Self { rows, moves }
    }
}

#[derive(Debug)]
struct Board {
    column: Vec<VecDeque<Crate>>,
}

impl Board {
    fn new() -> Self {
        Self {
            column: vec![VecDeque::new(); 9],
        }
    }

    fn populate(&mut self, rows: &Rows) {
        for row in rows.0.iter().rev() {
            for (n, r) in row.iter().enumerate() {
                if let EmptyOrCrate::Crate(c) = r {
                    self.column[n].push_back(*c);
                }
            }
        }
    }

    fn move_(&mut self, move_: u32, from: u32, to: u32) {
        for _ in 0..move_ {
            let c = self.column[from as usize - 1].pop_back().unwrap();
            self.column[to as usize - 1].push_back(c);
        }
    }

    fn move2(&mut self, move_: u32, from: u32, to: u32) {
        let mut crates = vec![];
        for _ in 0..move_ {
            crates.push(self.column[from as usize - 1].pop_back().unwrap());
        }
        for c in crates.into_iter().rev() {
            self.column[to as usize - 1].push_back(c);
        }
    }

    fn word(&self) -> String {
        let tops: Vec<String> = self
            .column
            .iter()
            .map(|c| c.get(c.len() - 1).unwrap().to_string())
            .collect();

        tops.join("")
    }
}

#[part_one]
fn part_one(input: String) -> String {
    let input = Input::parse(input);
    let mut board = Board::new();
    board.populate(&input.rows);

    for m in &input.moves.0 {
        board.move_(m.move_, m.from, m.to);
    }

    board.word()
}

#[part_two]
fn part_two(input: String) -> String {
    let input = Input::parse(input);
    let mut board = Board::new();
    board.populate(&input.rows);

    for m in &input.moves.0 {
        board.move2(m.move_, m.from, m.to);
    }

    board.word()
}

harness!(part_1: "FZCMJCRHZ", part_2: "JSDHQMZGF");
