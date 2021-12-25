use itertools::Itertools;
use std::{collections::HashMap, env, fs, vec};

type Point = (usize, usize);
#[derive(Debug)]
enum Cucumber {
    Down(),
    Right(),
}

impl Cucumber {}

struct Cucumbers {
    width: usize,
    height: usize,
    cucumbers: HashMap<Point, Cucumber>,
}

impl Cucumbers {
    fn print(&self) {
        let out = (0..self.height).map(|x| {
            (0..self.width)
                .map(|y| {
                    if let Some(cucumber) = self.cucumbers.get(&(x, y)) {
                        match cucumber {
                            Cucumber::Down() => 'v',
                            Cucumber::Right() => '>',
                        }
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        });

        out.for_each(|o| println!("{:?}", o));
    }

    fn get_target(&self, pos: &Point, cucumber: &Cucumber) -> Option<Point> {
        let target_point = match cucumber {
            // Cucumber::Down() => (pos.0, (pos.1 + 1) % self.height),
            // Cucumber::Right() => ((pos.0 + 1) % self.width, pos.1),
            Cucumber::Down() => ((pos.0 + 1) % self.height, pos.1),
            Cucumber::Right() => (pos.0, (pos.1 + 1) % self.width),
        };
        // println!("For {:?}, target is {:?}", pos, target_point);
        if self.cucumbers.get(&target_point).is_none() {
            Some(target_point)
        } else {
            None
        }
    }
    fn step(&mut self) -> usize {
        let right = self
            .cucumbers
            .iter()
            .filter(|(_k, v)| matches!(v, Cucumber::Right()));

        let right_movements = right
            .filter_map(|(k, v)| self.get_target(k, v).map(|target| (*k, target)))
            .collect_vec();
        right_movements.iter().for_each(|(start, end)| {
            let cucumber = self.cucumbers.remove(&start).unwrap();
            self.cucumbers.insert(*end, cucumber);
        });
        let down = self
            .cucumbers
            .iter()
            .filter(|(_k, v)| matches!(v, Cucumber::Down()));
        let down_movements = down
            .filter_map(|(k, v)| self.get_target(k, v).map(|target| (*k, target)))
            .collect_vec();
        down_movements.iter().for_each(|(start, end)| {
            let cucumber = self.cucumbers.remove(&start).unwrap();
            self.cucumbers.insert(*end, cucumber);
        });

        // println!("{:?}", movements.len());
        // self.print();

        down_movements.len() + right_movements.len()
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

    let width = input_lines.clone().next().unwrap().len();
    let height = input_lines.clone().count();

    let mut cucumbers = Cucumbers {
        width,
        height,
        cucumbers: input_lines
            .enumerate()
            .fold(HashMap::new(), |mut acc, (row, line)| {
                println!("Line: {}", line);
                acc.extend(line.chars().enumerate().filter_map(|(col, c)| match c {
                    'v' => Some(((row, col), Cucumber::Down())),
                    '>' => Some(((row, col), Cucumber::Right())),
                    '.' => None,
                    a => {
                        println!("Extra char {}", a);
                        None
                    }
                }));
                acc
            }),
    };

    cucumbers.print();
    let mut i = 0;
    while {
        // println!("Step {}", i);
        let moved = cucumbers.step();
        // println!();
        i += 1;
        // if i > 100 {
        //     panic!();
        // }
        moved > 0
    } {}
    
    println!("Finished after step {}", i);
}
