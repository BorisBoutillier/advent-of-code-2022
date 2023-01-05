use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    env, fs,
    num::NonZeroUsize,
};

use lru::LruCache;

#[derive(Debug)]
struct Valve {
    flow: u32,
    tunnels: Vec<u32>,
}

#[derive(Debug, Clone)]
enum Action {
    GoTo(u32),
    OpenValve(u32),
}

#[derive(Debug, Clone)]
struct Path {
    opened_valves: Vec<u32>,
    position: Vec<u32>,
    score: u32,
    min_score: u32,
    seen: HashMap<Vec<u32>, u32>,
    time: u32,
    end_time: u32,
}
impl Path {
    fn new(start_id: u32, count: usize, end_time: u32) -> Path {
        Path {
            opened_valves: vec![],
            position: vec![start_id; count],
            score: 0,
            min_score: 0,
            seen: HashMap::new(),
            time: 0,
            end_time,
        }
    }
    fn update(&mut self, valves: &HashMap<u32, Valve>) {
        self.time += 1;
        let tick_score = self
            .opened_valves
            .iter()
            .map(|id| valves[id].flow)
            .sum::<u32>();
        self.score += tick_score;
        self.min_score = self.score + tick_score * (self.end_time - self.time);
        let v = self.seen.entry(self.position.clone()).or_insert(0);
        *v = (*v).max(self.opened_valves.len() as u32);
    }
    fn next(&self, valves: &HashMap<u32, Valve>) -> Vec<Path> {
        let mut pathes = vec![];
        let mut actions = vec![vec![]];
        if self.position.len() == 2 && self.position[0] == self.position[1] {
            let mut new_actions = valves[&self.position[0]]
                .tunnels
                .iter()
                .map(|v| Action::GoTo(*v))
                .collect::<Vec<_>>();
            new_actions.push(Action::OpenValve(self.position[0]));
            actions = vec![];
            for (i, action0) in new_actions.iter().enumerate() {
                for action1 in new_actions.iter().skip(i) {
                    actions.push(vec![action0.clone(), action1.clone()])
                }
            }
        } else {
            for position in self.position.iter() {
                let mut new_actions = vec![];
                let mut my_actions = valves[position]
                    .tunnels
                    .iter()
                    .map(|v| Action::GoTo(*v))
                    .collect::<Vec<_>>();
                my_actions.push(Action::OpenValve(*position));
                for action in actions.iter() {
                    for my_action in my_actions.iter() {
                        let mut new_action = action.clone();
                        new_action.push(my_action.clone());
                        new_actions.push(new_action);
                    }
                }
                actions = new_actions;
            }
        }
        for action in actions.iter() {
            let mut new_path = self.clone();
            let mut new_position = self.position.clone();
            let mut ok = true;
            for (i, my_action) in action.iter().enumerate() {
                match my_action {
                    Action::GoTo(id) => {
                        new_position[i] = *id;
                    }
                    Action::OpenValve(id) => {
                        if valves[id].flow > 0 && !new_path.opened_valves.contains(id) {
                            new_path.opened_valves.push(*id);
                            new_path.opened_valves.sort();
                        } else {
                            ok = false;
                        }
                    }
                }
            }
            new_position.sort();
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

struct ValveIds {
    ids: HashMap<String, u32>,
    cur_id: u32,
}
impl ValveIds {
    fn new() -> ValveIds {
        ValveIds {
            ids: HashMap::new(),
            cur_id: 0,
        }
    }
    fn get(&mut self, name: &str) -> u32 {
        let name = name.to_string();
        match self.ids.entry(name) {
            Entry::Vacant(e) => {
                let id = self.cur_id;
                e.insert(self.cur_id);
                self.cur_id += 1;
                id
            }
            Entry::Occupied(e) => *e.get(),
        }
    }
}

struct Seen {
    cache: LruCache<(Vec<u32>, Vec<u32>), Vec<Option<u32>>>,
}
impl Seen {
    fn new() -> Seen {
        Seen {
            cache: LruCache::new(NonZeroUsize::new(5_000_000).unwrap()),
        }
    }
    fn seen_better(&mut self, path: &Path) -> bool {
        let position = path.position.to_vec();
        let opened_valves = path.opened_valves.to_vec();
        let entry = (opened_valves, position);
        let time = path.time as usize;
        match self.cache.get_mut(&entry) {
            None => {
                let mut scores = vec![None; 30];
                scores[time] = Some(path.score);
                self.cache.put(entry, scores);
                false
            }
            Some(scores) => {
                //if scores[time].is_some() && scores[time].unwrap() >= path.score {
                //    true
                //} else {
                //    scores[time] = Some(path.score);
                //    false
                //}
                if scores
                    .iter()
                    .enumerate()
                    .any(|(t, s)| t <= time && s.is_some() && s.unwrap() >= path.score)
                {
                    true
                } else {
                    scores[time] = Some(path.score);
                    false
                }
            }
        }
        // Entry::Vacant(e) => {
        //     e.insert(path.score);
        //     false
        // }
        // Entry::Occupied(mut e) => {
        //     if *e.get() >= path.score {
        //         true
        //     } else {
        //         e.insert(path.score);
        //         false
        //     }
        // }
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
    let mut valve_ids = ValveIds::new();
    let filename = args[1].clone();

    for line in fs::read_to_string(filename).unwrap().lines() {
        let content = line.split_whitespace().collect::<Vec<_>>();
        let id = valve_ids.get(content[1]);
        let flow = content[4]
            .trim_matches(|c: char| !c.is_numeric())
            .parse::<u32>()
            .unwrap();
        let tunnels = content
            .iter()
            .skip(9)
            .map(|s| s.trim_end_matches(|c: char| c == ',').to_string())
            .map(|valve_name| valve_ids.get(&valve_name))
            .collect::<Vec<_>>();
        valves.insert(id, Valve { flow, tunnels });
    }
    println!("Loaded {} valves", valves.len());

    let max_flow = valves.values().map(|v| v.flow).sum::<u32>();

    let end_time = if part == "1" { 30 } else { 26 };
    let n_actor = if part == "1" { 1 } else { 2 };
    let mut stack = vec![Path::new(valve_ids.get("AA"), n_actor, end_time)];
    let mut best_score = 0;
    let mut _best_path = None;
    let mut iterations = 0;
    let mut useless_count = 0;
    let mut all_seen = Seen::new();
    while !stack.is_empty() {
        iterations += 1;
        if iterations % 100000 == 0 {
            println!(
                "IT {}K Best {} : x{} , skipped {}K, Cache {}K",
                iterations / 1000,
                best_score,
                stack.len(),
                useless_count / 1000,
                all_seen.cache.len()
            );
        }
        // Implementation A on input.txt part 1 : 273417064 iterations ( 27 min )
        // Implementation B on input.txt part 1 :  18064955 iterations ( 3 min )
        let best_id = stack
            .iter()
            .enumerate()
            .max_by_key(|(_, h)| h.min_score)
            .unwrap()
            .0;
        let mut cur_path = stack.swap_remove(best_id);
        //println!("Cur: {:?}", cur_path);
        cur_path.update(&valves);
        if cur_path.time == end_time {
            if cur_path.score >= best_score {
                best_score = cur_path.score;
                _best_path = Some(cur_path);
            }
        } else {
            if cur_path.score + max_flow * (end_time - cur_path.time) <= best_score {
                useless_count += 1;
                continue;
            }
            // Do nothing
            //if let Some(new_path) = cur_path.open_valve(&valves) {
            //    stack.push(new_path);
            //}
            for path in cur_path.next(&valves).into_iter() {
                if all_seen.seen_better(&path) {
                    continue;
                }
                stack.push(path)
            }
        }
    }
    println!("Iterations: {}", iterations);
    println!("Useless skip: {}", useless_count);
    println!("Best score: {}", best_score);
    println!("Cache size: {}", all_seen.cache.len());
}
