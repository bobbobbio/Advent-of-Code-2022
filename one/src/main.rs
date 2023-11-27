#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;

#[part_one]
fn part_one(input: List<List<u32, TermWith<NewLine>>, SepBy<NewLine>>) -> u32 {
    input
        .into_iter()
        .map(|l| l.into_iter().sum())
        .max()
        .unwrap()
}

#[part_two]
fn part_two(input: List<List<u32, TermWith<NewLine>>, SepBy<NewLine>>) -> u32 {
    let mut elf_calories: Vec<u32> = input.into_iter().map(|l| l.into_iter().sum()).collect();
    elf_calories.sort_by_key(|&k| std::cmp::Reverse(k));
    elf_calories.into_iter().take(3).sum()
}

harness!(part_1: 67622, part_2: 201491);
