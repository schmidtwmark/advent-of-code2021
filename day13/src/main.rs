use itertools::Itertools;
use std::{env, fs};
use std::collections::HashSet;

fn main() {
    let (filename, _sample_param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split('\n');
    let mut pts: HashSet<(usize, usize)> = input_lines.clone()
        .take_while(|line| *line != "")
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    // println!("{} pts, {:?}", pts.len(), pts);
    let folds : Vec<(bool, usize)> = input_lines.skip_while(|line| *line != "").skip(1).map(|line| {
        let (fold, num) = line.split('=').collect_tuple().unwrap();

        (fold.chars().nth(11).unwrap() == 'x', num.parse().unwrap()) 
    }).collect_vec();
    // println!("{:?}", folds);

    for (is_vertical, along_line) in folds {
        println!("Folding {} along {}", if is_vertical {"vertically"} else {"horizontally"}, along_line);
        let folded : HashSet<(usize, usize)> = pts.iter().map(|(x, y)| {
            if is_vertical && x >= &along_line {
                (2* along_line - x, *y)
            } else if !is_vertical && y >= &along_line {
                (*x, 2* along_line - y)
            } else {
                (*x, *y)
            }
        }).collect();
        pts = folded;
        // println!("Folded pts length = {}, pts = {:?}", pts.len(), pts);
    }

    let mut arr = [['.'; 50];50];

    for (x, y) in pts {
        arr[y][x] = '*';
    }
    for row in arr {
        println!("{:?}", row);
    } 

}
