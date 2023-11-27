#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter;

#[derive(Clone)]
struct Tree(u64);

impl HasParser for Tree {
    #[into_parser]
    fn parser() -> _ {
        digit().map(|d| Self(d.to_string().parse().unwrap()))
    }
}

#[derive(HasParser)]
struct Grid(List<List<Tree, Nil>, TermWith<NewLine>>);

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let row = iter::repeat(Tree(1)).take(width).collect();
        Self(iter::repeat(row).take(height).collect())
    }

    fn get(&self, x: usize, y: usize) -> u64 {
        self.0[y][x].0
    }

    fn set(&mut self, x: usize, y: usize, value: u64) {
        self.0[y][x].0 = value;
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }
}

fn find_visible(
    input: &Grid,
    iter: impl Iterator<Item = (usize, usize)>,
    visible: &mut HashSet<(usize, usize)>,
) {
    let mut tallest = -1;

    for (x, y) in iter {
        let tree = input.get(x, y) as i64;
        if tree > tallest {
            tallest = tree;
            visible.insert((x, y));
        }
    }
}

#[part_one]
fn part_one(input: Grid) -> usize {
    let mut visible = HashSet::new();

    for y in 0..input.height() {
        find_visible(&input, (0..input.width()).map(|x| (x, y)), &mut visible);
        find_visible(
            &input,
            (0..input.width()).rev().map(|x| (x, y)),
            &mut visible,
        );
    }

    for x in 0..input.width() {
        find_visible(&input, (0..input.height()).map(|y| (x, y)), &mut visible);
        find_visible(
            &input,
            (0..input.height()).rev().map(|y| (x, y)),
            &mut visible,
        );
    }

    visible.len()
}

fn subtract_maps(a: &mut HashMap<i64, u64>, b: &HashMap<i64, u64>) {
    for v in 0..=9 {
        *a.entry(v).or_insert(0) -= b.get(&v).unwrap_or(&0);
    }
}

fn calculate_viewscore(
    input: &Grid,
    iter: impl Iterator<Item = (usize, usize)>,
    scores: &mut Grid,
) {
    let mut trees_under = HashMap::new();
    let mut stack = vec![];

    let mut last_height = HashMap::new();
    for (n, (x, y)) in iter.enumerate() {
        let tree = input.get(x, y) as i64;

        let mut blocking_tree = false;
        let mut my_trees_under = trees_under.clone();
        for h in tree..=9 {
            if let Some(past_n) = last_height.get(&h) {
                subtract_maps(&mut my_trees_under, &stack[*past_n]);
                blocking_tree = true;
                break;
            }
        }

        let mut num_trees_viewable = *my_trees_under.get(&tree).unwrap_or(&0);
        if blocking_tree {
            num_trees_viewable += 1;
        }
        scores.set(x, y, scores.get(x, y) * num_trees_viewable);

        for v in tree..=9 {
            *trees_under.entry(v).or_insert(0) += 1;
        }
        stack.push(trees_under.clone());
        last_height.insert(tree, n);
    }
}

#[part_two]
fn part_two(input: Grid) -> u64 {
    let mut scores = Grid::new(input.width(), input.height());

    for y in 0..input.height() {
        calculate_viewscore(&input, (0..input.width()).map(|x| (x, y)), &mut scores);
        calculate_viewscore(
            &input,
            (0..input.width()).rev().map(|x| (x, y)),
            &mut scores,
        );
    }

    for x in 0..input.width() {
        calculate_viewscore(&input, (0..input.height()).map(|y| (x, y)), &mut scores);
        calculate_viewscore(
            &input,
            (0..input.height()).rev().map(|y| (x, y)),
            &mut scores,
        );
    }

    let mut best_score = 0;
    for y in 0..scores.height() {
        for x in 0..scores.width() {
            let score = scores.get(x, y);
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

harness!(part_1: 1816, part_2: 383520);
