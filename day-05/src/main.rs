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
    let file_s = fs::read_to_string(filename).expect("Issue reading file");
    let content = file_s.split("\n\n").collect::<Vec<_>>();
    assert_eq!(content.len(), 2);

    let stack_s = content[0].split('\n').collect::<Vec<_>>();
    assert!(
        stack_s
            .iter()
            .map(|l| l.len())
            .collect::<HashSet<_>>()
            .len()
            == 1
    );
    let n_stack = stack_s[0].len() / 4 + 1;
    let mut stacks = vec![vec![]; n_stack];
    for entry in stack_s.iter().take(stack_s.len() - 1) {
        for (j, stack) in stacks.iter_mut().enumerate() {
            let idx = j * 4 + 1;
            let value = entry[idx..idx + 1].to_string();
            if value != " " {
                stack.insert(0, value);
            }
        }
    }
    println!("Starting stacks: {:?}", stacks);

    for line in content[1].split('\n') {
        let entries = line.split_ascii_whitespace().collect::<Vec<_>>();
        if entries.is_empty() {
            break;
        }
        //println!("Entry: {:?}", entries);
        let qty = entries[1].parse::<usize>().unwrap();
        let start = entries[3].parse::<usize>().unwrap() - 1;
        let end = entries[5].parse::<usize>().unwrap() - 1;
        let new_start_len = stacks[start].len() - qty;
        let mut moved = stacks[start][new_start_len..].to_vec();
        if part == "1" {
            moved.reverse();
        }
        stacks[end].extend_from_slice(&moved);
        stacks[start].truncate(new_start_len);
        //println!(" -> stacks: {:?}", stacks);
    }

    println!("End stacks: {:?}", stacks);
    let code = stacks
        .iter_mut()
        .map(|s| s.pop().unwrap())
        .collect::<String>();
    println!("Code: {}", code)
}
