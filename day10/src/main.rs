
use std::{env, fs};
use itertools::Itertools;

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    // let mut mappings = HashMap::new();
    // mappings.insert('{',('}', 1197));
    // mappings.insert('<',('>', 25137));
    // mappings.insert('(',(')', 3));
    // mappings.insert('[',(']', 57));

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split('\n').map(|line| line.chars().collect_vec()).collect_vec();
    println!("part_one {}", part_one(&input_lines));
    println!("part_two {}", part_two(&input_lines));
}

fn _is_opening(candidate: &char) -> bool {
    matches!(candidate, '{'| '('| '<' |'[')
}
fn get_score(closing: &char) -> i32 {
    match closing{
        '}' => 1197,
        ')' => 3, 
        '>' => 25137,
        ']' => 57,
        _ => 0
    }
}
fn get_score2(closing: &char) -> usize {
    match closing{
        '}' => 3,
        ')' => 1, 
        '>' => 4,
        ']' => 2,
        _ => 0
    }
}

fn get_closing(opening: &char) -> Option<char> {
    match opening {
        '{' => Some('}'),
        '(' => Some(')'),
        '<' => Some('>'),
        '[' => Some(']'),
        _ => None 
    }
}

fn part_one(lines: &[Vec<char>]) -> i32 {
    lines.iter().map(|line| {
        // println!("Looking at line {:?}", line);
        let mut stack: Vec<char> = vec![];
        for char in line {
            if let Some(closing) = get_closing(char) {
                stack.push(closing);
            } else if let Some(last) = stack.pop() {
                if last != *char {
                    // println!("Got error with char {}", char);
                    return get_score(char);
                }
            } else {
                println!("Popping empty list?");
            }
        }
        0 
    }).sum()
}
fn part_two(lines: &[Vec<char>]) -> usize {
    let filtered = lines.iter().map(|line| {
        // println!("Looking at line {:?}", line);
        let mut stack: Vec<char> = vec![];
        for char in line {
            if let Some(closing) = get_closing(char) {
                stack.push(closing);
            } else if let Some(last) = stack.pop() {
                if last != *char {
                    return None;
                }
            } else {
                println!("Popping empty list?");
            }
        }
        Some(stack.iter().rev().fold(0, |acc, closing_character| {
            acc * 5 + get_score2(closing_character)
        }))
    }).flatten().sorted().collect_vec();
    println!("filtered: {:?}", filtered);
    *filtered.get(filtered.len() / 2).unwrap()
}