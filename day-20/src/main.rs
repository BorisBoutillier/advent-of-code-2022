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

    let decryption_key = if part == "1" { 1 } else { 811589153 };
    // Same value can be present multiple time, so keep line index to desambiguate
    let mut numbers = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(idx, l)| (idx, l.parse::<i64>().unwrap() * decryption_key))
        .collect::<Vec<_>>();
    let size = numbers.len() as i64;
    println!("Numbers size: {}", size);
    //println!("Start: {:?}", numbers);
    let decryption_order = numbers.clone();
    let n_loop = if part == "1" { 1 } else { 10 };
    for _n in 0..n_loop {
        for &value in decryption_order.iter() {
            //println!("{:?}", numbers);
            //println!("Value: {}", value.1);
            let index = numbers
                .iter()
                .enumerate()
                .find(|(_, &v)| v == value)
                .unwrap()
                .0;
            //println!("Index: {}", index);
            numbers.remove(index);

            let new_index = (index as i64 + value.1).rem_euclid(size - 1);
            numbers.insert(new_index as usize, value);
            //println!("New index: {}", new_index);
        }
        //println!("End loop {} {:?}", _n, numbers);
    }
    let index_0 = numbers
        .iter()
        .enumerate()
        .find(|(_, &v)| v.1 == 0)
        .unwrap()
        .0;
    let v1000 = numbers[(index_0 + 1000) % size as usize].1;
    let v2000 = numbers[(index_0 + 2000) % size as usize].1;
    let v3000 = numbers[(index_0 + 3000) % size as usize].1;
    println!(
        "id 0: {} => {} + {} + {} = {}",
        index_0,
        v1000,
        v2000,
        v3000,
        v1000 + v2000 + v3000
    );
}
