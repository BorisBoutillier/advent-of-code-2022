use std::{env, fs};

fn main() {
    // Find input file name
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Expecting an input file path, found {:?}", args);
    }
    let filename = args[1].clone();

    let mut total_score_a = 0u32;
    let mut total_score_b = 0u32;
    for line in fs::read_to_string(filename)
        .expect("Issue reading file")
        .lines()
    {
        let entries = line.split(' ').collect::<Vec<_>>();
        if entries.len() != 2 {
            panic!("Unexpected line content : {:?}", entries)
        };
        let score_a1 = match (entries[0], entries[1]) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            x => panic!("Unexpected oppoenent entry {:?}", x),
        };
        let score_a2 = match entries[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            x => panic!("Unexpected me entry {}", x),
        };
        let score_a = score_a1 + score_a2;
        total_score_a += score_a;

        let score_b1 = match (entries[0], entries[1]) {
            ("A", "X") => 3, // Scissor
            ("A", "Y") => 1, // Rock
            ("A", "Z") => 2, // Paper
            ("B", "X") => 1, // Rock
            ("B", "Y") => 2, // Paper
            ("B", "Z") => 3, // Scissor
            ("C", "X") => 2, // Paper
            ("C", "Y") => 3, // Scissor
            ("C", "Z") => 1, // Rock
            x => panic!("Unexpected oppoenent entry {:?}", x),
        };
        let score_b2 = match entries[1] {
            "X" => 0, // Lose
            "Y" => 3, // Draw
            "Z" => 6, // Win
            x => panic!("Unexpected me entry {}", x),
        };
        let score_b = score_b1 + score_b2;
        total_score_b += score_b;
    }
    println!("Total score A {}", total_score_a);
    println!("Total score B {}", total_score_b);
}
