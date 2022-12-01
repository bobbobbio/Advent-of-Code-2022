#![feature(type_alias_impl_trait)]

use advent::prelude::*;

#[part_one]
fn part_one(input: List<List<u32, NewLine>, SepByNewLine>) -> u32 {
    let mut higest_elf_calories = 0;
    for l in input {
        higest_elf_calories = std::cmp::max(l.into_iter().sum(), higest_elf_calories);
    }
    higest_elf_calories
}

#[part_two]
fn part_two(input: List<List<u32, NewLine>, SepByNewLine>) -> u32 {
    let mut elf_calories: Vec<u32> = input.into_iter().map(|l| l.into_iter().sum()).collect();
    elf_calories.sort_by_key(|&k| std::cmp::Reverse(k));
    elf_calories.into_iter().take(3).sum()
}

harness!();
