use itertools::Itertools;
use std::{cmp, env, fs};

#[derive(Debug, Clone)]
struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(pos: u32) -> Player {
        Player {
            pos: pos - 1,
            score: 0,
        }
    }
}

fn part_one(mut player1: Player, mut player2: Player) {
    let mut idx = 0; // Go 0-99 inclusive, wrap around
    let mut player1_moving = true;
    let mut count = 0;

    while player1.score < 1000 && player2.score < 1000 {
        let dist: u32 = (0..3)
            .map(|_i| {
                idx = (idx + 1) % 100;
                idx
            })
            .sum();

        let player = if player1_moving {
            &mut player1
        } else {
            &mut player2
        };

        player.pos = (player.pos + dist) % 10;
        player.score += player.pos + 1;
        // println!(
        //     "Moving player{} {} steps to pos {}, score is {}",
        //     if player1_moving { 1 } else { 2 },
        //     dist,
        //     player.pos + 1,
        //     player.score
        // );
        player1_moving = !player1_moving;
        count += 3;
    }

    println!(
        "Part one: {}",
        count * cmp::min(player1.score, player2.score)
    );
}

fn waiting_scores(universes: &Vec<Vec<Vec<Vec<usize>>>>) -> usize {
    universes.iter().flatten().flatten().flatten().sum()
}

fn simulate_step(
    universes: Vec<Vec<Vec<Vec<usize>>>>,
) -> (Vec<Vec<Vec<Vec<usize>>>>, usize, usize) {
    let mut new_universes = vec![vec![vec![vec![0; 21]; 21]; 10]; 10]; // 4d array of score counts.
    let mut p1_winners = 0;
    let mut p2_winners = 0;

    let die_times = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    universes.into_iter().enumerate().for_each(|(pos1, a)| {
        a.into_iter().enumerate().for_each(|(pos2, b)| {
            b.into_iter().enumerate().for_each(|(score1, c)| {
                c.into_iter()
                    .enumerate()
                    .for_each(|(score2, universe_count)| {
                        if universe_count > 0 {
                            for (die1, times1) in die_times {
                                let new_pos1 = (pos1 + die1) % 10;
                                let new_score1 = score1 + new_pos1 + 1;
                                // println!("New pos {} new score {}", new_pos, new_score);
                                if new_score1 < 21 {
                                    // Simulate player 2 for these universes
                                    for (die2, times2) in die_times {
                                        let new_pos2 = (pos2 + die2) % 10;
                                        let new_score2 = score2 + new_pos2 + 1;

                                        if new_score2 < 21 {
                                            // Transfer universes here
                                            new_universes[new_pos1][new_pos2][new_score1]
                                                [new_score2] += universe_count * times2 * times1;
                                        } else {
                                            p2_winners += universe_count * times2;
                                        }
                                    }
                                } else {
                                    p1_winners += universe_count * times1;
                                }
                            }
                        }
                    });
            });
        });
    });

    (new_universes, p1_winners, p2_winners)
}

fn part_two(player1: Player, player2: Player) {
    let mut universes = vec![vec![vec![vec![0; 21]; 21]; 10]; 10]; // 4d array of score counts. First axis is position, second is score, value is # of universes at that score

    // set initial condition

    universes[player1.pos as usize][player2.pos as usize][0][0] = 1;

    let mut player1_winners = 0;
    let mut player2_winners = 0;

    loop {
        let out = simulate_step(universes);
        universes = out.0;
        player1_winners += out.1;
        player2_winners += out.2;
        println!(
            "Num universes: {}, P1 winners {}, P2 winners {}",
            waiting_scores(&universes),
            player1_winners,
            player2_winners
        );
        if waiting_scores(&universes) == 0 {
            println!("Max is {}", std::cmp::max(player1_winners, player2_winners));
            break;
        }
    }
}

fn main() {
    let (filename, _sample_param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split('\n');
    let (player1, player2) = input_lines
        .map(|line| Player::new(line.split(": ").last().unwrap().parse().unwrap()))
        .collect_tuple()
        .unwrap();

    part_one(player1.clone(), player2.clone());
    part_two(player1, player2)
}
