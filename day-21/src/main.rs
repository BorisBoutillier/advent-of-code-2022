use std::{collections::HashMap, env, fs};

#[derive(Clone, Debug)]
enum Op {
    Value(u64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Humn,
}
impl Op {
    fn new(content: Vec<&str>) -> Op {
        if content.len() == 3 {
            match content[1] {
                "+" => Op::Add(content[0].to_string(), content[2].to_string()),
                "-" => Op::Sub(content[0].to_string(), content[2].to_string()),
                "*" => Op::Mul(content[0].to_string(), content[2].to_string()),
                "/" => Op::Div(content[0].to_string(), content[2].to_string()),
                _ => panic!(),
            }
        } else {
            assert_eq!(content.len(), 1);
            Op::Value(content[0].parse::<u64>().unwrap())
        }
    }
}
#[derive(Clone, Debug)]
struct World {
    monkeys: HashMap<String, Op>,
    part_2_root: (String, String),
}
impl World {
    fn new(filename: &str, part: &str) -> World {
        let mut monkeys = HashMap::new();
        let mut part_2_root = (String::from(""), String::from(""));
        for line in fs::read_to_string(filename).unwrap().lines() {
            let split = line.split(':').collect::<Vec<_>>();
            let content = split[1].split_whitespace().collect::<Vec<&str>>();
            let name = split[0].to_string();
            if name == "root" && part == "2" {
                part_2_root = (content[0].to_string(), content[2].to_string());
            } else if name == "humn" && part == "2" {
                monkeys.insert(String::from("humn"), Op::Humn);
            } else {
                monkeys.insert(split[0].to_string(), Op::new(content));
            }
        }
        World {
            monkeys,
            part_2_root,
        }
    }
    fn get_value(&self, name: &str) -> Option<u64> {
        let value = match self.monkeys[name].clone() {
            Op::Value(v) => Some(v),
            Op::Humn => {
                //println!("get Humn");
                None
            }
            Op::Add(m0, m1) => {
                let v0 = self.get_value(&m0);
                let v1 = self.get_value(&m1);
                if v0.is_none() || v1.is_none() {
                    None
                } else {
                    Some(v0.unwrap() + v1.unwrap())
                }
            }
            Op::Sub(m0, m1) => {
                let v0 = self.get_value(&m0);
                let v1 = self.get_value(&m1);
                if v0.is_none() || v1.is_none() {
                    None
                } else {
                    Some(v0.unwrap() - v1.unwrap())
                }
            }
            Op::Mul(m0, m1) => {
                let v0 = self.get_value(&m0);
                let v1 = self.get_value(&m1);
                if v0.is_none() || v1.is_none() {
                    None
                } else {
                    Some(v0.unwrap() * v1.unwrap())
                }
            }
            Op::Div(m0, m1) => {
                let v0 = self.get_value(&m0);
                let v1 = self.get_value(&m1);
                if v0.is_none() || v1.is_none() {
                    None
                } else {
                    Some(v0.unwrap() / v1.unwrap())
                }
            }
        };
        value
    }
    fn solve_humn(&mut self) -> u64 {
        let mut value = self.get_value(&self.part_2_root.1).unwrap();
        let mut current = self.part_2_root.0.clone();
        loop {
            let op = self.monkeys[&current].clone();
            //println!("Current: {}  -> {:?}", current, op);
            match op {
                Op::Humn => {
                    return value;
                }
                Op::Value(_) => {
                    panic!()
                }
                Op::Add(m0, m1) => {
                    let v0 = self.get_value(&m0);
                    let v1 = self.get_value(&m1);
                    assert!(v0.is_some() || v1.is_some());
                    if v0.is_some() {
                        current = m1;
                        value -= v0.unwrap();
                    } else {
                        current = m0;
                        value -= v1.unwrap();
                    }
                }
                Op::Sub(m0, m1) => {
                    let v0 = self.get_value(&m0);
                    let v1 = self.get_value(&m1);
                    assert!(v0.is_some() || v1.is_some());
                    if v0.is_some() {
                        current = m1;
                        value = v0.unwrap() - value;
                    } else {
                        current = m0;
                        value += v1.unwrap();
                    }
                }
                Op::Mul(m0, m1) => {
                    let v0 = self.get_value(&m0);
                    let v1 = self.get_value(&m1);
                    assert!(v0.is_some() || v1.is_some());
                    if v0.is_some() {
                        current = m1;
                        value /= v0.unwrap();
                    } else {
                        current = m0;
                        value /= v1.unwrap();
                    }
                }
                Op::Div(m0, m1) => {
                    let v0 = self.get_value(&m0);
                    let v1 = self.get_value(&m1);
                    assert!(v0.is_some() || v1.is_some());
                    if v0.is_some() {
                        current = m1;
                        value = v0.unwrap() / value;
                    } else {
                        current = m0;
                        value *= v1.unwrap();
                    }
                }
            }
        }
    }
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
    let mut world = World::new(&filename, &part);

    if part == "1" {
        println!("Value: {}", world.get_value("root").unwrap());
    } else {
        println!("Humn: {}", world.solve_humn());
    }
}
