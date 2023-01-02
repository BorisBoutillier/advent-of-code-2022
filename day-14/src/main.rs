use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Debug)]
struct World {
    blocks: HashMap<u32, HashSet<u32>>,
    abyss_y: u32,
    sand_count: u32,
}
impl World {
    fn new() -> World {
        World {
            blocks: HashMap::new(),
            abyss_y: u32::MIN,
            sand_count: 0,
        }
    }
    fn add_rockline(&mut self, x0: u32, y0: u32, x1: u32, y1: u32) {
        assert!(x0 == x1 || y0 == y1, "Not a line");
        self.abyss_y = self.abyss_y.max(y0 + 1).max(y1 + 1);
        if x0 == x1 {
            for y in (y0.min(y1))..=(y0.max(y1)) {
                self.blocks.entry(x0).or_insert_with(HashSet::new).insert(y);
            }
        } else {
            for x in (x0.min(x1))..=(x0.max(x1)) {
                self.blocks.entry(x).or_insert_with(HashSet::new).insert(y0);
            }
        }
    }
    fn add_floor(&mut self) {
        let x0 = self.blocks.keys().min().unwrap() - self.abyss_y;
        let x1 = self.blocks.keys().max().unwrap() + self.abyss_y;
        let y = self.abyss_y + 1; // Abyss is already max_y +1;
        self.add_rockline(x0, y, x1, y);
    }
    fn add_sand(&mut self, x: u32, y: u32) {
        //println!("New sand: ({},{}): ", x, y);
        self.blocks.entry(x).or_insert_with(HashSet::new).insert(y);

        self.sand_count += 1;
    }
    fn is_blocked(&self, x: u32, y: u32) -> bool {
        self.blocks.get(&x).map_or(false, |h| h.contains(&y))
    }
    pub fn drop_sand(&mut self) -> u32 {
        loop {
            // Drop a new sand on (500,0)
            let (mut x, mut y) = (500, 0);
            loop {
                if !self.is_blocked(x, y + 1) {
                    y += 1;
                    if y == self.abyss_y {
                        println!("Stopped on abyss");
                        return self.sand_count;
                    }
                } else if !self.is_blocked(x - 1, y + 1) {
                    x -= 1;
                    y += 1;
                } else if !self.is_blocked(x + 1, y + 1) {
                    x += 1;
                    y += 1;
                } else {
                    self.add_sand(x, y);
                    if (x, y) == (500, 0) {
                        println!("Stopped on initial");
                        return self.sand_count;
                    }
                    (x, y) = (500, 0);
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

    let mut world = World::new();
    for line in fs::read_to_string(filename).unwrap().lines() {
        let mut cur = None;
        for content in line.split("->") {
            let coords = content
                .trim()
                .split(',')
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            if cur.is_none() {
                cur = Some((coords[0], coords[1]));
            } else {
                let (x0, y0) = cur.unwrap();
                let (x1, y1) = (coords[0], coords[1]);
                world.add_rockline(x0, y0, x1, y1);
                cur = Some((x1, y1));
            }
            println!("Content {:?}", coords);
        }
    }
    if part == "2" {
        world.add_floor();
    }
    println!("World: {:?}", world);
    let sand_count = world.drop_sand();
    println!("Sand count: {}", sand_count);
}
