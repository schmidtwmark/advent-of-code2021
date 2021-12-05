use std::fs;
// use itertools::Itertools;

type Point = (usize,usize);
type Segment = (Point, Point);
const DIMENSION: usize = 1000;
const filename: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let input_lines = contents.split("\n");
    let segments : Vec<Segment>= input_lines.map(|s| { 
        let pts: Vec<Point> = s.split(" -> ").map(|point_str| {
            let xy: Vec<usize> = point_str.split(",").map(|i| i.parse().unwrap()).collect();
            (xy[0], xy[1])
        }).collect();
        (pts[0], pts[1])
    }).collect();
    println!("segments: {:?}", segments);

    let mut board = [[0; DIMENSION]; DIMENSION]; // 2d array with 1m elements
    for segment in segments {
        apply_segment(&segment, &mut board);
    }
    println!("\n");
    // print_board(&board);
    println!("score: {:?}", score(&board));
}

fn apply_segment(segment: &Segment, board: &mut [[i32; DIMENSION]; DIMENSION]) {
    let ((x1,y1),(x2,y2)) = segment;
    let (y_1, y_2) = if y2 > y1 {(y1, y2)} else {(y2, y1)};
    let (x_1, x_2) = if x2 > x1 {(x1, x2)} else {(x2, x1)};
    if x1 == x2 { // vertical
        println!("Drawing vertical {:?}", segment);
        for y in *y_1..=*y_2 {
            board[y][*x1] += 1;
        }
    } else if y1 == y2 { //horizontal
        println!("Drawing horizontal {:?}", segment);
        for x in *x_1..=*x_2 {
            board[*y1][x] += 1;
        }
    } else if (y_2 - y_1) == (x_2 - x_1) { // 45 degree diagonal
        println!("Drawing diagonal {:?}", segment);
        let length = y_2 - y_1;
        if(x2 > x1) {
            if (y2 > y1) {
                for i in 0..=length {
                    board[*y1 + i][*x1 + i] += 1;
                }
            } else {
                for i in 0..=length {
                    board[*y1 - i][*x1 + i] += 1;
                }
            }
        } else {
            if (y2 > y1) {
                for i in 0..=length {
                    board[*y1 + i][*x1 - i] += 1;
                }
            } else {
                for i in 0..=length {
                    board[*y1 - i][*x1 - i] += 1;
                }
            }

        }
    } else { //non 45 degree diagonal
        println!("Ignoring diagonal {:?}", segment);
    }
    // print_board(&board);
}

fn score(board: &[[i32; DIMENSION]; DIMENSION]) -> usize {
    board.iter().flatten().filter(|v| **v >= 2).count()
}
fn print_board(board: &[[i32; DIMENSION]; DIMENSION]) {
    board.iter().enumerate().for_each(|(i,row)| {
        println!("{} {:?}", i, row);
    });
}
