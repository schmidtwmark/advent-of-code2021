use std::fs;
use itertools::Itertools;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines = contents.split("\n");

    let nums : Vec<i32> = lines.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let mut bingos :Vec<Bingo> = lines
        .chunks(6).into_iter()
        .map(|chunk| {
            chunk.map(|s| { 
                // println!("s: {:?}", s);
                s.split_whitespace().collect() 
            }).collect()
        }).map(|vals| { Bingo::new(vals)}).collect();

    let mut winning_indices : Vec<usize> = vec![];
    for num in nums {
        let mut index = 0;
        for bingo in &mut bingos {
            bingo.number_called(&num);
            let answer = bingo.check();
            if answer > 0 {
                if !winning_indices.iter().any(|candidate| {
                    candidate == &index 
                }) {
                    println!("Found answer, is {}", answer * num);
                    winning_indices.push(index);
                }
            }
            index+= 1;
        }
    }

    println!("No answer found");

}

const DIMENSION: usize= 5;

struct Bingo {
    // TODO use fixed size arrays here
    rows: Vec<Vec<Option<i32>>>,
}


impl Bingo {
    fn new(input: Vec<Vec<&str>>) -> Bingo {  
        Bingo {
            rows: input.iter().skip(1).map(|row| {
                row.iter().map(|value| {
                    Some(value.parse().unwrap())
                }).collect()
            }).collect()
        }
    }

    fn number_called(self: &mut Self, called: &i32) {
        // self.rows = self.rows.iter().map(|row| {
        //     row.into_iter().map(|value| {
        //         match value {
        //             Some(actual) => if actual == called {None} else {value.as_ref()}
        //             None => value.as_ref()
        //         }
        //     }).collect()
        // }).collect();
        for row in &mut self.rows {
            for value in row {
                match value {
                    Some(actual) => if actual == called { *value = None},
                    None => ()
                }
            }
        }
    }

    // Returns 0 if the card is not a winner, otherwise sums up everything else
    fn check(self: &Self) -> i32 {
        let any_rows_filled = self.rows.iter().any(|row| {
            row.iter().all(|value| value.is_none())
        });

        let any_cols_filled = self.rows.iter().fold(vec![0; DIMENSION], |mut acc, row| {
            row.iter().enumerate().for_each(|(i, value)| {
                if value.is_none() {
                    acc[i] += 1;
                }
            });
            acc
        }).iter().any(|column_none_count| {
            column_none_count == &DIMENSION
        });
        if any_cols_filled || any_rows_filled {
            Iterator::flatten(self.rows.iter()).fold(0, |acc, val| {
                match val {
                    Some(value) => acc + value,
                    None => acc
                }
            })
        } else {
            0
        }

    }
}
