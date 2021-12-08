
use std::{env, fs, collections::HashMap};
use itertools::Itertools;


fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split('\n');
    let rows : Vec<(Vec<String>, Vec<String>)> = input_lines.map(|s| {
        let (input, output) = s.split(" | ")
        .map(|section| {
            let vec = section.split(' ').map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort_unstable();
                chars.into_iter().collect::<String>()
            }).collect_vec();
            vec
        }).collect_tuple().unwrap();
        (input, output)
    }).collect_vec();

    // println!("Rows: {:?}", rows);
    println!("Part one: {}", part_one(&rows));
    println!("Part two: {}", part_two(&rows));


}

// Count 1s, 4s, 7s, 8s in output section
fn part_one(rows: &[(Vec<String>, Vec<String>)]) -> usize{
    let (mut ones, mut fours, mut sevens, mut eights) = (0,0,0,0);

    rows.iter().for_each(|(_input, output)| {
        ones += output.iter().filter(|s| s.len() == 2).count();
        fours += output.iter().filter(|s| s.len() == 4).count();
        sevens += output.iter().filter(|s| s.len() == 3).count();
        eights += output.iter().filter(|s| s.len() == 7).count();
    });
    ones + fours + sevens + eights
}

fn string_contains_char(target: &str, c: &char) -> bool {
    target.contains(|candidate| c == &candidate)
}

// sum output values 
fn part_two(rows: &[(Vec<String>, Vec<String>)]) -> usize {
    let mut sum = 0;
    rows.iter().for_each(|(input, output)| {
        let mut known_mappings : HashMap<char, char> = HashMap::new();
        let mut known_digits : [&str; 10 ]= [""; 10];
        let empty_string = String::from("");

        // let total_iter = input.iter().chain(output.iter());
        known_digits[1] = input.iter().find(|s| s.len() == 2).unwrap_or(&empty_string);
        known_digits[4] = input.iter().find(|s| s.len() == 4).unwrap_or(&empty_string);
        known_digits[7] = input.iter().find(|s| s.len() == 3).unwrap_or(&empty_string);
        known_digits[8] = input.iter().find(|s| s.len() == 7).unwrap_or(&empty_string);

        
        // Anything not common between 1 and 7 is the top segment A
        known_mappings.insert('a', known_digits[7].chars().find(|c| string_contains_char(known_digits[1], c)).unwrap());

        //3 is 7 plus two letters
        known_digits[3] = input.iter().filter(|s| s.len() == 5 && !known_digits.contains(&s.as_str())).find(|s| 
            s.chars().filter(|c| 
                !string_contains_char(known_digits[7], c)).count() == 2).unwrap();
        
        // anything shared between 3,4 and 8 that is not in 1 is middle segment D
        known_mappings.insert('d', known_digits[8].chars().find(|c| 
            string_contains_char(known_digits[3], c) && 
            string_contains_char(known_digits[4], c) && 
            !string_contains_char(known_digits[1], c))
        .unwrap());
        
        // zero is 8 - middle
        let zero_string = known_digits[8].chars().filter(|c| c != &known_mappings[&'d']).collect::<String>();
        known_digits[0] = &zero_string;

        // 9 has length 6, is not in the list, and contains all elements of 1
        known_digits[9] = input.iter().filter(|s| s.len() == 6 && !known_digits.contains(&s.as_str())).find( |s|
            known_digits[1].chars().all(|c| 
            string_contains_char(s, &c))
        ).unwrap();

        // 6 is the last known element of length 6
        known_digits[6] = input.iter().find(|s| s.len() == 6 && !known_digits.contains(&s.as_str())).unwrap();

        // 5 is 6 but missing an element
        known_digits[5] = input.iter().filter(|s| s.len() == 5 && !known_digits.contains(&s.as_str())).find( |s|
            known_digits[6].chars().filter(|c| 
            !string_contains_char(s, c)).count() == 1
        ).unwrap();
        known_digits[2] = input.iter().find(|s| s.len() == 5 && !known_digits.contains(&s.as_str())).unwrap();
        println!("input: {:?} | {:?} ", input, output);
        println!("known_digits: {:?}\nknown_mappings: {:?}", known_digits, known_mappings);
        let output_value :usize = output.iter().map(|s|
            known_digits.iter().find_position(|mapping|
                mapping == &&s.as_str()
            ).unwrap().0.to_string()).collect::<String>().parse().unwrap();
        println!("output: {:?}", output_value);
        sum += output_value
    });

    sum

}