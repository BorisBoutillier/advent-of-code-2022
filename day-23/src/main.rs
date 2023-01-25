use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

struct World {
    elves: HashSet<(i32, i32)>,
    moves: Vec<(Vec<(i32, i32)>, i32, i32)>,
}
impl World {
    fn new(filename: &str) -> World {
        let moves = vec![
            // Nothing
            (
                vec![
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ],
                0,
                0,
            ),
            // North
            (vec![(-1, -1), (0, -1), (1, -1)], 0, -1),
            // South
            (vec![(-1, 1), (0, 1), (1, 1)], 0, 1),
            // West
            (vec![(-1, -1), (-1, 0), (-1, 1)], -1, 0),
            // East
            (vec![(1, -1), (1, 0), (1, 1)], 1, 0),
        ];
        let mut elves = HashSet::new();
        for (y, line) in fs::read_to_string(&filename).unwrap().lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    elves.insert((x as i32, y as i32));
                }
            }
        }
        World { elves, moves }
    }
    fn round(&mut self) -> bool {
        // Plan moves
        let mut planned_moves = HashMap::new();
        let mut dests = HashMap::new();
        for &(elf_x, elf_y) in self.elves.iter() {
            let mut found = false;
            for (checks, dx, dy) in self.moves.iter() {
                if checks
                    .iter()
                    .all(|(dx, dy)| !self.elves.contains(&(elf_x + dx, elf_y + dy)))
                {
                    let dest = (elf_x + dx, elf_y + dy);
                    planned_moves.insert((elf_x, elf_y), dest);
                    dests.entry(dest).and_modify(|v| *v += 1).or_insert(1);
                    found = true;
                    break;
                }
            }
            if !found {
                let dest = (elf_x, elf_y);
                planned_moves.insert((elf_x, elf_y), dest);
                dests.entry(dest).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        //println!("Planned: {:?}", planned_moves);
        //println!("Dest: {:?}", dests);
        // Execute moves
        let mut one_moved = false;
        let mut new_elves = HashSet::new();
        for (&from, &dest) in planned_moves.iter() {
            if dests[&dest] == 1 {
                new_elves.insert(dest);
                if from != dest {
                    one_moved = true;
                }
            } else {
                new_elves.insert(from);
            }
        }
        self.elves = new_elves;
        // Update checks, Nothing always remains first
        let first = self.moves.remove(1);
        self.moves.push(first);
        one_moved
    }
    fn get_empty_ground_tiles(&self) -> i32 {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        for &(elf_x, elf_y) in self.elves.iter() {
            min_x = min_x.min(elf_x);
            min_y = min_y.min(elf_y);
            max_x = max_x.max(elf_x);
            max_y = max_y.max(elf_y);
        }
        (max_x - min_x + 1) * (max_y - min_y + 1) - self.elves.len() as i32
    }
    fn _sorted_elves(&self) -> Vec<(i32, i32)> {
        let mut elves = self.elves.iter().copied().collect::<Vec<_>>();
        elves.sort();
        elves
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
    let mut world = World::new(&filename);
    println!("Start with {} elves", world.elves.len());
    if part == "1" {
        //println!("  {:?}", world._sorted_elves());
        //println!("  = {}", world.get_empty_ground_tiles());
        for _i in 0..10 {
            world.round();
            //println!("Round {}, {} elves", i, world.elves.len());
            //println!("  {:?}", world._sorted_elves());
            //println!("  = {}", world.get_empty_ground_tiles());
        }
        println!("Score: {}", world.get_empty_ground_tiles());
    } else {
        let mut i = 0;
        loop {
            i += 1;
            if !world.round() {
                break;
            };
        }
        println!("First round without moves: {}", i);
    }
}
