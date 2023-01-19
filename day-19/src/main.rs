use std::{env, fs};

#[derive(Clone, Debug)]
struct Blueprint {
    id: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: (u32, u32),
    geode_robot: (u32, u32),
}
impl Blueprint {
    fn new(s: &str) -> Blueprint {
        let words = s.split_whitespace().collect::<Vec<_>>();
        let id = words[1]
            .trim_matches(|c: char| !c.is_numeric())
            .parse::<u32>()
            .unwrap();
        let ore_robot = words[6].parse::<u32>().unwrap();
        let clay_robot = words[12].parse::<u32>().unwrap();
        let obsidian_robot = (
            words[18].parse::<u32>().unwrap(),
            words[21].parse::<u32>().unwrap(),
        );
        let geode_robot = (
            words[27].parse::<u32>().unwrap(),
            words[30].parse::<u32>().unwrap(),
        );
        Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        }
    }
}

#[derive(Clone, Debug)]
struct Run {
    time: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
}
impl Run {
    fn new() -> Run {
        Run {
            time: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }
    fn next(&self, blueprint: &Blueprint) -> Vec<Run> {
        let mut next = vec![];
        //// Possible creation
        // None
        next.push(self.clone());
        // Create ore robot
        if self.ore >= blueprint.ore_robot {
            let mut run = self.clone();
            run.ore -= blueprint.ore_robot;
            run.ore_robot += 1;
            next.push(run);
        }
        // Create clay robot
        if self.ore >= blueprint.clay_robot {
            let mut run = self.clone();
            run.ore -= blueprint.clay_robot;
            run.clay_robot += 1;
            next.push(run);
        }
        // Create obsidiant robot
        if self.ore >= blueprint.obsidian_robot.0 && self.clay >= blueprint.obsidian_robot.1 {
            let mut run = self.clone();
            run.ore -= blueprint.obsidian_robot.0;
            run.clay -= blueprint.obsidian_robot.1;
            run.obsidian_robot += 1;
            next.push(run);
        }
        // Create geode robot
        if self.can_create_geode_robot(blueprint) {
            let mut run = self.clone();
            run.ore -= blueprint.geode_robot.0;
            run.obsidian -= blueprint.geode_robot.1;
            run.geode_robot += 1;
            next.push(run);
        }
        // advance time and produce ore based on start of this time configuration, so self.
        next.iter_mut().for_each(|mut run| {
            run.time += 1;
            run.ore += self.ore_robot;
            run.clay += self.clay_robot;
            run.obsidian += self.obsidian_robot;
            run.geode += self.geode_robot;
        });

        next
    }
    fn can_create_geode_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.geode_robot.0 && self.obsidian >= blueprint.geode_robot.1
    }
    // Return a maximum potential score for this score.
    // Guaranteeing it cannot be exceeded
    fn potential_max_score(&self, blueprint: &Blueprint, end_time: &u32) -> u32 {
        if self.can_create_geode_robot(blueprint) {
            self.geode
                + (0..(end_time - self.time))
                    .map(|i| self.geode_robot + i)
                    .sum::<u32>()
        } else {
            self.geode
                + self.geode_robot
                + (0..(end_time - 1 - self.time))
                    .map(|i| self.geode_robot + i)
                    .sum::<u32>()
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

    let blueprints = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| Blueprint::new(s))
        .collect::<Vec<_>>();

    println!("N blueprints: {}", blueprints.len());
    let end_time = if part == "1" { 24 } else { 32 };
    let mut quality_total = 0;
    let mut part_2_score = 1;
    for (i, blueprint) in blueprints.iter().enumerate() {
        if part == "2" && i >= 3 {
            break;
        }
        let mut runs = vec![Run::new()];
        let mut best = 0;
        while runs.len() > 0 {
            let run = runs.pop().unwrap();
            if run.time == end_time {
                if run.geode > best {
                    best = run.geode;
                    //println!("New best: {}", best);
                }
            } else {
                // Update best based on "do nothing" in this run
                best = best.max(run.geode + (end_time - run.time) * run.geode_robot);
                // Cut runs that can't beat current best in remaining time
                if run.potential_max_score(blueprint, &end_time) <= best {
                    continue;
                } else {
                    runs.extend(run.next(blueprint).into_iter());
                }
            }
        }
        println!("Blueprint {}: {}", blueprint.id, best);
        quality_total += blueprint.id * best;
        part_2_score *= best;
    }
    if part == "1" {
        println!("Total quality: {}", quality_total);
    } else {
        println!("First 3 blueprints multiplicative score: {}", part_2_score);
    }
}
