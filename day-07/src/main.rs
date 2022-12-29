use std::{collections::HashMap, env, fs};

#[derive(Debug, Clone)]
struct Node {
    dir: bool,
    nodes: HashMap<String, Node>,
    size: u32,
}
impl Node {
    fn new_dir() -> Node {
        Node {
            dir: true,
            nodes: HashMap::new(),
            size: 0,
        }
    }
    fn new_file(size: u32) -> Node {
        Node {
            dir: false,
            nodes: HashMap::new(),
            size,
        }
    }
    fn total_size(&self) -> u32 {
        self.size + self.nodes.values().map(|n| n.total_size()).sum::<u32>()
    }
    fn find_mut(&mut self, path: &[String]) -> &mut Node {
        if path.is_empty() {
            return self;
        }
        self.nodes.get_mut(&path[0]).unwrap().find_mut(&path[1..])
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

    let mut cur_dir = vec![];
    let mut top = Node::new_dir();

    for line in fs::read_to_string(filename).expect("No file").lines() {
        let entries = line.split_ascii_whitespace().collect::<Vec<_>>();
        if entries[0] == "$" {
            if entries[1] == "ls" {
                continue;
            }
            assert!(entries[1] == "cd");
            match entries[2] {
                "/" => {
                    cur_dir = vec![];
                }
                ".." => {
                    cur_dir.pop();
                }
                name => {
                    cur_dir.push(name.to_string());
                }
            }
        } else {
            let cur_node = top.find_mut(&cur_dir);
            let name = entries[1].to_string();
            match entries[0] {
                "dir" => {
                    cur_node.nodes.insert(name, Node::new_dir());
                }
                size => {
                    cur_node
                        .nodes
                        .insert(name, Node::new_file(size.parse::<u32>().unwrap()));
                }
            }
        }
    }
    println!("Total size: {}", top.total_size());

    if part == "1" {
        const SIZE_LIMIT: u32 = 100000;
        let mut total_size = 0u32;

        let mut stack = vec![(String::from(""), top)];
        while !stack.is_empty() {
            let (cur_name, cur_node) = stack.pop().unwrap();
            if !cur_node.dir {
                continue;
            }
            let cur_size = cur_node.total_size();
            if cur_size <= SIZE_LIMIT {
                println!("Found {}: {}", cur_name, cur_size);
                total_size += cur_size;
            }
            stack.extend_from_slice(&cur_node.nodes.into_iter().collect::<Vec<_>>());
        }

        println!(
            "Total size of directory at most {} : {}",
            SIZE_LIMIT, total_size
        );
    } else {
        const SYSTEM_SIZE: u32 = 70_000_000;
        const EXP_FREE_SIZE: u32 = 30_000_000;

        let space_needed = EXP_FREE_SIZE - (SYSTEM_SIZE - top.total_size());
        println!("Space needed: {}", space_needed);

        let mut dir_size = vec![];
        let mut stack = vec![(String::from(""), top)];
        while !stack.is_empty() {
            let (_cur_name, cur_node) = stack.pop().unwrap();
            if !cur_node.dir {
                continue;
            }
            let cur_size = cur_node.total_size();
            if cur_size >= space_needed {
                dir_size.push(cur_size);
            }
            stack.extend_from_slice(&cur_node.nodes.into_iter().collect::<Vec<_>>());
        }
        dir_size.sort();
        println!("Smallest directory to delete: {}", dir_size[0]);
    }
}
