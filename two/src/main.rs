#![feature(type_alias_impl_trait)]

use advent::prelude::*;

#[derive(Copy, Clone)]
#[repr(u32)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, HasParser)]
#[repr(u32)]
enum Outcome {
    #[parse(string = "Z")]
    Win = 6,
    #[parse(string = "Y")]
    Draw = 3,
    #[parse(string = "X")]
    Lose = 0,
}

impl Outcome {
    fn to_win(&self, opponent: Play) -> Play {
        match (*self, opponent) {
            (Self::Win, Play::Rock) => Play::Paper,
            (Self::Win, Play::Paper) => Play::Scissors,
            (Self::Win, Play::Scissors) => Play::Rock,
            (Self::Lose, Play::Rock) => Play::Scissors,
            (Self::Lose, Play::Paper) => Play::Rock,
            (Self::Lose, Play::Scissors) => Play::Paper,
            (Self::Draw, p) => p,
        }
    }
}

impl HasParser for Play {
    #[into_parser]
    fn parser() -> _ {
        choice((
            char('A').map(|_| Self::Rock),
            char('B').map(|_| Self::Paper),
            char('C').map(|_| Self::Scissors),
            char('X').map(|_| Self::Rock),
            char('Y').map(|_| Self::Paper),
            char('Z').map(|_| Self::Scissors),
        ))
    }
}

impl Play {
    fn vs(&self, other: Self) -> Outcome {
        match (*self, other) {
            (Self::Rock, Self::Paper) => Outcome::Lose,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Scissors) => Outcome::Lose,
            (Self::Scissors, Self::Rock) => Outcome::Lose,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

#[derive(HasParser)]
struct Entry1 {
    opponent: Play,
    mine: Play,
}

impl Entry1 {
    fn score(&self) -> u32 {
        self.mine as u32 + self.mine.vs(self.opponent) as u32
    }
}

#[part_one]
fn part_one(i: List<Entry1, TermWith<NewLine>>) -> u32 {
    i.into_iter().map(|e| e.score()).sum()
}

#[derive(HasParser)]
struct Entry2 {
    opponent: Play,
    outcome: Outcome,
}

impl Entry2 {
    fn score(&self) -> u32 {
        self.outcome as u32 + self.outcome.to_win(self.opponent) as u32
    }
}

#[part_two]
fn part_two(i: List<Entry2, TermWith<NewLine>>) -> u32 {
    i.into_iter().map(|e| e.score()).sum()
}

harness!(part_1: 10310, part_2: 14859);
