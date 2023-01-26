use std::{env, fs};

fn snafu_to_i64(s: &str) -> i64 {
    let mut v = 0i64;
    for (i, c) in s.chars().rev().enumerate() {
        let cv = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        };
        v += cv * 5i64.pow(i as u32)
    }
    v
}
fn i64_to_snafu(v: i64) -> String {
    let mut s = vec![];
    let mut v = v;
    assert!(v > 0);
    while v > 0 {
        v += 2;

        s.insert(
            0,
            match v % 5 {
                0 => String::from("="),
                1 => String::from("-"),
                2 => String::from("0"),
                3 => String::from("1"),
                4 => String::from("2"),
                _ => panic!(),
            },
        );
        v /= 5;
    }
    s.into_iter().collect::<String>()
}

fn _test() {
    for i in 1..11 {
        println!("{} : {}", i, i64_to_snafu(i));
    }
}

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
    let mut total = 0;
    for line in fs::read_to_string(&filename).unwrap().lines() {
        total += snafu_to_i64(line);
    }
    println!("Total: {} -> {}", total, i64_to_snafu(total));
}
