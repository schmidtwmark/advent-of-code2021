
use std::{env, fs};
use itertools::Itertools;
use std::collections::HashSet;

type Point = (usize, usize);

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let mut input = contents.split('\n').map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect_vec();
    println!("Before steps");
    display(&input);

    let mut total_flashed = 0;
    let mut i = 1;

    while {
        println!("\n\nProcessing step {}", i);
        let (new_input, flashed) = step(input);
        input = new_input;
        total_flashed += flashed;
        println!("\n\nafter step {}: {} flashed, {} total_flashed", i, flashed, total_flashed);
        i+= 1;
        display(&input);
        flashed != 100
    } {}
}

fn display(vals: &Vec<Vec<u32>>) {
    vals.iter().for_each(|line| {
        println!("{:?}", line);
    });
}

fn get_neighbor_indices(pt: &Point) -> HashSet<Point>{
    let mut out = HashSet::new();
    let (x, y) = pt;
    let x_plus= x + 1;
    let y_plus  = y + 1;
    let x_minus = x.checked_sub(1);
    let y_minus = y.checked_sub(1);
    out.insert((x_plus, *y));
    out.insert((*x, y_plus));
    out.insert((x_plus, y_plus));
    if let Some(x_)= x_minus {
        out.insert((x_, *y));
        out.insert((x_, y_plus));
    }
    if let Some(y_)= y_minus {
        out.insert((*x, y_));
        out.insert((x_plus, y_));
        if let Some(x_)= x_minus {
            out.insert((x_, y_));
        }
    }
    // println!("Neighbors to ({},{}), {:?}", x, y, out);
    out
}

fn step(input: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, usize) {
    let mut incremented = input.iter().map(|row| {
        row.iter().map(|value| {
            value + 1        
        }).collect_vec()
    }).collect_vec();

    // display(&incremented);

    let mut flashed = HashSet::new();
    let mut prev_count;
    while {
        prev_count = flashed.len();
        let all_flashed :HashSet<Point> = incremented.iter().enumerate().map(|(x, row)| {
            row.iter().enumerate().filter_map(|(y, value)| {
                if value > &9 {
                    Some((x, y))
                } else {
                    None
                }
            }).collect_vec()
        }).flatten().collect();

        let new_flashed= all_flashed.difference(&flashed);
        // println!("new flashed count: {}, new_flashed{:?}", new_flashed.clone().count(), new_flashed);

        new_flashed.for_each(|pt| {
            get_neighbor_indices(pt).iter().for_each(|(x,y)| {
                incremented.get_mut(*x).map(|row| row.get_mut(*y).map(|value| { 
                    // println!("Incrementing neighbor ({},{}), currently {}",*x, *y, value); 
                    *value+=1
                }));
            });
        });

        flashed = all_flashed;
        // println!("all_flashed count {:?}, prev_count {}", flashed.len(), prev_count);
        // display(&incremented);
        flashed.len() > prev_count 
    } {}

    incremented.iter_mut().for_each(|row| row.iter_mut().for_each(|value| if *value > 9 { *value = 0 }));


    (incremented, flashed.len())
}
