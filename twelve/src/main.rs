#![feature(type_alias_impl_trait)]

use advent::prelude::*;
use std::cmp;
use std::collections::{BinaryHeap, HashSet};

struct Graph {
    nodes: Vec<Vec<i32>>,
}

impl Graph {
    fn new(width: usize, height: usize) -> Self {
        Self {
            nodes: vec![vec![0; width]; height],
        }
    }

    fn get(&self, i: (i32, i32)) -> Option<i32> {
        if (i.1 as usize) < self.nodes.len() && (i.0 as usize) < self.nodes[0].len() {
            Some(self.nodes[i.1 as usize][i.0 as usize])
        } else {
            None
        }
    }

    fn shortest_path(&mut self, start: (i32, i32), end: (i32, i32)) -> Option<i32> {
        let mut distance = 0;
        let mut curr = start;

        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        while curr != end {
            let value = self.get(curr).unwrap();
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let next = (curr.0 + dx, curr.1 + dy);
                if visited.contains(&next) {
                    continue;
                }

                if let Some(next_value) = self.get(next) {
                    if next_value - value <= 1 {
                        heap.push(cmp::Reverse((distance + 1, next)));
                    }
                }
            }
            assert!(visited.insert(curr));

            loop {
                if let Some(cmp::Reverse((next_distance, next))) = heap.pop() {
                    if visited.contains(&next) {
                        continue;
                    }
                    distance = next_distance;
                    curr = next;
                    break;
                } else {
                    return None;
                }
            }
        }

        Some(distance)
    }

    fn build(list: List<List<char, Nil>, TermWith<NewLine>>) -> ((i32, i32), (i32, i32), Self) {
        let mut g = Graph::new(list[0].len(), list.len());

        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in list.into_iter().enumerate() {
            for (x, h) in line.into_iter().enumerate() {
                if h == 'S' {
                    start = (x as i32, y as i32);
                } else if h == 'E' {
                    end = (x as i32, y as i32)
                } else {
                    g.nodes[y][x] = h as i32 - 'a' as i32;
                }
            }
        }
        g.nodes[end.1 as usize][end.0 as usize] = 'z' as i32 - 'a' as i32;

        (start, end, g)
    }
}

#[part_one]
fn part_one(list: List<List<char, Nil>, TermWith<NewLine>>) -> i32 {
    let (start, end, mut g) = Graph::build(list);
    g.shortest_path(start, end).unwrap()
}

#[part_two]
fn part_two(list: List<List<char, Nil>, TermWith<NewLine>>) -> i32 {
    let (_, end, mut g) = Graph::build(list);

    let mut distances = vec![];
    for y in 0..g.nodes.len() as i32 {
        for x in 0..g.nodes[0].len() as i32 {
            let p = (x, y);
            if g.get(p).unwrap() == 0 {
                if let Some(d) = g.shortest_path(p, end) {
                    distances.push(d);
                }
            }
        }
    }
    distances.into_iter().min().unwrap()
}

harness!(part_1: 370, part_2: 363);
