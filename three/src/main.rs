#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;
use std::collections::HashSet;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, HasParser)]
struct Item(char);

impl Item {
    fn value(&self) -> u32 {
        if self.0.is_uppercase() {
            (self.0 as u32 - 'A' as u32) + 27
        } else {
            (self.0 as u32 - 'a' as u32) + 1
        }
    }
}

#[part_one]
fn part_one(input: List<List<Item, Nil>, TermWith<NewLine>>) -> u32 {
    let mut sum = 0;
    for items in input {
        let first_half: HashSet<Item> = items.iter().cloned().take(items.len() / 2).collect();
        let second_half: HashSet<Item> = items.iter().cloned().skip(items.len() / 2).collect();
        let common: Vec<Item> = first_half.intersection(&second_half).cloned().collect();
        assert_eq!(common.len(), 1, "{common:?}");
        sum += common[0].value();
    }
    sum
}

#[part_two]
fn part_two(input: List<List<Item, Nil>, TermWith<NewLine>>) -> u32 {
    let mut sum = 0;
    let mut iter = input.iter().peekable();
    while iter.peek().is_some() {
        let mut group = (&mut iter)
            .take(3)
            .map(|s| s.iter().cloned().collect::<HashSet<Item>>());

        let first: HashSet<Item> = group.next().unwrap();
        let common = group.fold(first, |a, b| a.intersection(&b).cloned().collect());
        let common: Vec<Item> = common.into_iter().collect();
        assert_eq!(common.len(), 1, "{common:?}");
        sum += common[0].value();
    }
    sum
}

harness!(part_1: 8240, part_2: 2587);
