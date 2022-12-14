#![feature(type_alias_impl_trait)]

use advent::prelude::*;
use std::fmt;

#[derive(Clone, Copy, HasParser)]
#[parse(sep_by = ",")]
struct Coordinate {
    x: usize,
    y: usize,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl std::ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Coordinate {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct CoordinateList(Vec<Coordinate>);

impl HasParser for CoordinateList {
    #[into_parser]
    fn parser() -> _ {
        sep_by(Coordinate::parser(), string(" -> ")).map(Self)
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl Tile {
    fn is_air(&self) -> bool {
        std::matches!(self, Self::Air)
    }
}

#[derive(Default)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    floor: Option<usize>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content_start = self.content_start();
        for y in content_start.y..self.height() {
            for x in content_start.x..self.width() {
                let s = match self.get(Coordinate { x, y }).unwrap() {
                    Tile::Air => ".",
                    Tile::Rock => "#",
                    Tile::Sand => "+",
                };
                write!(f, "{s}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum SandMoveError {
    Stopped,
    Infinity,
}

impl Map {
    fn from_input(input: List<CoordinateList, TermWith<NewLine>>) -> Self {
        let mut map = Self::default();

        for line in input {
            let mut iter = line.0.into_iter();
            let mut start = iter.next().unwrap();
            while let Some(end) = iter.next() {
                map.rock_line(start, end);
                start = end;
            }
        }

        map
    }

    fn content_start(&self) -> Coordinate {
        Coordinate {
            x: self
                .tiles
                .iter()
                .map(|l| l.iter().position(|t| !t.is_air()).unwrap_or(usize::MAX))
                .min()
                .unwrap(),
            y: 0,
        }
    }

    fn width(&self) -> usize {
        self.tiles.iter().map(|l| l.len()).max().unwrap_or(0)
    }

    fn height(&self) -> usize {
        std::cmp::max(self.tiles.len(), self.floor.unwrap_or(0) + 1)
    }

    fn maybe_grow(&mut self, width: usize, height: usize) {
        while height > self.tiles.len() {
            self.tiles.push(vec![]);
        }
        let max_width = std::cmp::max(self.width(), width);
        for i in 0..self.tiles.len() {
            while max_width > self.tiles[i].len() {
                self.tiles[i].push(Tile::Air);
            }
        }
    }

    fn get(&self, c: Coordinate) -> Option<Tile> {
        if let Some(floor) = &self.floor {
            if c.y == *floor {
                return Some(Tile::Rock);
            }
            if c.y >= self.tiles.len() && c.y < *floor {
                return Some(Tile::Air);
            }
        }

        if c.y >= self.tiles.len() {
            return None;
        }
        if c.x >= self.tiles[c.y].len() {
            return Some(Tile::Air);
        }

        Some(self.tiles[c.y][c.x])
    }

    fn set(&mut self, c: Coordinate, tile: Tile) {
        self.maybe_grow(c.x + 1, c.y + 1);
        self.tiles[c.y][c.x] = tile;
    }

    fn rock_line(&mut self, start: Coordinate, end: Coordinate) {
        let mut c = start;
        self.set(c, Tile::Rock);
        if c.x == end.x {
            while c.y < end.y {
                c.y += 1;
                self.set(c, Tile::Rock);
            }
            while c.y > end.y {
                c.y -= 1;
                self.set(c, Tile::Rock);
            }
        } else {
            assert_eq!(c.y, end.y);
            while c.x < end.x {
                c.x += 1;
                self.set(c, Tile::Rock);
            }
            while c.x > end.x {
                c.x -= 1;
                self.set(c, Tile::Rock);
            }
        }
    }

    fn try_move_down(&mut self, c: Coordinate) -> std::result::Result<Coordinate, SandMoveError> {
        let mut new = c + Coordinate { x: 0, y: 1 };
        match self.get(new) {
            Some(v) if v.is_air() => return Ok(new),
            None => return Err(SandMoveError::Infinity),
            _ => (),
        }

        new = (c + Coordinate { x: 0, y: 1 }) - Coordinate { x: 1, y: 0 };
        match self.get(new) {
            Some(v) if v.is_air() => return Ok(new),
            None => return Err(SandMoveError::Infinity),
            _ => (),
        }

        new = c + Coordinate { x: 1, y: 1 };
        match self.get(new) {
            Some(v) if v.is_air() => return Ok(new),
            None => return Err(SandMoveError::Infinity),
            _ => (),
        }

        Err(SandMoveError::Stopped)
    }

    fn add_sand(&mut self, start: Coordinate) -> bool {
        let mut c = start;
        if !self.get(c).unwrap().is_air() {
            return false;
        }

        loop {
            match self.try_move_down(c) {
                Ok(new) => c = new,
                Err(SandMoveError::Infinity) => return false,
                Err(SandMoveError::Stopped) => break,
            }
        }

        self.set(c, Tile::Sand);

        true
    }
}

#[part_one]
fn part_one(input: List<CoordinateList, TermWith<NewLine>>) -> u32 {
    let mut map = Map::from_input(input);

    let mut sand = 0;
    while map.add_sand(Coordinate { x: 500, y: 0 }) {
        sand += 1;
    }

    sand
}

#[part_two]
fn part_two(input: List<CoordinateList, TermWith<NewLine>>) -> u32 {
    let highest = input
        .iter()
        .map(|l| l.0.iter().map(|c| c.y).max().unwrap_or(0))
        .max()
        .unwrap();

    let mut map = Map::from_input(input);

    let floor_y = highest + 2;
    map.floor = Some(floor_y);

    let mut sand = 0;
    while map.add_sand(Coordinate { x: 500, y: 0 }) {
        sand += 1;
    }

    sand
}

harness!(part_1: 1061, part_2: 25055);
