use itertools::Itertools;
use regex::Regex;
use std::{env, fs, ops::{AddAssign, RangeInclusive}, collections::HashSet};
#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector {
    x: i32,
    y: i32
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Vector {
    fn new(x: i32, y: i32) -> Vector{
        Vector {
            x,
            y
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct State{
    position: Vector,
    velocity: Vector,
    max_height: i32
}

impl State {
    fn new(position: (i32, i32), velocity: (i32, i32)) -> State{
        State {
            position: Vector::new(position.0, position.1),
            velocity: Vector::new(velocity.0, velocity.1),
            max_height: i32::MIN
        }
    }
}

fn step(state: &mut State) {
    state.position += state.velocity;
    let dx = state.velocity.x.signum();
    state.velocity.x -= dx;
    state.velocity.y -= 1;

    // keep track of max height
    state.max_height = std::cmp::max(state.max_height, state.position.y);
}

fn progress(initial_velocity: (i32, i32), x_range: &RangeInclusive<i32>, y_range: &RangeInclusive<i32>) -> Option<State>{
    let mut current= State::new((0,0), initial_velocity);

    while current.position.x.abs() <= x_range.end().abs() && &current.position.y >= y_range.start() {
        step(&mut current);
        // println!("Current: {:?}", current);
        if x_range.contains(&current.position.x) && y_range.contains(&current.position.y) {
            // println!("State in range!");
            return Some(current);
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
    let re = Regex::new(r"target area: x=(-?\d*)..(-?\d*), y=(-?\d*)..(-?\d*)").unwrap();
    let mut caps = re.captures_iter(&contents);
    let (x1, x2, y1, y2) = caps
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.unwrap().as_str().parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    let x_range = x1..=x2;
    let y_range = y1..=y2;

    println!("X range {:?}\nY range {:?}", x_range, y_range);

    println!("Part one {}", part_one(x_range.clone(), y_range.clone()))    ;
    println!("Part two {}", part_two(x_range.clone(), y_range.clone()))    ;
}

fn part_two(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> usize {
    let mut vels = HashSet::new();
    for x_vel in -500..500{
        for y_vel in -500..500 {
            let initial_velocity = (x_vel, y_vel);
            if let Some(_state)= progress(initial_velocity, &x_range, &y_range) {
                vels.insert(initial_velocity);
            }
        }
    }
    println!("Vels: {:?}", vels);
    vels.len()

}

fn part_one(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> i32 {
    let inital_x_vels = x_range.clone().filter_map(|target_x| {
        for i in 1..target_x {
            let sum = (i * i + 1) / 2;
            match sum {
                _ if sum == target_x => return Some(i - 1),
                _ if sum > target_x => break,
                _ => ()
            };
        }
        None
    }).collect_vec();
    let mut max_height = i32::MIN;
    println!("initial_x_vels: {:?}", inital_x_vels);

    for x_vel in inital_x_vels {
        for y_vel in 0..500 {
            let initial_velocity = (x_vel, y_vel);
            if let Some(state)= progress(initial_velocity, &x_range, &y_range) {
                    max_height = std::cmp::max(max_height, state.max_height) 
            }
        }
    }
    max_height
}
