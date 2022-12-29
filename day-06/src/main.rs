use std::{collections::HashSet, env, fs};

fn main() {
    // Find input file name
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
    let chars = fs::read_to_string(filename)
        .expect("Issue reading file")
        .chars()
        .collect::<Vec<_>>();
    let exp_count = if part == "1" { 4 } else { 14 };
    for (i, entry) in chars.windows(exp_count).enumerate() {
        let s = entry.iter().collect::<HashSet<_>>();
        if s.len() == exp_count {
            println!(
                "First {} different chars ends at indice: {}",
                exp_count,
                i + exp_count
            );
            break;
        }
    }
}
