use std::{env, fs};

#[derive(Default, Clone, Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
    #[default]
    None,
}
impl Operation {
    pub fn parse(op: &str, value: &str) -> Operation {
        match op {
            "+" => Operation::Add(value.parse::<u64>().unwrap()),
            "*" => match value {
                "old" => Operation::Square,
                v => Operation::Mul(v.parse::<u64>().unwrap()),
            },
            _ => panic!(),
        }
    }
    pub fn apply(&self, v: u64) -> u64 {
        match self {
            Operation::Add(x) => v + x,
            Operation::Mul(x) => v * x,
            Operation::Square => v * v,
            Operation::None => panic!(),
        }
    }
}
#[derive(Default, Clone, Debug)]
struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub divisability: u64,
    pub true_dest: u64,
    pub false_dest: u64,
    pub inspect_count: u64,
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
    let mut monkeys = vec![];
    let mut monkey = Monkey::default();
    for line in fs::read_to_string(filename).expect("no file").lines() {
        let content = line.split_whitespace().collect::<Vec<_>>();
        if content.is_empty() {
            monkeys.push(monkey.clone());
        } else {
            match content[0] {
                "Monkey" => {
                    monkey = Monkey::default();
                }
                "Starting" => {
                    monkey.items = content
                        .iter()
                        .skip(2)
                        .map(|s| s.trim_end_matches(',').parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                }
                "Operation:" => {
                    monkey.operation = Operation::parse(content[4], content[5]);
                }
                "Test:" => {
                    monkey.divisability = content[3].parse::<u64>().unwrap();
                }
                "If" => match content[1] {
                    "true:" => {
                        monkey.true_dest = content[5].parse::<u64>().unwrap();
                    }
                    "false:" => {
                        monkey.false_dest = content[5].parse::<u64>().unwrap();
                    }
                    _ => panic!(),
                },
                _ => panic!("content: {:?}", content),
            }
        }
    }

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("{}: {:?}", i, monkey);
    }
    let modulo = 2 * monkeys.iter().map(|m| m.divisability).product::<u64>();

    let n_round = if part == "1" { 20 } else { 10000 };
    let div_3 = part == "1";
    for round in 0..n_round {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut moves = vec![];
            for &item in monkey.items.iter() {
                let mut worry = monkey.operation.apply(item);
                if div_3 {
                    worry /= 3
                };
                let worry = worry % modulo;
                let dest = if worry % monkey.divisability == 0 {
                    monkey.true_dest
                } else {
                    monkey.false_dest
                };
                moves.push((dest, worry));
                monkey.inspect_count += 1;
            }
            monkey.items = vec![];
            for (dest, item) in moves.into_iter() {
                monkeys[dest as usize].items.push(item);
            }
        }
        println!("After round: {}", round);
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("   {}: {:?}", i, monkey);
        }
    }
    let mut counts = monkeys.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));
    println!("Monkey business: {}", counts[0] * counts[1]);
}
