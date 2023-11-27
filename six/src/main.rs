#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;
use std::collections::HashSet;

fn do_it(input: List<List<char, Nil>, TermWith<NewLine>>, marker_len: usize) -> usize {
    for packet in input {
        for i in 0..(packet.len() - marker_len) {
            let c: HashSet<_> = packet[i..(i + marker_len)].iter().collect();
            if c.len() == marker_len {
                return i + marker_len;
            }
        }
    }
    panic!();
}

#[part_one]
fn part_one(input: List<List<char, Nil>, TermWith<NewLine>>) -> usize {
    do_it(input, 4)
}

#[part_two]
fn part_two(input: List<List<char, Nil>, TermWith<NewLine>>) -> usize {
    do_it(input, 14)
}

harness!(part_1: 1198, part_2: 3120);
