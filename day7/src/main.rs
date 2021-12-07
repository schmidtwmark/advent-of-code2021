
use std::{cmp::min, env, fs};

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let mut input_lines = contents.split('\n');

    let mut nums: Vec<i32> = input_lines.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    // println!("nums: {:?}", nums);
    nums.sort_unstable();

    let count = nums.len();
    let mid = count / 2;
    let median = nums[mid];
    let float_mean: f64 = nums.iter().sum::<i32>() as f64 / count as f64;
    let (low_mean, high_mean) = (float_mean.floor() as i32, float_mean.ceil() as i32);
    println!("float_mean: {}, mean: {:?}, count: {}, mid: {}, median: {}", float_mean, (low_mean, high_mean), count, mid, median);

    let part_one : i32 = nums.iter().map(|num| { (num - median).abs() }).sum();
    println!("part_one {:?}", part_one);

    // let part_two : Vec<i32> = nums.iter().map(|num| { cost((num - mean).abs())}).collect();
    let part_two_low : i32 = nums.iter().map(|num| { cost((num - low_mean).abs())}).sum();
    let part_two_high : i32 = nums.iter().map(|num| { cost((num - high_mean).abs())}).sum();
    println!("part_two_low: {:?} part_two_high: {:?}", part_two_low, part_two_high);
    println!("part_two {}", min(part_two_low, part_two_high));
}

fn cost(steps: i32) -> i32 {
    (steps * (steps + 1)) / 2
}