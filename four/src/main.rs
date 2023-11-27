#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;

#[derive(HasParser, Debug, Clone)]
#[parse(sep_by = "-")]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start <= other.end && self.start >= other.start)
            || (self.end >= other.start && self.end <= other.end)
    }
}

#[derive(HasParser, Debug, Clone)]
#[parse(sep_by = ",")]
struct TwoRanges(Range, Range);

#[part_one]
fn part_one(l: List<TwoRanges, TermWith<NewLine>>) -> usize {
    l.iter()
        .filter(|t| t.0.contains(&t.1) || t.1.contains(&t.0))
        .count()
}

#[part_two]
fn part_two(l: List<TwoRanges, TermWith<NewLine>>) -> usize {
    l.iter()
        .filter(|t| t.0.overlaps(&t.1) || t.1.overlaps(&t.0))
        .count()
}

harness!(part_1: 534, part_2: 841);
