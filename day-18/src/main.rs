use std::{collections::HashSet, env, fs};

struct World {
    cubes: HashSet<Vec<i32>>,
    external_air: HashSet<Vec<i32>>,
    min: Vec<i32>,
    max: Vec<i32>,
}
impl World {
    fn new() -> World {
        World {
            cubes: HashSet::new(),
            external_air: HashSet::new(),
            min: vec![i32::MAX, i32::MAX, i32::MAX],
            max: vec![i32::MIN, i32::MIN, i32::MIN],
        }
    }
    fn add_cube(&mut self, pos: &[i32]) {
        assert!(pos.len() == 3);
        self.cubes.insert(pos.to_vec());
        for i in 0..3 {
            self.max[i] = self.max[i].max(pos[i]);
            self.min[i] = self.min[i].min(pos[i]);
        }
    }
    fn build_external_air(&mut self) {
        assert!(self.external_air.len() == 0);
        self.external_air = self.get_connected_air(vec![0, 0, 0])
        //println!("BUILDING");
    }
    fn get_connected_air(&self, pos: Vec<i32>) -> HashSet<Vec<i32>> {
        let mut connected_air = HashSet::new();
        let mut to_check = vec![pos];
        while to_check.len() > 0 {
            let cur = to_check.pop().unwrap();
            //println!("CUR {:?}", cur);
            if self.cubes.contains(&cur) || connected_air.contains(&cur) {
                continue;
            }
            connected_air.insert(cur.to_vec());
            for dpos in [
                vec![-1, 0, 0],
                vec![1, 0, 0],
                vec![0, -1, 0],
                vec![0, 1, 0],
                vec![0, 0, -1],
                vec![0, 0, 1],
            ] {
                let new_pos = vec![cur[0] + dpos[0], cur[1] + dpos[1], cur[2] + dpos[2]];
                if (0..3).any(|i| new_pos[i] < self.min[i] - 1 || new_pos[i] > self.max[i] + 1) {
                    continue;
                }
                to_check.push(new_pos);
            }
        }
        connected_air
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
    for cube in fs::read_to_string(filename).unwrap().lines() {
        let coords = cube
            .split(',')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        world.add_cube(&coords)
    }
    println!("N cubes: {}", world.cubes.len());
    println!("Mins: {:?}", world.min);
    println!("Maxs: {:?}", world.max);
    world.build_external_air();

    let mut n_surface = 0;
    for cube in world.cubes.iter() {
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let side_coords = vec![cube[0] + dx, cube[1] + dy, cube[2] + dz];
            if !world.cubes.contains(&side_coords) {
                if part == "1" {
                    n_surface += 1;
                } else {
                    if world.external_air.contains(&side_coords) {
                        n_surface += 1;
                    }
                }
            }
        }
    }
    println!("N external surfaces: {}", n_surface);
}
