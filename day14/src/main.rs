
use std::{env, fs};
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp;

type Pair = (char, char);

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let mut input_lines = contents.split('\n');
    let mut polymer = input_lines.next().unwrap().chars().collect_vec();
    let pair_insertions : HashMap<Pair, char> = input_lines.skip(1).filter_map(|line| {
        let splat :Vec<&str> = line.split(" -> ").collect();
        let pair: Pair = splat.get(0)?.chars().collect_tuple()?;
        let target = splat.get(1)?.chars().next()?;
        Some((pair, target))
    }).collect();

    println!("Pair insertions: {:?}", pair_insertions);


    println!("Polymer: {:?}", polymer);
    let mut pair_frequencies = polymer.windows(2).filter_map(|slice| {
        let (a, b) = slice.iter().collect_tuple()?;
        Some((*a,*b))
    }).fold(HashMap::<Pair, usize>::new(), |mut m, pair| {
        *m.entry(pair).or_default() += 1;
        m
    });
    for i in 1..=10 {
        pair_frequencies = better_step(pair_frequencies, &pair_insertions);
        println!("After step {}, freqs are:\n{:?}\n\n", i, pair_frequencies);
    }

    let (begin_freqs, end_freqs) = pair_frequencies.iter()
        .fold((HashMap::<&char, usize>::new(), HashMap::<&char, usize>::new()),
         |(mut begin, mut end), ((a,b), count)| {  
            *begin.entry(a).or_default() += count;
            *end.entry(b).or_default() += count;
            (begin, end)
    });
    println!("begin_freqs:\n{:?}\nend_freqs:\n{:?}", begin_freqs, end_freqs);
    let mut total_freqs = begin_freqs.keys().fold(HashMap::<&char, usize>::new(), |mut m, k| {
        *m.entry(k).or_default() = match (begin_freqs.get(k), end_freqs.get(k)) {
            (Some(begin), Some(end)) => cmp::max(*begin, *end),
            _ => panic!("Key not present in begin or end")
        };
        m
    });

    // This is really fucking annoying
    if let (Some(first), Some(last)) = (polymer.first(), polymer.last()) {
        if first == last {
            *total_freqs.entry(first).or_default() += 1;
        }
    }
    println!("Total freqs: {:?}", total_freqs);


    let total_max = total_freqs.iter().max_by_key(|(_, v)| *v).map_or(0, |(_, v)| *v);
    let total_min = total_freqs.iter().min_by_key(|(_, v)| *v).map_or(0, |(_, v)| *v);

    println!("Part 2 Max - min: {:?}", total_max - total_min);

    for _i in 1..=10 {
        polymer = step(polymer, &pair_insertions);
        // println!("After step {}, polymer size is: {:?}", i, polymer.len());
    }
    let freqs = polymer.iter().fold(HashMap::<&char, usize>::new(), |mut m, val| { 
        *m.entry(val).or_default() += 1;
        m
    });

    println!("Frequencies: {:?}", freqs);
    let max = freqs.iter().max_by_key(|(_, v)| *v).map_or(0, |(_, v)| *v);
    let min = freqs.iter().min_by_key(|(_, v)| *v).map_or(0, |(_, v)| *v);

    println!("Part 1 Max - min: {:?}", max - min);


}
fn better_step(pair_frequencies: HashMap<Pair, usize>, insertions: &HashMap<Pair, char>) -> HashMap<Pair, usize>{
    let mut out = HashMap::new();

    for (pair, count) in pair_frequencies {
        let (a, b) = pair;
        if let Some(insertion) = insertions.get(&pair) {
            *out.entry((a, *insertion)).or_default() += count;
            *out.entry((*insertion, b)).or_default() += count;
        } else {
            *out.entry(pair).or_default() += count;
        }
    }

    out
}

fn step(polymer: Vec<char>, insertions: &HashMap<Pair, char>) -> Vec<char>{
    let mut out = polymer.windows(2).filter_map(|slice| {
        let (a, b) = slice.iter().collect_tuple()?;
        if let Some(insertion) = insertions.get(&(*a, *b)) {
            Some(vec![*a, *insertion])
        } else {
            Some(vec![*a])
        }
    }).flatten().collect_vec();
    if let Some(last) = polymer.last() {
        out.push(*last);
    }
    out
}