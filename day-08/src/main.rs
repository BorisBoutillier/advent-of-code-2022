#![allow(clippy::needless_range_loop)]
use std::{collections::HashSet, env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!(
            "Expecting an input file path and a part number ( 1 or 2), found {:?}",
            args
        );
    }
    let part = args[2].clone();
    if part != "1" && part != "2" {
        panic!("part number must be 1 or 2, not {}", part)
    }
    let filename = args[1].clone();
    let mut trees = vec![];
    for line in fs::read_to_string(filename).expect("no file").lines() {
        trees.push(
            line.chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Vec<_>>(),
        );
    }
    println!("Trees: {:?}, {}", trees, trees.len());
    let height = trees.len();
    let width = trees[0].len();
    if part == "1" {
        let mut visible = HashSet::new();

        for y in 0..height {
            // Left to Right
            let mut cur_size = -1i32;
            for x in 0..width {
                if trees[y][x] > cur_size {
                    visible.insert((y, x));
                    cur_size = trees[y][x];
                }
            }
            // Right to left
            let mut cur_size = -1i32;
            for x in (0..width).rev() {
                if trees[y][x] > cur_size {
                    visible.insert((y, x));
                    cur_size = trees[y][x];
                }
            }
        }
        for x in 0..width {
            // Top to Bottom
            let mut cur_size = -1i32;
            for y in 0..height {
                if trees[y][x] > cur_size {
                    visible.insert((y, x));
                    cur_size = trees[y][x];
                }
            }
            // Bottom to top
            let mut cur_size = -1i32;
            for y in (0..height).rev() {
                if trees[y][x] > cur_size {
                    visible.insert((y, x));
                    cur_size = trees[y][x];
                }
            }
        }
        println!("Visible : {}", visible.len());
    } else {
        let mut best_score = 0u32;
        let mut best_tree = None;
        for y in 0..height {
            for x in 0..width {
                println!("DOING ({},{})", y, x);
                let mut score = 1u32;
                let size = trees[y][x];

                // Left to Right
                if x < width - 1 {
                    let mut count = 0u32;
                    for xx in (x + 1)..width {
                        count += 1;
                        if trees[y][xx] >= size {
                            break;
                        }
                    }
                    println!(" LtR {}", count);
                    score *= count;
                }

                // Right to Left
                if x > 0 {
                    let mut count = 0u32;
                    for xx in (0..x).rev() {
                        count += 1;
                        if trees[y][xx] >= size {
                            break;
                        }
                    }
                    score *= count;
                    println!(" RtL {}", count);
                }

                // Top to Bottom
                if y < height - 1 {
                    let mut count = 0u32;
                    for yy in (y + 1)..height {
                        count += 1;
                        if trees[yy][x] >= size {
                            break;
                        }
                    }
                    println!(" TtB {}", count);
                    score *= count;
                }

                // Bottom to Top
                if y > 0 {
                    let mut count = 0u32;
                    for yy in (0..y).rev() {
                        count += 1;
                        if trees[yy][x] >= size {
                            break;
                        }
                    }
                    println!(" BtT {}", count);
                    score *= count;
                }
                if score > best_score {
                    best_score = score;
                    best_tree = Some((y, x));
                }
                println!(" = {}", score);
                best_score = best_score.max(score);
            }
        }
        println!(
            "Best score: {} at ({},{})",
            best_score,
            best_tree.unwrap().0,
            best_tree.unwrap().1
        );
    }
}
