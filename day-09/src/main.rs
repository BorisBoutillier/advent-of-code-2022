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
    let mut moves = vec![];
    for line in fs::read_to_string(filename).expect("no file").lines() {
        let content = line.split(' ').collect::<Vec<_>>();
        let count = content[1].parse::<u32>().unwrap();
        let dir = content[0].to_string();
        (0..count).for_each(|_| moves.push(dir.clone()));
    }
    println!("Moves: {:?}", moves);

    let n_knot = if part == "1" { 2 } else { 10 };
    let mut seen = HashSet::new();
    let mut rope = vec![(0i32, 0i32); n_knot];
    for m in moves.iter() {
        rope[0] = match m.as_str() {
            "U" => (rope[0].0, rope[0].1 - 1),
            "D" => (rope[0].0, rope[0].1 + 1),
            "L" => (rope[0].0 - 1, rope[0].1),
            "R" => (rope[0].0 + 1, rope[0].1),
            _ => panic!(),
        };
        for i in 1..n_knot {
            let diff = knot_move(i, (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1));
            rope[i] = (rope[i].0 + diff.0, rope[i].1 + diff.1);
        }

        seen.insert(rope[n_knot - 1]);
        println!("{} -> {:?}", m, rope);
    }
    println!("Total occupied tiles: {}", seen.len());
}

fn knot_move(_i: usize, diff: (i32, i32)) -> (i32, i32) {
    assert!(diff.0.abs() <= 2);
    assert!(diff.1.abs() <= 2);
    if diff.0.abs() != 2 && diff.1.abs() != 2 {
        (0, 0)
    } else {
        let d0 = if diff.0.abs() == 2 {
            diff.0 / 2
        } else {
            diff.0
        };
        let d1 = if diff.1.abs() == 2 {
            diff.1 / 2
        } else {
            diff.1
        };
        (d0, d1)
    }
}
