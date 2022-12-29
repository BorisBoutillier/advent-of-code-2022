use std::{env, fs};

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

    let mut x_t = vec![1i32];
    for line in fs::read_to_string(filename).expect("no file").lines() {
        let content = line.split(' ').collect::<Vec<_>>();
        let x = *(x_t.last().unwrap());
        if content[0] == "noop" {
            x_t.push(x);
        } else {
            assert!(
                content[0] == "addx" && content.len() == 2,
                "content: {:?}",
                content
            );
            let v = content[1].parse::<i32>().unwrap();
            x_t.push(x);
            x_t.push(x + v);
        }
    }
    if part == "1" {
        let mut total = 0;
        for i in (20..=220).step_by(40) {
            println!("{:3}th : {}", i, x_t[i - 1]);
            total += i as i32 * x_t[i - 1];
        }
        println!("Total: {}", total);
    } else {
        let mut idx = 1;
        let mut crt = String::from("");
        while idx < x_t.len() {
            let crt_x = (idx - 1) % 40;
            let c = if (x_t[idx - 1] - crt_x as i32).abs() <= 1 {
                "#"
            } else {
                "."
            };
            crt += c;
            if idx < 21 {
                println!("i: {}, crt_x: {} x:{}", idx, crt_x, x_t[idx - 1],);
                println!("CRT: {}", crt);
            }
            if idx % 40 == 0 {
                crt += "\n";
            }
            idx += 1;
        }
        println!("CRT:\n{}", crt);
    }
}
