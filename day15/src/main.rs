
use std::{env, fs};
use itertools::Itertools;
use std::collections::{VecDeque, HashSet};

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let risk_levels = contents.split('\n').map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect_vec()).collect_vec();
    let width = risk_levels.len();
    let mut bigger_risks = vec![vec![0; width * 5]; width * 5];

    risk_levels.iter().enumerate().for_each(|(x, row)| row.iter().enumerate().for_each(|(y, val)| {
        for big_x in 0u32..5 {
            for big_y in 0u32..5 {
                let mut value = *val + big_x + big_y;
                if value > 9 {
                    value %= 9;
                }
                bigger_risks[width * big_x as usize + x][width * big_y as usize + y] = value;
            }
        }
    }));

    bigger_risks.iter().for_each(|row| println!("{:?}", row));
    bigger_risks.iter().skip(2).step_by(width).for_each(|row| println!("{:?}", row.iter().skip(3).step_by(width).collect_vec()));
    println!("Part One: {:?}", part_one(&risk_levels));
    //3063 is too high
    println!("Part Two: {:?}", part_one(&bigger_risks));


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


fn part_one(risk_levels: &[Vec<u32>]) -> Option<u32>{
    let mut risk_and_costs = risk_levels.iter().map(|row| row.iter().map(|val| (*val, std::u32::MAX)).collect_vec()).collect_vec();
    let mut frontier = VecDeque::new();
    frontier.push_back((0,0));
    risk_and_costs[0][0].1 = 0;
    while !frontier.is_empty()  {
        if let Some((x, y)) = frontier.pop_front() {
            let (_risk, cost) = risk_and_costs[x][y];
            let neighbors = get_neighbor_indices(&x, &y);
            neighbors.iter().for_each(|(neighbor_x, neighbor_y)| {
                if let Some((neighbor_risk, neighbor_cost)) = risk_and_costs.get_mut(*neighbor_x).and_then(|row| row.get_mut(*neighbor_y)) {
                    let candidate_cost = cost + *neighbor_risk;
                    if candidate_cost < *neighbor_cost {
                        *neighbor_cost = candidate_cost;
                        frontier.push_back((*neighbor_x, *neighbor_y));
                    }
                }
            })
        }
    }

    // risk_and_costs.iter().for_each(|row| println!("{:?}", row));

    risk_and_costs.last().and_then(|a|a.last().map(|v| v.1))
}