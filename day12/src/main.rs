
use std::{env, fs};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
type Network<'a> = HashMap<&'a str, Cave<'a>>;

#[derive(Debug)]
struct Cave<'a> {
    neighbors: HashSet<&'a str>,
    small: bool
}

impl Cave<'_> {
    fn new<'a>(small: bool) -> Cave<'a> {
        Cave {
            neighbors:  HashSet::new(),
            small
        }
    }
}

// Return unique path count
fn bfs<'a>(network: &Network, start: &'a str, end: &'a str) -> Option<usize> {
    let mut queue: VecDeque<(&str, Vec<&str>)> = VecDeque::new();
    let mut paths: Vec<Vec<&str>> = vec![];
    queue.push_back((start, vec![start]));
    while !queue.is_empty()  {
        let (node, current_path) = queue.pop_front()?;
        let cave = network.get(node)?;
        // println!("Visiting cave for path: {} {:?}", node, current_path);
        for neighbor in &cave.neighbors {
            let neighbor_cave = network.get(neighbor)?;
            if !neighbor_cave.small || !current_path.contains(neighbor) {
                let mut new_path = current_path.clone();
                new_path.push(neighbor);
                if neighbor == &end {
                    paths.push(new_path);
                } else {
                    queue.push_back((neighbor, new_path));
                }
            }
        }
    }
    Some(paths.len())
}

// Return unique path count
fn bfs2<'a>(network: &Network, start: &'a str, end: &'a str) -> Option<usize> {
    let mut queue: VecDeque<(&str, Vec<&str>, bool)> = VecDeque::new();
    let mut paths: Vec<Vec<&str>> = vec![];
    queue.push_back((start, vec![start], false));
    while !queue.is_empty()  {
        let (node, current_path, reuse_small) = queue.pop_front()?;
        let cave = network.get(node)?;
        // println!("Visiting cave for path: {} {} {:?}", node, reuse_small, current_path);
        for neighbor in &cave.neighbors {
            let neighbor_cave = network.get(neighbor)?;
            let mut new_path = current_path.clone();
            new_path.push(neighbor);
            if neighbor == &end {
                paths.push(new_path);
            } else if neighbor != &start {
                if neighbor_cave.small { 
                    if !current_path.contains(neighbor) {
                        queue.push_back((neighbor, new_path, reuse_small));
                    } else if !reuse_small {
                        queue.push_back((neighbor, new_path, true));
                    }
                } else {
                    queue.push_back((neighbor, new_path, reuse_small));
                }
            }
        }
    }
    Some(paths.len())
}


fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let paths : Vec<(&str, &str)> = contents.split('\n').map(|line| line.split('-').collect_tuple().unwrap()).collect_vec();

    let mut network : Network = HashMap::new();
    for path in paths {
        let (begin, end) = path;
        if !network.contains_key(begin)  {
            network.insert(begin, Cave::new(!begin.chars().all(char::is_uppercase)));
        }
        if !network.contains_key(end)  {
            network.insert(end, Cave::new(!end.chars().all(char::is_uppercase)));
        }

        network.get_mut(begin).unwrap().neighbors.insert(end);
        network.get_mut(end).unwrap().neighbors.insert(begin);
    }

    println!("Network {:?}", network);
    println!("Part one: {}", bfs(&network, "start", "end").unwrap());
    println!("Part one: {}", bfs2(&network, "start", "end").unwrap());

}