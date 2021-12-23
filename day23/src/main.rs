use itertools::Itertools;
use std::time::Duration;
use std::{collections::HashMap, env, fs, thread::sleep};

fn abs_diff(a: &usize, b: &usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Amphipods {
    // pods: [Amphipod; 8],
    pods: Vec<Amphipod>,
}

impl Amphipods {
    fn print(&self) {
        println!("#############");
        let row0: String = (0..=10)
            .map(|i| self.get_pod_at(i, 0).map_or('.', |p| p.ident))
            .collect();
        println!("#{}#", row0);
        let (row1, row2): (String, String) = (1..=2)
            .map(|depth| {
                (2..=8)
                    .step_by(2)
                    .map(|i| self.get_pod_at(i, depth).map_or('.', |p| p.ident))
                    .interleave("###".chars())
                    .collect()
            })
            .collect_tuple()
            .unwrap();
        println!("###{}###\n###{}###", row1, row2);
        println!("#############");
    }
    fn get_pod_at(&self, pos: usize, depth: usize) -> Option<&Amphipod> {
        self.pods
            .iter()
            .find(|a| a.position == pos && a.depth == depth)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Amphipod {
    ident: char,
    movement_cost: usize,
    position: usize,
    depth: usize,
    goal_position: usize,
    has_exited: bool,
}

#[derive(Debug)]
struct Move {
    new_position: usize,
    new_depth: usize,
    cost: usize,
}

impl Move {
    fn new(new_position: usize, new_depth: usize, cost: usize) -> Move {
        Move {
            new_position,
            new_depth,
            cost,
        }
    }
}

impl Amphipod {
    fn new(ident: char, depth: usize, position: usize) -> Amphipod {
        let idx = ident as u32 - 'A' as u32;
        Amphipod {
            ident,
            movement_cost: 10_usize.pow(idx),
            position,
            depth,
            goal_position: (idx * 2 + 2) as usize,
            has_exited: false,
        }
    }
    fn path_open(&self, all: &[Amphipod], start: usize, end: usize) -> bool {
        let left = std::cmp::min(start, end);
        let right = std::cmp::max(start, end);
        let r = left..=right;
        let filtered = all
            .iter()
            .filter(|a| *a != self && r.contains(&a.position) && a.depth == 0);
        // println!(
        //     "Filtered({}): start, end ({},{}), {:?}",
        //     self.ident,
        //     start,
        //     end,
        //     filtered.clone().collect_vec()
        // );
        filtered.count() == 0
    }

    fn get_possible_moves(&self, _idx: usize, all: &Amphipods) -> Vec<Move> {
        let mut out = vec![];
        if self.goal_position == self.position {
            if self.depth == 2 {
                return out;
            } else if self.depth == 1 {
                if let Some(pod) = all.get_pod_at(self.goal_position, 2) {
                    if pod.ident == self.ident {
                        return out;
                    }
                }
            }
        }
        if self.has_exited {
            if self.goal_position == self.position {
                return out; // If we've exited, and we're at home, there's nothing else to do
            }
            // Can we get to our home?
            if self.path_open(&all.pods, self.position, self.goal_position)
                && all.get_pod_at(self.goal_position, 1).is_none()
            {
                let cost_to_entry =
                    abs_diff(&self.goal_position, &self.position) * self.movement_cost;
                if all.get_pod_at(self.goal_position, 2).is_none() {
                    out.push(Move::new(
                        self.goal_position,
                        2,
                        cost_to_entry + (self.movement_cost * 2),
                    ));
                } else {
                    out.push(Move::new(
                        self.goal_position,
                        1,
                        cost_to_entry + self.movement_cost,
                    ));
                }
            }
        } else {
            let targets = [0, 1, 3, 5, 7, 9, 10]
                .iter()
                .filter(|pos| self.path_open(&all.pods, self.position, **pos));
            if self.depth == 2 && all.get_pod_at(self.position, 1).is_none() {
                // At pos 2, depth is clear
                out.extend(targets.map(|pos| {
                    let dist = abs_diff(&self.position, pos);
                    Move::new(*pos, 0, (dist + 2) * self.movement_cost)
                }));
            } else if self.depth == 1 {
                out.extend(targets.map(|pos| {
                    let dist = abs_diff(&self.position, pos);
                    Move::new(*pos, 0, (dist + 1) * self.movement_cost)
                }));
            }
        }
        out
    }

    fn move_pod(&mut self, movement: Move) {
        self.has_exited = true;
        self.position = movement.new_position;
        self.depth = movement.new_depth;
    }
}

fn calculate(amphipods: Amphipods, cache: &mut HashMap<Amphipods, usize>) -> usize {
    let mut minimum = usize::MAX;
    if let Some(cost) = cache.get(&amphipods) {
        println!("Returning from cache");
        return *cost;
    }
    amphipods.print();
    if amphipods.pods.iter().all(|a| a.position == a.goal_position) {
        println!("Donezo");
        return 0;
    }
    for (idx, pod) in amphipods.pods.iter().enumerate() {
        let moves = pod.get_possible_moves(idx, &amphipods);
        // if moves.len() > 0 {
        //     println!("{}: {} moves: {:?}", pod.ident, moves.len(), moves);
        // }
        for movement in moves {
            let mut new_pods = amphipods.clone();
            let cost = movement.cost;
            new_pods.pods[idx].move_pod(movement);
            let total = cost
                .checked_add(calculate(new_pods.clone(), cache))
                .unwrap_or(usize::MAX);
            *cache.entry(new_pods).or_default() = total;
            minimum = std::cmp::min(total, minimum);
        }
    }
    // panic!();
    // sleep(Duration::from_secs(1));
    minimum
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
    let rows = input_lines
        .skip(2)
        .take(2)
        .map(|line| line.chars().skip(3).step_by(2).take(4).collect_vec())
        .collect_vec();
    println!("{:?}", rows);
    let amphipods: Amphipods = Amphipods {
        pods: rows
            .iter()
            .enumerate()
            .map(|(depth, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(pos, c)| Amphipod::new(*c, depth + 1, 2 + 2 * pos))
            })
            .flatten()
            .collect_vec(),
    };
    println!("{:?}", amphipods);
    println!("{}", calculate(amphipods, &mut HashMap::new()));
}
