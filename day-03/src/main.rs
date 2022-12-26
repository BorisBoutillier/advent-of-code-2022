use std::{collections::HashSet, env, fs};

fn main() {
    // Find input file name
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Expecting an input file path, found {:?}", args);
    }
    let filename = args[1].clone();
    let file_s = fs::read_to_string(filename).expect("Issue reading file");
    let lines = file_s.lines().collect::<Vec<_>>();

    let mut total_priorities = 0u32;
    for line in lines.iter() {
        let line_len = line.len();
        if line_len % 2 != 0 {
            panic!("Not an even line")
        };
        let first = &line[0..(line_len / 2)];
        let second = &line[(line_len / 2)..];
        let first_set = first.chars().collect::<HashSet<char>>();
        let second_set = second.chars().collect::<HashSet<char>>();
        //println!("First: {:?} ; Second: {:?}", first_set, second_set);
        let commons = first_set.intersection(&second_set).collect::<HashSet<_>>();
        assert!(
            commons.len() == 1,
            "More than one common item {:?}",
            commons
        );
        let common = **(commons.iter().next().unwrap());
        assert!(common.is_ascii());
        let priority = if common.is_ascii_uppercase() {
            (common as u32 - 'A' as u32) + 27
        } else {
            (common as u32 - 'a' as u32) + 1
        };
        //println!("Common {:?} = {} ", common, priority);
        total_priorities += priority;
    }
    println!("Total priorities: {}", total_priorities);

    let mut total_group_priorities = 0u32;
    for group_line in lines.chunks(3) {
        let item_0 = group_line[0].chars().collect::<HashSet<char>>();
        let item_1 = group_line[1].chars().collect::<HashSet<char>>();
        let item_2 = group_line[2].chars().collect::<HashSet<char>>();
        let commons_a = item_0
            .intersection(&item_1)
            .copied()
            .collect::<HashSet<_>>();
        let commons = commons_a.intersection(&item_2).collect::<HashSet<_>>();
        assert!(
            commons.len() == 1,
            "More than one common item {:?}",
            commons
        );
        let common = **(commons.iter().next().unwrap());
        let priority = if common.is_ascii_uppercase() {
            (common as u32 - 'A' as u32) + 27
        } else {
            (common as u32 - 'a' as u32) + 1
        };
        //println!("Group common {:?} = {} ", common, priority);
        total_group_priorities += priority;
    }
    println!("Total group priorities: {}", total_group_priorities);
}
