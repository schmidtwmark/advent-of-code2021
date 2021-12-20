use itertools::Itertools;
use std::{env, fs};

fn print_image<U>(image: &[U])
where
    U: AsRef<[i32]> + std::fmt::Debug,
{
    println!("\n");
    for line in image {
        let out: String = line
            .as_ref()
            .iter()
            .map(|i| match i {
                // 0 => ' ',
                // 1 => '\u{2588}',
                0 => '.',
                1 => '#',
                _ => unreachable!(),
            })
            .collect();
        println!("{}", out);
    }
    println!("\n");
}

fn enhance<U>(image: &[U], algorithm: &[i32], generation: i32) -> Vec<Vec<i32>>
where
    U: AsRef<[i32]>,
{
    let width = image.len();
    let height = image[0].as_ref().iter().count();
    let mut new_image = vec![vec![0; width + 2]; height + 2];
    println!("image dims: ({},{})", width, height);
    println!(
        "new image dims: ({},{})",
        new_image.len(),
        new_image[0].len()
    );
    let outer = if generation % 2 == 1 && algorithm[0] == 1 {
        1
    } else {
        0
    };
    let mut old_image_big_frame = vec![vec![outer; width + 4]; height + 4];

    // Oh to do this by copying the slice, but the compiler will not shut up
    for x in 0..width {
        for y in 0..height {
            old_image_big_frame[x + 2][y + 2] = image[x].as_ref()[y];
        }
    }

    // print_image(&old_image_big_frame);

    for (new_x, row) in new_image.iter_mut().enumerate().take(width + 2) {
        for (new_y, val) in row.iter_mut().enumerate().take(height + 2) {
            let big_x = new_x + 1;
            let big_y = new_y + 1;

            let x_range = (big_x - 1)..=(big_x + 1);
            let y_range = (big_y - 1)..=(big_y + 1);

            let mut out = 0;
            for x in x_range {
                for y in y_range.clone() {
                    out <<= 1;
                    out |= old_image_big_frame[x][y];
                }
            }
            // println!("For newx newy ({},{}), algo pos is {:b}", new_x, new_y, out);
            *val = algorithm[out as usize];
        }
    }

    new_image
}

fn count_lit<U>(image: &[U]) -> usize
where
    U: AsRef<[i32]>,
{
    image.iter().fold(0, |acc, u| {
        acc + u.as_ref().iter().fold(0, |acc2, v| match v {
            0 => acc2,
            1 => acc2 + 1,
            _ => unreachable!(),
        })
    })
}

fn main() {
    let (filename, _sample_param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let (algorithm, image_list) = contents
        .split("\n\n")
        .map(|str| {
            str.chars()
                .filter_map(|c| match c {
                    '.' => Some(0),
                    '#' => Some(1),
                    '\n' => Some(2),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    let mut image = image_list
        .split(|i| *i == 2)
        .map(|slice| slice.iter().cloned().collect_vec())
        .collect_vec();

    print_image(&image);

    for i in 0..50 {
        image = enhance(&image, &algorithm, i);
        print_image(&image);
    }

    println!("Num pixels lit is {}", count_lit(&image));
}
