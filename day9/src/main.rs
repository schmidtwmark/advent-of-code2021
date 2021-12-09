
use std::{env, fs};
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split('\n');
    let input = input_lines.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect_vec();
 
    // println!("{:?}", input);
    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

fn score(input: &[Vec<u32>], row_index: usize, column_index: usize, value: &u32) -> u32{
    let pts = [(row_index.checked_sub(1), Some(column_index)), (Some(row_index+1), Some(column_index)), (Some(row_index), Some(column_index+1)), (Some(row_index), column_index.checked_sub(1))];
    if pts.iter().all(|(x, y)| x.map_or(true, |x| y.map_or(true, |y| input.get(x).map_or(true, |row| row.get(y).map_or(true, |candidate| value < candidate))))) {
        value + 1
    } else {
        0
    }
}

fn part_one(input: &[Vec<u32>]) -> u32 {
    input.iter().enumerate().fold(0, |acc: u32, (row_index, row)| {
        acc + row.iter().enumerate().map(|(column_index, value)| {
            score(input, row_index, column_index, value) 
        }).sum::<u32>()
    })
}

fn get_neighbor_indices(x: &usize, y: &usize) -> HashSet<(usize, usize)> {
    let mut out = HashSet::new();
    let x_plus= x + 1;
    let y_plus  = y + 1;
    let x_minus = x.checked_sub(1);
    let y_minus = y.checked_sub(1);
    out.insert((x_plus, *y));
    out.insert((*x, y_plus));
    if let Some(x_)= x_minus {
        out.insert((x_, *y));
    }
    if let Some(y_)= y_minus {
        out.insert((*x, y_));
    }
    // println!("Neighbors to ({},{}), {:?}", x, y, out);
    out
}

fn part_two(input: &[Vec<u32>]) -> usize {
    let low_points = input.iter().enumerate().filter_map(|(row_index, row)| {
        let filtered_row = row.iter().enumerate().filter_map(|(column_index, value)| {
            if score(input, row_index, column_index, value) != 0 {
                Some((row_index, column_index, value))
            } else {
                None
            }
        }).collect_vec();
        if !filtered_row.is_empty() {
            Some(filtered_row)
        } else {
            None
        }
    }).flatten().collect_vec();

    // println!("low_points {:?}", low_points);
    let regions = low_points.iter().map(|(x, y, _value)| {
        let mut pts = HashSet::new();
        // println!("Examining pt {},{} with value {}", x, y, value);
        pts.insert((*x, *y));
        let mut last_size = 0;
        while last_size < pts.len() {
            last_size = pts.len();
            let neighbors = pts.iter().fold(HashSet::new(), |acc, (x, y)| {
                let pt_neighbors = get_neighbor_indices(x, y).into_iter().filter(|(x, y)| {
                    input.get(*x).map_or(false, |row| 
                        row.get(*y).map_or(false, |candidate|  {
                            // println!("candidate: {:?}", candidate);
                            *candidate != 9
                        }
                        )
                    )
                }).collect();
                // println!("valid neighbors: {:?}", pt_neighbors);
                acc.union(&pt_neighbors).copied().collect()
            });
            pts = pts.union(&neighbors).copied().collect();

            // println!("pts_len: {}, pts: {:?}", pts.len(), pts);
        }
        pts
    }).collect_vec();
    // println!("regions {:?}", regions);
    regions.iter().map(|set| set.len()).sorted().rev().take(3).product()


}
