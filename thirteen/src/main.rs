#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct PacketList(Vec<Packet>);

combine::parser! {
    fn packet_list[Input]()(Input) -> PacketList
    where [Input: Stream<Token = char>]
    {
        sep_by(Packet::parser(), char(',')).map(PacketList)
    }
}

impl HasParser for PacketList {
    #[into_parser]
    fn parser() -> _ {
        packet_list()
    }
}

#[derive(HasParser, Clone, Debug, PartialEq, Ord, Eq)]
enum Packet {
    #[parse(before = "[", after = "]")]
    List(PacketList),
    Number(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => a.partial_cmp(b),
            (a @ Packet::Number(_), Packet::List(b)) => PacketList(vec![a.clone()]).partial_cmp(b),
            (Packet::List(a), b @ Packet::Number(_)) => a.partial_cmp(&PacketList(vec![b.clone()])),
        }
    }
}

#[derive(HasParser, Debug)]
#[parse(sep_by = "\n", after = "\n")]
struct PacketPair {
    left: Packet,
    right: Packet,
}

#[part_one]
fn part_one(input: List<PacketPair, SepBy<NewLine>>) -> usize {
    let mut idx_sum = 0;
    for (i, pair) in input.into_iter().enumerate() {
        if pair.left <= pair.right {
            idx_sum += i + 1;
        }
    }
    idx_sum
}

#[part_two]
fn part_two(input: List<PacketPair, SepBy<NewLine>>) -> usize {
    let mut all_packets: Vec<_> = input
        .iter()
        .map(|p| [p.left.clone(), p.right.clone()])
        .flatten()
        .collect();

    let divider1: Packet = parse::parse_str("[[2]]").unwrap();
    let divider2: Packet = parse::parse_str("[[6]]").unwrap();

    all_packets.push(divider1.clone());
    all_packets.push(divider2.clone());

    all_packets.sort();

    let pos1 = all_packets.iter().position(|p| p == &divider1).unwrap() + 1;
    let pos2 = all_packets.iter().position(|p| p == &divider2).unwrap() + 1;

    pos1 * pos2
}

harness!(part_1: 5252, part_2: 20592);
