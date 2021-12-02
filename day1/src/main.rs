
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");
    let numbers: Vec<i32> = lines.map(|s | s.parse().unwrap()).collect();
    let part_one_count = part_one(&numbers);
    let part_two_count = part_two(&numbers);

    println!("Increasing count is {}", part_one_count);
    println!("Part 2 count is {}", part_two_count);
    
}

fn part_one(numbers: &Vec<i32>) -> usize{
    numbers.windows(2).filter(|pair| pair[1] > pair[0]).count()
}
fn part_two(numbers: &Vec<i32>) -> usize{
    part_one(&numbers.windows(3).map(|slice| slice.iter().sum()).collect())

}