use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Expecting an input file path, found {:?}", args);
    }
    let filename = args[1].clone();
    let file_s = fs::read_to_string(filename).expect("Issue reading file");
    let lines = file_s.lines().collect::<Vec<_>>();

    let mut total_included = 0u32;
    let mut total_overlap = 0u32;
    for line in lines {
        let mut elfs = line.split(',');
        let elf1_range = elfs
            .next()
            .unwrap()
            .split('-')
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let elf2_range = elfs
            .next()
            .unwrap()
            .split('-')
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(elf1_range.len(), 2);
        assert_eq!(elf2_range.len(), 2);
        let included = ((elf1_range[0] <= elf2_range[0]) && (elf1_range[1] >= elf2_range[1]))
            || ((elf2_range[0] <= elf1_range[0]) && (elf2_range[1] >= elf1_range[1]));
        if included {
            total_included += 1;
        }
        let overlap = !((elf1_range[1] < elf2_range[0]) || (elf1_range[0] > elf2_range[1]));
        if overlap {
            total_overlap += 1;
        }
    }
    println!("Total included: {}", total_included);
    println!("Total overlap: {}", total_overlap);
}
