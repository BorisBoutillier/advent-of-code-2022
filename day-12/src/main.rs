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

    let mut heights = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in fs::read_to_string(filename)
        .expect("no file")
        .lines()
        .enumerate()
    {
        let mut h = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x, y);
                h.push(0);
            } else if c == 'E' {
                end = (x, y);
                h.push(25);
            } else {
                h.push(c as u32 - 'a' as u32);
            }
        }
        heights.push(h);
    }
    println!("Heights: {:?}", heights);
    println!("End {:?}", end);

    let starts = if part == "1" {
        vec![start]
    } else {
        heights
            .iter()
            .enumerate()
            .flat_map(|(y, h)| {
                h.iter()
                    .enumerate()
                    .filter_map(|(x, e)| if e == &0 { Some((x, y)) } else { None })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };
    println!("All Starts {:?}", starts.len());
    let mut shortest_path_to_end = u32::MAX;
    let mut _shortest_path = None;
    let mut shortest = vec![vec![1000u32; heights[0].len()]; heights.len()];

    let mut pathes = vec![vec![(end, 0)]];

    while !pathes.is_empty() {
        let path = pathes.pop().unwrap();
        let (cur_pos, cur_dist) = path.last().unwrap();
        if cur_dist > &shortest_path_to_end {
            continue;
        }
        shortest[cur_pos.1][cur_pos.0] = *cur_dist;
        if starts.contains(cur_pos) {
            let path_len = path.len() as u32;
            if path_len < shortest_path_to_end {
                shortest_path_to_end = path_len;
                _shortest_path = Some(path);
            }
            continue;
        }
        let cur_height = heights[cur_pos.1][cur_pos.0];
        for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
            let dest_x = cur_pos.0 as i32 + dx;
            let dest_y = cur_pos.1 as i32 + dy;
            if dest_x < 0
                || dest_x >= heights[0].len() as i32
                || dest_y < 0
                || dest_y >= heights.len() as i32
            {
                continue;
            }
            let dest = (dest_x as usize, dest_y as usize);
            if heights[dest.1][dest.0] + 1 >= cur_height && shortest[dest.1][dest.0] > cur_dist + 1
            {
                shortest[dest.1][dest.0] = cur_dist + 1;
                let mut new_path = path.clone();
                new_path.push((dest, cur_dist + 1));
                pathes.push(new_path);
            }
        }
    }
    println!("Shortest {}", shortest_path_to_end - 1);
}
