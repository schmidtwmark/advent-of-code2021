use itertools::Itertools;
use std::{collections::VecDeque, env, fs};

#[derive(Debug, Clone, PartialEq)]
enum SnailfishNumber {
    Regular(u32),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl SnailfishNumber {
    fn from_str(line: &str) -> SnailfishNumber {
        let mut chars = line.chars();
        if let Some(first) = chars.next() {
            let maybe_first_num = first.to_digit(10);
            match first {
                '[' => {
                    // Find split point comma
                    let mut depth = 1;
                    let mut comma_index = 0;
                    let mut idx = 1;
                    for c in chars {
                        idx += 1;
                        match c {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            ',' => {
                                if depth == 1 {
                                    comma_index = idx;
                                }
                            }
                            _ => (),
                        }
                    }
                    let (a, b) = line.split_at(comma_index);
                    SnailfishNumber::Pair(
                        Box::new(SnailfishNumber::from_str(&a[1..a.len() - 1])),
                        Box::new(SnailfishNumber::from_str(&b[..b.len() - 1])),
                    )
                }
                _ => {
                    if let Some(first_num) = maybe_first_num {
                        if let Some((_before, after)) = line.split_once(',') {
                            SnailfishNumber::Pair(
                                Box::new(SnailfishNumber::Regular(first_num)),
                                Box::new(SnailfishNumber::from_str(after)),
                            )
                        } else {
                            SnailfishNumber::Regular(first_num)
                        }
                    } else {
                        todo!()
                    }
                }
            }
        } else {
            unreachable!()
        }
    }

    fn flatten_num(&self, depth: u32) -> Vec<(u32, u32)> {
        let mut out = vec![];

        match self {
            SnailfishNumber::Regular(v) => out.push((*v, depth)),
            SnailfishNumber::Pair(a, b) => {
                out.append(&mut a.flatten_num(depth + 1));
                out.append(&mut b.flatten_num(depth + 1));
            }
        }

        out
    }
}

type NumberDepth = (u32, u32);
type FlattenedNum = Vec<NumberDepth>;

fn push_down(num: &mut [NumberDepth]) {
    num.iter_mut().for_each(|(_v, d)| *d += 1);
}

fn add(mut a: FlattenedNum, mut b: FlattenedNum) -> FlattenedNum {
    push_down(&mut a);
    push_down(&mut b);
    a.append(&mut b);
    reduce(&mut a);
    a
}

fn explode(num: &mut FlattenedNum) -> bool {
    for i in 0..num.len() {
        let (v, d) = num[i];
        if d != 4 {
            continue;
        }

        if i != 0 {
            num[i - 1].0 += v;
        }

        if i + 2 < num.len() {
            num[i + 2].0 += num[i + 1].0;
        }

        num[i] = (0, 3);
        num.remove(i + 1);

        return true;
    }

    false
}

fn split(num: &mut FlattenedNum) -> bool {
    for i in 0..num.len() {
        let (v, d) = num[i];
        if v < 10 {
            continue;
        }

        let div = v as f32 / 2.0;
        let (a, b) = (div.floor() as u32, div.ceil() as u32);

        num[i] = (a, d + 1);
        num.insert(i + 1, (b, d + 1));
        return true;
    }

    false
}

fn reduce(num: &mut FlattenedNum) {
    loop {
        if !explode(num) && !split(num) {
            break;
        }
    }
}
fn magnitude(num: &FlattenedNum) -> u32 {
    // let snailfish = SnailfishNumber::from_flat(num);
    let mut copy = num.clone();
    while copy.len() > 1 {
        for i in 0..copy.len() - 1 {
            let (v, d) = copy[i];
            let (v1, d1) = copy[i + 1];
            if d == d1 {
                let new_d = d.checked_sub(1).unwrap_or(0);
                copy[i] = (3 * v + 2 * v1, new_d);
                copy.remove(i + 1);
                break;
            }
        }
    }

    copy[0].0
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
    let inputs = input_lines
        .map(|line| {
            let mut out = vec![];
            let mut depth = 0;
            for c in line.chars() {
                match c {
                    '[' => {
                        depth += 1;
                    }
                    ',' => (),
                    ']' => {
                        depth -= 1;
                    }
                    d => {
                        out.push((d.to_digit(10).unwrap(), depth - 1));
                    }
                }
            }
            out
        })
        .collect_vec();
    println!("Inputs:\n{:?}\n\n", inputs);

    let output = inputs
        .clone()
        .into_iter()
        .fold(None, |acc: Option<FlattenedNum>, v| {
            if let Some(old) = acc {
                Some(add(old, v))
            } else {
                Some(v)
            }
        })
        .unwrap();
    println!("\n\nOutput:\n{:?}", output);
    println!("\n\nMagnitude:\n{:?}", magnitude(&output));

    let mut max = 0;
    inputs.clone().into_iter().for_each(|a| {
        inputs.clone().into_iter().for_each(|b| {
            let ab = magnitude(&add(a.clone(), b.clone()));
            if ab > max {
                max = ab;
            }
        });
    });
    println!("Max is {}", max);
}

