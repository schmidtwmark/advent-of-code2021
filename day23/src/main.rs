use itertools::Itertools;
// use std::time::Duration;
use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    env,
    fs,
    // thread::sleep,
};

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
    max_depth: usize,
    pods: Vec<Amphipod>,
}

impl Amphipods {
    fn print(&self) {
        println!("#############");
        let row0: String = (0..=10)
            .map(|i| {
                self.get_pod_at(i, 0).map_or('.', |p| {
                    if p.has_exited {
                        p.ident
                    } else {
                        p.ident.to_ascii_lowercase()
                    }
                })
            })
            .collect();
        println!("#{}#", row0);
        (1..=self.max_depth)
            .map(|depth| {
                (2..=8)
                    .step_by(2)
                    .map(|i| {
                        self.get_pod_at(i, depth).map_or('.', |p| {
                            if p.has_exited {
                                p.ident
                            } else {
                                p.ident.to_ascii_lowercase()
                            }
                        })
                    })
                    .interleave("###".chars())
                    .collect()
            })
            .for_each(|s: String| println!("###{}###", s));
        println!("#############");
    }

    fn get_pods(&self) -> Vec<(usize, &Amphipod)> {
        (0..=self.max_depth)
            .map(|depth| (0..=10).filter_map(move |pos| self.get_pod_and_index_at(pos, depth)))
            .flatten()
            .collect()
    }

    fn get_pod_and_index_at(&self, pos: usize, depth: usize) -> Option<(usize, &Amphipod)> {
        self.pods
            .iter()
            .enumerate()
            .find(|(_i, a)| a.position == pos && a.depth == depth)
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Move {
    target: usize,
    ident: char,
    new_position: usize,
    new_depth: usize,
    cost: usize,
}

impl Move {
    fn new(target: usize, ident: char, new_position: usize, new_depth: usize, cost: usize) -> Move {
        Move {
            target,
            ident,
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

    fn get_possible_moves(&self, idx: usize, all: &Amphipods) -> Vec<Move> {
        let mut out = vec![];

        if self.goal_position == self.position
            && (self.depth..=all.max_depth).all(|i| {
                all.get_pod_at(self.position, i)
                    .map_or(false, |p| p.ident == self.ident)
            })
        {
            return vec![]; // We are in place or stuck, no more moves to make
        }

        // Can we exit our tube?
        if (0..self.depth).all(|i| all.get_pod_at(self.position, i).is_none()) {
            // Can we get to our home?
            if self.path_open(&all.pods, self.position, self.goal_position)
                && (1..=all.max_depth).all(|i| {
                    all.get_pod_at(self.goal_position, i)
                        .map_or(true, |p| p.ident == self.ident)
                })
            {
                let target_depth = (1..=all.max_depth)
                    .filter(|i| all.get_pod_at(self.goal_position, *i).is_none())
                    .last()
                    .unwrap();

                assert!((self.goal_position != self.position) || (self.depth != target_depth));

                let cost_to_entry =
                    (abs_diff(&self.goal_position, &self.position) + self.depth + target_depth)
                        * self.movement_cost;

                out.push(Move::new(
                    idx,
                    self.ident,
                    self.goal_position,
                    target_depth,
                    cost_to_entry,
                ));
            }

            // If we haven't exited, can we get to any of the points on the top row?
            if !self.has_exited {
                // Add targets along top row
                let targets = [0, 1, 3, 5, 7, 9, 10]
                    .iter()
                    .filter(|pos| self.path_open(&all.pods, self.position, **pos));

                out.extend(targets.map(|pos| {
                    let dist = abs_diff(&self.position, pos) + self.depth;
                    Move::new(idx, self.ident, *pos, 0, dist * self.movement_cost)
                }));
            }
        }

        out
    }

    fn move_pod(&mut self, movement: &Move) {
        self.has_exited = true;
        self.position = movement.new_position;
        self.depth = movement.new_depth;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InvertedCost<T: Eq> {
    cost: usize,
    item: T,
    moves: VecDeque<Move>,
}

impl<T: Eq> InvertedCost<T> {
    fn new(cost: usize, item: T, moves: VecDeque<Move>) -> Self {
        Self { cost, item, moves }
    }
}

impl<T: Eq> PartialOrd for InvertedCost<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for InvertedCost<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn calculate(amphipods: Amphipods) -> (usize, VecDeque<Move>) {
    let mut candidates = BinaryHeap::with_capacity(128);
    let mut seen = HashMap::new();
    candidates.push(InvertedCost::new(0, amphipods, VecDeque::new()));

    while let Some(InvertedCost) = candidates.pop() {
        if InvertedCost
            .item
            .pods
            .iter()
            .all(|a| a.position == a.goal_position)
        {
            return (InvertedCost.cost, InvertedCost.moves);
        }

        println!(
            "Examining board with cost: {} and moves {:?}",
            InvertedCost.cost, InvertedCost.moves
        );
        InvertedCost.item.print();
        for (idx, pod) in InvertedCost.item.get_pods().iter() {
            let moves = pod.get_possible_moves(*idx, &InvertedCost.item);
            for movement in moves {
                println!("Examining move {:?}", movement);
                let mut new_pods = InvertedCost.item.clone();
                let cost = movement.cost;
                new_pods.pods[*idx].move_pod(&movement);
                let new_cost = InvertedCost.cost + cost;
                if let Some(existing_cost) = seen.get_mut(&new_pods) {
                    if new_cost < *existing_cost {
                        println!(
                            "Overwriting cost {} with new {} {:?}",
                            existing_cost, new_cost, new_pods
                        );
                        let mut new_moves = InvertedCost.moves.clone();
                        new_moves.push_back(movement);
                        *existing_cost = new_cost;
                        candidates.push(InvertedCost::new(new_cost, new_pods, new_moves));
                    }
                } else if seen.insert(new_pods.clone(), new_cost).is_none() {
                    println!("Inserting candidate for move {:?}", movement);
                    let mut new_moves = InvertedCost.moves.clone();
                    new_moves.push_back(movement);
                    candidates.push(InvertedCost::new(new_cost, new_pods, new_moves));
                }
            }
        }
    }
    unreachable!()
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
        .take(4)
        .filter_map(|line| {
            let it = line.chars().skip(3).step_by(2).take(4);
            if it.clone().any(|c| c.is_alphabetic()) {
                Some(it.collect_vec())
            } else {
                println!("Line {} not alpha", line);
                None
            }
        })
        .collect_vec();
    println!("{:?}, {}", rows, rows.len());
    let mut amphipods: Amphipods = Amphipods {
        max_depth: rows.len(),
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
    // println!("{:?}", amphipods);
    let (minimum, moves) = calculate(amphipods.clone());
    println!("{:?}", minimum);
    moves.iter().for_each(|m| {
        let mut new_pods = amphipods.clone();
        let target = new_pods.pods.get_mut(m.target).unwrap();

        println!(
            "\nMove {} from ({}, {}) to ({},{}) cost {}\nBefore:",
            m.ident, target.position, target.depth, m.new_position, m.new_depth, m.cost
        );
        target.move_pod(m);
        amphipods.print();
        println!("\nAfter:");
        new_pods.print();
        amphipods = new_pods;
    });

    println!("Sum: {:?}", moves.iter().map(|m| m.cost).sum::<usize>());
}
