#![feature(type_alias_impl_trait)]

use advent::prelude::*;
use std::collections::HashMap;

#[derive(HasParser)]
#[parse(before = "throw to monkey ")]
struct ThrowTo(u128);

#[derive(HasParser)]
#[parse(sep_by = "\n")]
struct Test {
    #[parse(before = "divisible by ")]
    divisible_by: u128,
    #[parse(before = "    If true: ")]
    if_true: ThrowTo,
    #[parse(before = "    If false: ")]
    if_false: ThrowTo,
}

impl Test {
    fn apply(&self, worry: u128) -> u128 {
        if worry % self.divisible_by == 0 {
            self.if_true.0
        } else {
            self.if_false.0
        }
    }
}

#[derive(HasParser)]
enum Op {
    #[parse(string = "+")]
    Plus,
    #[parse(string = "*")]
    Times,
}

impl Op {
    fn apply(&self, gcm: u128, a: u128, b: u128) -> u128 {
        match self {
            Self::Plus => a + b,
            Self::Times => (a % gcm) * (b % gcm),
        }
    }
}

#[derive(HasParser)]
enum OldOrValue {
    #[parse(string = "old")]
    Old,
    Value(u128),
}

#[derive(HasParser)]
#[parse(before = "new = ")]
struct Operation {
    a: OldOrValue,
    op: Op,
    b: OldOrValue,
}

impl Operation {
    fn apply(&self, gcm: u128, v: u128) -> u128 {
        use OldOrValue::*;
        match (&self.a, &self.op, &self.b) {
            (Old, op, Old) => op.apply(gcm, v, v),
            (Old, op, Value(b)) => op.apply(gcm, v, *b),
            (Value(a), op, Old) => op.apply(gcm, *a, v),
            (Value(a), op, Value(b)) => op.apply(gcm, *a, *b),
        }
    }
}

#[derive(HasParser)]
#[parse(sep_by = "\n")]
struct Monkey {
    #[parse(before = "Monkey ", after = ":")]
    number: u128,
    #[parse(before = "  Starting items: ")]
    items: List<u128, SepBy<CommaSpace>>,
    #[parse(before = "  Operation: ")]
    operation: Operation,
    #[parse(before = "  Test: ", after = "\n")]
    test: Test,
}

impl Monkey {
    fn round(&mut self, divide_worry: bool, gcm: u128) -> Vec<(u128, u128)> {
        let mut res = vec![];
        for i in &self.items {
            let mut worry = self.operation.apply(gcm, *i);
            if divide_worry {
                worry /= 3;
            }
            let new_monkey = self.test.apply(worry);
            res.push((new_monkey, worry));
        }
        self.items = List::new();
        res
    }
}

fn run_rounds(input: List<Monkey, SepBy<NewLine>>, divide_worry: bool, rounds: usize) -> usize {
    let num_monkeys = input.len() as u128;

    let mut monkeys = HashMap::new();

    let mut gcm = 1;
    for m in input {
        if gcm % m.test.divisible_by != 0 {
            gcm *= m.test.divisible_by;
        }

        monkeys.insert(m.number, m);
    }

    let mut monkey_stats = HashMap::new();

    for _ in 0..rounds {
        for num in 0..num_monkeys {
            let m = monkeys.get_mut(&num).unwrap();
            let res = m.round(divide_worry, gcm);
            *monkey_stats.entry(num).or_insert(0) += res.len();
            for (m_num, worry) in res {
                let m = monkeys.get_mut(&m_num).unwrap();
                m.items.push(worry);
            }
        }
    }

    let mut counts: Vec<usize> = monkey_stats.values().copied().collect();
    counts.sort();

    counts[counts.len() - 1] * counts[counts.len() - 2]
}

#[part_one]
fn part_one(input: List<Monkey, SepBy<NewLine>>) -> usize {
    run_rounds(input, true, 20)
}

#[part_two]
fn part_two(input: List<Monkey, SepBy<NewLine>>) -> usize {
    run_rounds(input, false, 10_000)
}

harness!(part_1: 95472, part_2: 17926061332);
