use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Debug)]
struct Valve {
    flow: u32,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone)]
enum Action {
    GoTo(String),
    OpenValve(String),
    DoNothing,
}

#[derive(Debug, Clone)]
struct Path {
    opened_valves: HashSet<String>,
    position: Vec<String>,
    score: u32,
    seen: HashMap<Vec<String>, u32>,
    time: u32,
}
impl Path {
    fn new(count: usize) -> Path {
        Path {
            opened_valves: HashSet::new(),
            position: vec![String::from("AA"); count],
            score: 0,
            seen: HashMap::new(),
            time: 0,
        }
    }
    fn update(&mut self, valves: &HashMap<String, Valve>) {
        self.time += 1;
        self.score += self
            .opened_valves
            .iter()
            .map(|name| valves[name].flow)
            .sum::<u32>();
        let v = self.seen.entry(self.position.clone()).or_insert(0);
        *v = (*v).max(self.opened_valves.len() as u32);
    }
    fn next(&self, valves: &HashMap<String, Valve>) -> Vec<Path> {
        let mut pathes = vec![];
        let mut actions = vec![vec![]];
        for position in self.position.iter() {
            let mut new_actions = vec![];
            let mut my_actions = valves[position]
                .tunnels
                .iter()
                .map(|v| Action::GoTo(v.clone()))
                .collect::<Vec<_>>();
            my_actions.push(Action::OpenValve(position.clone()));
            for action in actions.iter() {
                for my_action in my_actions.iter() {
                    let mut new_action = action.clone();
                    new_action.push(my_action.clone());
                    new_actions.push(new_action);
                }
            }
            actions = new_actions;
        }
        for action in actions.iter() {
            let mut new_path = self.clone();
            let mut new_position = self.position.clone();
            let mut ok = true;
            for (i, my_action) in action.iter().enumerate() {
                match my_action {
                    Action::GoTo(valve_name) => {
                        new_position[i] = valve_name.clone();
                    }
                    Action::OpenValve(valve_name) => {
                        if valves[valve_name].flow > 0 && !self.opened_valves.contains(valve_name) {
                            new_path.opened_valves.insert(valve_name.clone());
                        } else {
                            ok = false;
                        }
                    }
                    _ => panic!(),
                }
            }
            if ok
                && (new_position == self.position
                    || !self.seen.contains_key(&new_position)
                    || self.seen[&new_position] < self.opened_valves.len() as u32)
            {
                new_path.position = new_position;
                pathes.push(new_path);
            }
        }
        pathes
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

    let mut valves = HashMap::new();
    let filename = args[1].clone();
    for line in fs::read_to_string(filename).unwrap().lines() {
        let content = line.split_whitespace().collect::<Vec<_>>();
        let name = content[1].to_string();
        let flow = content[4]
            .trim_matches(|c: char| !c.is_numeric())
            .parse::<u32>()
            .unwrap();
        let tunnels = content
            .iter()
            .skip(9)
            .map(|s| s.trim_end_matches(|c: char| c == ',').to_string())
            .collect::<Vec<_>>();
        valves.insert(name, Valve { flow, tunnels });
    }
    println!("Loaded {} valves", valves.len());

    let end_time = if part == "1" { 30 } else { 5 };
    let mut stack = vec![Path::new(if part == "1" { 1 } else { 2 })];
    let mut best_score = 0;
    let mut best_path = None;
    let mut c = 0;
    while !stack.is_empty() {
        c += 1;
        if c % 100000 == 0 {
            println!("{} : x{}", c, stack.len());
        }
        // Beware this implementation run 273417064 loops for my input. (27m on my laptop)
        let mut cur_path = stack.pop().unwrap();
        //println!("Cur: {:?}", cur_path);
        cur_path.update(&valves);
        if cur_path.time == end_time {
            if cur_path.score >= best_score {
                best_score = cur_path.score;
                best_path = Some(cur_path);
            }
        } else {
            // Do nothing
            //if let Some(new_path) = cur_path.open_valve(&valves) {
            //    stack.push(new_path);
            //}
            stack.extend(cur_path.next(&valves));
        }
    }
    println!("Count: {}", c);
    println!("Best score {}  : {:?}", best_score, best_path);
}
