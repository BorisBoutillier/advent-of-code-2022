use num::Integer;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

struct World {
    width: i32,
    height: i32,
    winds: HashMap<i32, Vec<(i32, i32, Dir)>>,
    start_x: i32,
    end_x: i32,
    blocked: HashMap<i32, HashSet<(i32, i32)>>,
    wrap_time: i32,
}
impl World {
    fn new(filename: &str) -> World {
        let mut width = 0;
        let mut height = 0;
        let mut start_x = -1;
        let mut end_x = -1;
        let mut start_winds = vec![];

        for (y, line) in fs::read_to_string(filename).unwrap().lines().enumerate() {
            if y == 0 {
                width = line.len() as i32 - 2;
                start_x = line.chars().enumerate().find(|(_, c)| *c == '.').unwrap().0 as i32 - 1;
            } else if line.chars().filter(|c| *c == '#').count() > 2 {
                height = y as i32 - 1;
                end_x = line.chars().enumerate().find(|(_, c)| *c == '.').unwrap().0 as i32 - 1;
            } else {
                line.chars().enumerate().for_each(|(x, c)| match c {
                    '<' => {
                        start_winds.push((x as i32 - 1, y as i32 - 1, Dir::West));
                    }
                    '>' => {
                        start_winds.push((x as i32 - 1, y as i32 - 1, Dir::East));
                    }
                    '^' => {
                        start_winds.push((x as i32 - 1, y as i32 - 1, Dir::North));
                    }
                    'v' => {
                        start_winds.push((x as i32 - 1, y as i32 - 1, Dir::South));
                    }
                    _ => {}
                })
            }
        }
        let mut winds = HashMap::new();
        let mut blocked = HashMap::new();
        blocked.insert(
            0,
            start_winds
                .iter()
                .copied()
                .map(|(x, y, _)| (x, y))
                .collect::<HashSet<_>>(),
        );
        winds.insert(0, start_winds);
        let wrap_time = width.lcm(&height);
        let mut world = World {
            width,
            height,
            winds,
            start_x,
            end_x,
            blocked,
            wrap_time,
        };
        for i in 1..wrap_time {
            world.compute_winds(i);
        }
        world
    }
    fn compute_winds(&mut self, time: i32) {
        assert!(self.winds.contains_key(&(time - 1)) && !self.winds.contains_key(&time));
        assert!(self.blocked.contains_key(&(time - 1)) && !self.blocked.contains_key(&time));
        let mut next_winds = vec![];
        let mut next_blocked = HashSet::new();
        for &(x, y, dir) in self.winds[&(time - 1)].iter() {
            let (new_x, new_y) = match dir {
                Dir::West => ((x - 1).rem_euclid(self.width), y),
                Dir::East => ((x + 1).rem_euclid(self.width), y),
                Dir::North => (x, (y - 1).rem_euclid(self.height)),
                Dir::South => (x, (y + 1).rem_euclid(self.height)),
            };
            next_winds.push((new_x, new_y, dir));
            next_blocked.insert((new_x, new_y));
        }
        //println!("COMPUTE time {}. Winds: {:?}", time, next_winds);
        //println!("                 Blocked: {:?}", next_blocked);
        self.winds.insert(time, next_winds);
        self.blocked.insert(time, next_blocked);
    }
    fn is_blocked(&self, time: i32, x: i32, y: i32) -> bool {
        let ok = if y == self.height {
            x != self.end_x
        } else if y == -1 {
            x != self.start_x
        } else if x < 0 || x >= self.width || y < 0 || y > self.height {
            true
        } else {
            let wrapped_time = time % self.wrap_time;
            self.blocked[&wrapped_time].contains(&(x, y))
        };
        //println!("Is blocked at {} {},{} -> {}", time, x, y, ok);
        ok
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
    let world = World::new(&filename);
    println!(
        "World:  {}x{} Start:{}  End:{}, Wrap: {}",
        world.width, world.height, world.start_x, world.end_x, world.wrap_time
    );
    let n_iter = if part == "1" { 1 } else { 3 };
    let mut total_time = 0;
    for i in 0..n_iter {
        let (start_x, start_y) = if i.is_even() {
            (world.start_x, -1)
        } else {
            (world.end_x, world.height)
        };
        let (end_x, end_y) = if i.is_even() {
            (world.end_x, world.height)
        } else {
            (world.start_x, -1)
        };
        // Last entries will be first tested
        // prioritize going toward current end
        // This order has huge impact on number of iteration as
        // this faster creates better best, thus further cutting more runs.
        let nexts = if i.is_even() {
            [(0, 0), (-1, 0), (0, -1), (1, 0), (0, 1)]
        } else {
            [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)]
        };
        let start_time = total_time;
        let mut stack = vec![(start_time, start_x, start_y)];
        let mut best = i32::MAX;
        let mut count = 0;
        let mut seen = HashMap::new();
        println!(
            "LOOP {} Start ({},{}) End ({},{}) Start Time {}",
            i, start_x, start_y, end_x, end_y, start_time
        );
        while !stack.is_empty() {
            //if i == 1 {
            //    println!("Stack: {:?}", stack);
            //}
            let (cur_time, cur_x, cur_y) = stack.pop().unwrap();
            let cur_wrap_time = cur_time % world.wrap_time;
            if seen
                .get(&(cur_wrap_time, cur_x, cur_y))
                .unwrap_or(&i32::MAX)
                <= &cur_time
            {
                //println!("  SKIP 1 ; {} {} {}", cur_wrap_time, cur_x, cur_y);
                continue;
            }
            seen.insert((cur_wrap_time, cur_x, cur_y), cur_time);
            if cur_time + (cur_x - end_x).abs() + (cur_y - end_y).abs() >= best {
                //println!("  SKIP 2");
                continue;
            }
            for &(dx, dy) in &nexts {
                let new_x = cur_x + dx;
                let new_y = cur_y + dy;
                //println!("  CHECK {},{},{}", cur_time + 1, new_x, new_y);
                if new_y == end_y && new_x == end_x {
                    best = best.min(cur_time + 1);
                    //println!("Best improved to {}", best);
                } else if !world.is_blocked(cur_time + 1, new_x, new_y) {
                    stack.push((cur_time + 1, new_x, new_y));
                }
            }
            count += 1;
            //if count == 5 {
            //    break;
            //}
        }
        println!("Best {} in {} iterations", best, count);
        total_time = best;
    }
    println!("Total: {}", total_time);
}
