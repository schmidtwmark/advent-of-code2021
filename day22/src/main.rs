use itertools::Itertools;
use lazy_static::lazy_static;
use num::{Num, PrimInt, Signed};
use regex::Regex;
use std::{collections::HashMap, collections::HashSet, env, fs, ops::RangeInclusive};

#[derive(Debug)]
struct Command<T> {
    range: Range3D<T>,
    turn_on: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Range3D<T> {
    x: RangeInclusive<T>,
    y: RangeInclusive<T>,
    z: RangeInclusive<T>,
}

impl<T: Signed + PrimInt + Num + std::cmp::PartialOrd + std::cmp::Ord + Copy + std::fmt::Debug>
    Range3D<T>
{
    fn range_overlap(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> Option<RangeInclusive<T>> {
        let left = std::cmp::max(*a.start(), *b.start());
        let right = std::cmp::min(*a.end(), *b.end());
        if left > right {
            None
        } else {
            Some(left..=right)
        }
    }

    fn intersect(&self, other: &Range3D<T>) -> Option<Range3D<T>> {
        // Returns the volume contained in both
        if let Some(new_x) = Range3D::range_overlap(&self.x, &other.x) {
            if let Some(new_y) = Range3D::range_overlap(&self.y, &other.y) {
                if let Some(new_z) = Range3D::range_overlap(&self.z, &other.z) {
                    return Some(Range3D {
                        x: new_x,
                        y: new_y,
                        z: new_z,
                    });
                }
            }
        }
        None
    }
}

impl<T: std::str::FromStr + std::clone::Clone + std::fmt::Display + Num> Command<T> {
    fn from_line(line: &str) -> Option<Command<T>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(.*) x=(-?\d*)..(-?\d*),y=(-?\d*)..(-?\d*),z=(-?\d*)..(-?\d*)")
                    .unwrap();
        }

        if let Some(cap) = RE.captures_iter(line).next() {
            let mut iter = cap.iter().skip(1);
            let on = iter
                .next()
                .map_or(false, |c| c.map_or(false, |c1| c1.as_str() == "on"));
            let (x_range, y_range, z_range) = iter
                .filter_map(|c| c.and_then(|c1| c1.as_str().parse::<T>().ok()))
                .chunks(2)
                .into_iter()
                .map(|chunk| {
                    let (a, b) = chunk.collect_tuple().unwrap();
                    a..=b
                })
                .collect_tuple()
                .unwrap();
            Some(Command::<T> {
                range: Range3D::<T> {
                    x: x_range,
                    y: y_range,
                    z: z_range,
                },
                turn_on: on,
            })
        } else {
            println!("Skipping line {}", line);
            None
        }
    }
}

fn main() {
    let (filename, _sample_param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split('\n');
    let commands = input_lines
        .filter_map(Command::<i64>::from_line)
        .collect_vec();

    // part_one(&commands);
    part_two(&commands);
}

fn part_two(commands: &[Command<i64>]) {
    // Time to do some range math!
    let mut on_ranges: HashMap<Range3D<i64>, i64> = HashMap::new();
    for command in commands {
        println!("Processing command: {:?}", command);
        let mut new_ranges = on_ranges.clone();
        on_ranges.iter().for_each(|(r, count)| {
            if let Some(intersection) = command.range.intersect(r) {
                *new_ranges.entry(intersection).or_insert(0) -= count;
            }
        });
        if command.turn_on {
            *new_ranges.entry(command.range.clone()).or_insert(0) += 1;
        }
        on_ranges = new_ranges
    }
    let on: i64 = on_ranges
        .into_iter()
        .map(|(r, count)| {
            (r.x.end() - r.x.start() + 1)
                * (r.y.end() - r.y.start() + 1)
                * (r.z.end() - r.z.start() + 1)
                * count
        })
        .sum();
    println!("part two: {}", on);
}

fn _part_one(commands: &[Command<i64>]) {
    let mut on_pts = HashSet::new();
    let valid_range = -50..=50;
    for command in commands {
        for x in command.range.x.clone() {
            if !valid_range.contains(&x) {
                continue;
            }
            for y in command.range.y.clone() {
                if !valid_range.contains(&y) {
                    continue;
                }
                for z in command.range.z.clone() {
                    if !valid_range.contains(&z) {
                        continue;
                    }
                    let pt = (x, y, z);
                    if command.turn_on {
                        on_pts.insert(pt);
                    } else {
                        on_pts.remove(&pt);
                    }
                }
            }
        }
    }

    println!("On pts size: {}", on_pts.len());
}
