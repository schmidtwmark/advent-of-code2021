
use std::{env, fs};

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .expect(&format!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split("\n");
}