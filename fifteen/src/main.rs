#![feature(type_alias_impl_trait)]

use advent::prelude::*;
use range_collections::range_set::RangeSet2;
use range_collections::AbstractRangeSet as _;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, HasParser, Default, Hash, PartialEq, Eq)]
#[parse(sep_by = ", ")]
struct Coordinate {
    #[parse(before = "x=")]
    x: i32,
    #[parse(before = "y=")]
    y: i32,
}

impl Coordinate {
    fn distance(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
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

impl std::ops::SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

#[derive(HasParser, Debug)]
#[parse(sep_by = ": ")]
struct SensorData {
    #[parse(before = "Sensor at ")]
    sensor_pos: Coordinate,

    #[parse(before = "closest beacon is at ")]
    closest_beacon: Coordinate,
}

#[part_one]
fn part_one(input: List<SensorData, TermWith<NewLine>>) -> usize {
    let mut beacons = HashSet::new();
    for sd in &input {
        beacons.insert(sd.closest_beacon);
    }

    let lowest_x = input
        .iter()
        .map(|sd| sd.sensor_pos.x - sd.sensor_pos.distance(sd.closest_beacon))
        .min()
        .unwrap();

    let highest_x = input
        .iter()
        .map(|sd| sd.sensor_pos.x + sd.sensor_pos.distance(sd.closest_beacon))
        .max()
        .unwrap();

    let mut count = 0;
    for x in lowest_x..=highest_x {
        let c = Coordinate { x, y: 2000000 };
        if beacons.contains(&c) {
            continue;
        }
        for sd in &input {
            if sd.sensor_pos.distance(c) <= sd.sensor_pos.distance(sd.closest_beacon) {
                count += 1;
                break;
            }
        }
    }
    count
}

#[part_two]
fn part_two(input: List<SensorData, TermWith<NewLine>>) -> u64 {
    let mut rows: BTreeMap<i32, RangeSet2<i32>> = BTreeMap::new();
    for sd in input {
        let dist = sd.sensor_pos.distance(sd.closest_beacon);
        let start_y = sd.sensor_pos.y - dist;
        let end_y = sd.sensor_pos.y + dist;
        for y in start_y..=end_y {
            let y_dist = (sd.sensor_pos.y - y).abs();
            let x_dist = dist - y_dist;
            let range = (sd.sensor_pos.x - x_dist)..(sd.sensor_pos.x + x_dist + 1);
            rows.entry(y)
                .or_insert(RangeSet2::empty())
                .union_with(&RangeSet2::from(range));
        }
    }
    let search_area = 4_000_000;

    for y in 0..=search_area {
        let row = rows.get(&y).unwrap();
        let interest = RangeSet2::from(0..(search_area + 1));
        if row.intersection::<[i32; 2]>(&interest) == &interest {
            continue;
        }
        for x in 0..=search_area {
            if !row.contains(&x) {
                return (x as u64 * 4_000_000) + y as u64;
            }
        }
    }
    panic!("not found");
}

harness!(part_1: 5511201, part_2: 11318723411840);
