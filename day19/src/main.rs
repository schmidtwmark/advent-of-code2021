use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

type Point = (i32, i32, i32);

// Took this from some other poor bastard, didn't want to think through all the different rotations
// https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/19.rs
fn rotate(point: Point, rot: i32) -> Point {
    let (x, y, z) = point;
    match rot {
        0 => (x, y, z),
        1 => (x, z, -y),
        2 => (x, -y, -z),
        3 => (x, -z, y),
        4 => (y, x, -z),
        5 => (y, z, x),
        6 => (y, -x, z),
        7 => (y, -z, -x),
        8 => (z, x, y),
        9 => (z, y, -x),
        10 => (z, -x, -y),
        11 => (z, -y, x),
        12 => (-x, y, -z),
        13 => (-x, z, y),
        14 => (-x, -y, z),
        15 => (-x, -z, -y),
        16 => (-y, x, z),
        17 => (-y, z, -x),
        18 => (-y, -x, -z),
        19 => (-y, -z, x),
        20 => (-z, x, -y),
        21 => (-z, y, x),
        22 => (-z, -x, y),
        23 => (-z, -y, -x),
        _ => unreachable!(),
    }
}

fn get_known_beacons(known: &mut HashSet<Point>, second: &[Point]) -> Option<Point> {
    for rotation in 0..24 {
        let rotated = second.iter().map(|pt| rotate(*pt, rotation));
        if let Some((dist, shifted)) = known
            .iter()
            .cartesian_product(rotated.clone())
            .map(|(a, b)| {
                let dist = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
                (
                    dist,
                    rotated
                        .clone()
                        .into_iter()
                        .map(move |pt| (pt.0 + dist.0, pt.1 + dist.1, pt.2 + dist.2)),
                )
            })
            .find(|(_dist, shifted)| shifted.clone().filter(|pt| known.contains(pt)).count() >= 12)
        {
            println!("Found points, extending!");
            known.extend(shifted.into_iter());
            return Some(dist);
        }
    }
    None
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

    let mut scanner_id = 0;
    let re = Regex::new(r"--- scanner (\d*) ---").unwrap();

    let scanners = input_lines.filter(|line| line != &"").group_by(|line| {
        if let Some(num) = re.captures_iter(line).next().and_then(|cap| {
            cap.iter()
                .nth(1)
                .and_then(|s| s.and_then(|s1| s1.as_str().parse::<i32>().ok()))
        }) {
            scanner_id = num;
        }
        scanner_id
    });

    let map: HashMap<i32, Vec<Point>> = scanners
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.skip(1)
                    .map(|pt| {
                        let (x, y, z) = pt
                            .split(',')
                            .filter_map(|num| num.parse::<i32>().ok())
                            .collect_tuple()
                            .unwrap();
                        (x, y, z)
                    })
                    .collect_vec(),
            )
        })
        .collect();

    let mut distances: HashMap<i32, Point> = HashMap::new(); // Distance from origin to point
    distances.insert(0, (0, 0, 0));

    let mut known_beacons: HashSet<Point> = HashSet::new();
    known_beacons.extend(map[&0].iter());

    while distances.len() < map.len() {
        for (scanner, beacons) in map.iter() {
            if !distances.contains_key(scanner) {
                if let Some(dist) = get_known_beacons(&mut known_beacons, beacons) {
                    *distances.entry(*scanner).or_default() = dist;
                }

            }
        }
    }

    println!("Scanner positions: {:?}", distances);
    println!("Known beacons: {:?}", known_beacons.len());

    let manhattan = distances
        .values()
        .cartesian_product(distances.values())
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
        .max()
        .unwrap();

    println!("Manhattan {:?} ", manhattan);
}
