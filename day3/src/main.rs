use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");
    let (g1, e1) = part_two(lines);
    println!("Output is ({:b}, {:b}), ({}, {}), mult is {}", g1, e1, g1, e1, g1*e1);
}

fn fold_bit_counts(bit_counts: &Vec<(i32,i32)>, gamma: bool) -> i32{
    bit_counts.iter().fold(0, |mut acc, (zeroes, ones)| { 
        acc = acc << 1;
        if zeroes > ones {
            acc | (if gamma {0} else {1})
        } else {
            acc | (if gamma {1} else {0})
        }
    })
}
fn get_bit_counts<'a, I>(lines: I) -> Vec<(i32, i32)> where I: Iterator<Item = &'a str> {
    lines.fold(vec![(0,0); 12], |mut acc, val| {
        val.chars().enumerate().for_each(|(i, c)| {
            match c {
                '0' => acc[i].0 += 1,
                '1' => acc[i].1 += 1,
                _ => panic!()
            }
        });
        acc
    })
}
fn part_one<'a, I>(lines: I) -> (i32, i32) where I: Iterator<Item = &'a str> {
    let bit_counts = get_bit_counts(lines);
    println!("bit counts: {:?}", bit_counts);
    let gamma : i32 = fold_bit_counts(&bit_counts, true);
    let epsilon: i32 = fold_bit_counts(&bit_counts, false);
    (gamma, epsilon)
}


fn filter<'a>(index: usize, lines: Vec<&'a str>, greater: bool) -> Vec<&'a str> {
    let bit_counts = get_bit_counts(lines.clone().into_iter());
    let vals = lines.into_iter().filter(|s| {
        let (zeros, ones) = bit_counts[index];
        let char = s.chars().nth(index).unwrap();
        if greater {
            zeros > ones && char == '0' || ones >= zeros && char == '1'
        } else {
            ones >= zeros && char == '0' || zeros > ones && char == '1'
        }
    }).collect();
    vals
}

fn part_two<'a, I>(lines: I) -> (i32, i32) where I: Iterator<Item = &'a str> {
    let mut greater : Vec<&str>= lines.collect();
    let mut lesser = greater.clone();
    for x in 0..12 {
        if greater.len() > 1 {
            greater = filter(x, greater, true);
        }
        if lesser.len() > 1 {
            lesser = filter(x, lesser, false);
        }
    }
    (i32::from_str_radix(greater[0], 2).unwrap(), i32::from_str_radix(lesser[0], 2).unwrap())

}