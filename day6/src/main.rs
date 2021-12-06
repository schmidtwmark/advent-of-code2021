use std::fs;

const FILENAME : &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");

    let mut input_lines = contents.split("\n");
    let mut fish: Vec<usize> = input_lines.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let mut counts = [0; 9];
    for f in &fish {
        counts[*f] += 1;
    }
    println!("counts: {:?}", counts);
    for i in 0..256 {
        better_simulate(&mut counts); 
        println!("{}: total: {}, counts: {:?}", i + 1, counts.iter().sum::<usize>(), counts);
    }

    for _i in 0..80 {
        simulate(& mut fish);
    }
}
fn better_simulate(counts: &mut [usize]) {
    let zeroes = counts[0];
    for i in 1..counts.len() {
        counts[i - 1] = counts[i]; 
    }
    counts[6] += zeroes;
    counts[8] = zeroes;
}

fn simulate(fish: &mut Vec<usize>)  {
    let mut new_fish = 0;
    fish.iter_mut().for_each(|f| {
        *f = match f {
            0 => {new_fish += 1; 6},
            _ => *f - 1
        };
    });

    for _ in 0..new_fish {
        fish.push(8);
    };
}