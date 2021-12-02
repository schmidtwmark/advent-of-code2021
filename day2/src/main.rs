use std::{fs};

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");
    let (x, y) = get_aimed_coordinate(lines);
    let mult = x * y;
    println!("Coordinate is ({},{}) multiplied is {}", x, y, mult)
}

enum Command {
    Aim(i32),
    Forward(i32)
}

fn get_aimed_coordinate<'a, I>(lines: I) -> (i32, i32) where I: Iterator<Item = &'a str>{
    let (aim, x, y) = lines.map(|s| {
        let mut splat = s.split(" "); 
        let direction = splat.next().unwrap();
        let magnitude: i32 = splat.next().unwrap().parse().unwrap();
        match &direction[..1] {
            "f" => Command::Forward(magnitude),
            "d" => Command::Aim(magnitude),
            "u" => Command::Aim(-magnitude),
            _ => panic!()
        }
    }).fold((0, 0, 0), |acc, val| match val {
        Command::Aim(delta) => (acc.0 + delta, acc.1, acc.2),
        Command::Forward(delta) => (acc.0, acc.1 + delta, acc.2 + delta * acc.0)
    });
    (x, y)

}


fn get_coordinate<'a, I>(lines: I) -> (i32, i32) where I: Iterator<Item = &'a str>{
    lines.map(|s| {
        let mut splat = s.split(" "); 
        let direction = splat.next().unwrap();
        let magnitude: i32 = splat.next().unwrap().parse().unwrap();
        match &direction[..1] {
            "f" => (magnitude,0),
            "d" => (0, magnitude),
            "u" => (0, -magnitude),
            _ => panic!()
        }
    }).fold((0,0), |acc, val| (acc.0+val.0, acc.1+val.1))

}