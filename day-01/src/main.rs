use std::{env, fs};

fn main() {
    // Find input file name
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Expecting an input file path, found {:?}", args);
    }
    let filename = args[1].clone();
    let mut food_per_elf: Vec<Vec<u32>> = vec![];

    // Read input file as string, and fill the food_per_elf data
    let content =
        fs::read_to_string(&filename).unwrap_or_else(|_| panic!("file '{}' not found", filename));
    let mut cur_foods = vec![];
    for line in content.lines() {
        if line.is_empty() {
            food_per_elf.push(cur_foods);
            cur_foods = vec![];
        } else {
            cur_foods.push(line.parse::<u32>().expect("Not a number"));
        }
    }

    // Count total calories per elf
    let mut calories_per_elf = food_per_elf
        .iter()
        .map(|calories| calories.iter().sum::<u32>())
        .collect::<Vec<_>>();

    // Evaluates data
    println!("Number of elf {}", calories_per_elf.len());
    println!(
        "Maximum calories {}",
        calories_per_elf.iter().max().expect("No max")
    );
    calories_per_elf.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!(
        "Top 3 total calories {}",
        calories_per_elf.iter().take(3).sum::<u32>()
    );
}
