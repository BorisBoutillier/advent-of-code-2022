use std::{env, fs, num::NonZeroUsize};

use lru::LruCache;
#[derive(Clone)]
struct Rock {
    rows: Vec<u8>,
}
impl Rock {
    fn new(rows: Vec<u8>) -> Rock {
        Rock { rows }
    }
    fn new_with_step(rock: &Rock, step: u8) -> Rock {
        Rock {
            rows: rock.rows.iter().map(|row| row << step).collect::<Vec<_>>(),
        }
    }
    fn print(&self) {
        for row in self.rows.iter().rev() {
            let s = (0..World::WIDTH)
                .map(|i| if (row >> i) % 2 == 0 { '.' } else { '#' })
                .collect::<String>();
            println!("{}", s);
        }
    }
    fn with_wind(&mut self, wind: i8) -> Rock {
        match wind {
            1 => {
                if self.rows.iter().all(|row| (row >> (World::WIDTH - 1)) == 0) {
                    Rock {
                        rows: self.rows.iter().map(|row| row << 1).collect::<Vec<_>>(),
                    }
                } else {
                    self.clone()
                }
            }
            _ => {
                if self.rows.iter().all(|row| row % 2 == 0) {
                    Rock {
                        rows: self.rows.iter().map(|row| row >> 1).collect::<Vec<_>>(),
                    }
                } else {
                    self.clone()
                }
            }
        }
    }
}
struct World {
    winds: Vec<i8>,
    wind_id: usize,
    rocks: Vec<Rock>,
    base_height: i64,
    rock_id: usize,
    rock_nb: u32,
    // true if filled, false if empty
    state: Vec<u8>,
    cache: LruCache<(usize, usize, Vec<u8>), (u64, i64)>,
}

impl World {
    const WIDTH: i32 = 7;
    fn new(winds: &[i8]) -> World {
        let rocks = vec![
            Rock::new(vec![15]),
            Rock::new(vec![2, 7, 2]),
            Rock::new(vec![7, 4, 4]),
            Rock::new(vec![1, 1, 1, 1]),
            Rock::new(vec![3, 3]),
        ];

        World {
            winds: winds.to_vec(),
            wind_id: 0,
            rocks,
            rock_id: 0,
            rock_nb: 0,
            base_height: 0,
            state: vec![],
            cache: LruCache::new(NonZeroUsize::new(5_000_000).unwrap()),
        }
    }
    fn print(&self) {
        for row in self.state.iter().rev() {
            println!(
                "|{}|",
                (0..World::WIDTH)
                    .map(|i| if (row >> i) % 2 == 0 { '.' } else { '#' })
                    .collect::<String>()
            );
        }
        println!(
            "+{}+",
            vec!['-'; World::WIDTH as usize].iter().collect::<String>()
        );
        println!("+ {} ", self.base_height);
        println!();
        println!("Dropped: {}", self.rock_nb);
    }
    fn height(&self) -> i64 {
        self.base_height + self.state.len() as i64
    }
    fn set_height(&mut self, height: i64) {
        assert!(height >= 0);
        for _ in 0..(height - self.height()) {
            self.state.push(0u8);
        }
    }
    fn collides(&self, y: i64, row: &u8) -> bool {
        y < self.base_height
            || (y < self.height() && (self.state[(y - self.base_height) as usize] & row) != 0)
    }
    fn simplify(&mut self, iteration_idx: u64) -> Option<(u64, i64)> {
        let mut scan = 0u8;
        let full = (1u8 << World::WIDTH) - 1;
        let mut i = self.state.len() - 1;
        loop {
            scan |= self.state[i];
            if scan == full || i == 0 {
                break;
            }
            i -= 1;
        }
        if scan == full {
            self.state = self.state[i..self.state.len()].to_vec();
            self.base_height += i as i64;
        }
        let entry = (self.wind_id, self.rock_id, self.state.clone());
        if self.cache.contains(&entry) {
            self.cache.get(&entry).copied()
        } else {
            self.cache.put(entry, (iteration_idx, self.base_height));
            None
        }
    }
    fn drop_one(&mut self, iteration_idx: u64, debug: bool) -> Option<(u64, i64)> {
        if debug {
            println!("Rock choice");
            self.rocks[self.rock_id].print();
        }
        let mut rock = Rock::new_with_step(&self.rocks[self.rock_id], 2);
        let mut rock_base = self.height() + 3;
        self.rock_id = (self.rock_id + 1) % self.rocks.len();
        loop {
            if debug {
                println!("Rock base: {}", rock_base);
                rock.print();
            }
            // Apply winds
            let wind = self.winds[self.wind_id];
            self.wind_id = (self.wind_id + 1) % self.winds.len();
            let new_rock = rock.with_wind(wind);
            let mut ok = true;
            for (i, rock_row) in new_rock.rows.iter().enumerate() {
                if self.collides(i as i64 + rock_base, rock_row) {
                    ok = false;
                    break;
                }
            }
            if ok {
                rock = new_rock;
            }
            if debug {
                println!("Wind:{:2}", wind);
                println!("Rock base: {}", rock_base);
                rock.print();
            }
            // Apply fall
            let mut ok = true;
            for (i, rock_row) in rock.rows.iter().enumerate() {
                if self.collides(i as i64 + rock_base - 1, rock_row) {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
            rock_base -= 1;
        }
        if debug {
            println!("Rock end. base: {}", rock_base);
            rock.print();
        }
        self.set_height(rock_base + rock.rows.len() as i64);
        rock.rows.iter().enumerate().for_each(|(i, row)| {
            let y = i + (rock_base - self.base_height) as usize;
            assert!(
                (self.state[y] & row) == 0,
                "Missed: {} vs {}",
                self.state[y],
                row
            );
            self.state[y] |= row;
        });

        self.rock_nb += 1;
        self.simplify(iteration_idx)
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

    let wind = fs::read_to_string(filename)
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let mut world = World::new(&wind);

    let n_iteration = if part == "1" {
        2022
    } else {
        1_000_000_000_000u64
    };
    let mut i = 0;
    let mut jumped = false;
    while i < n_iteration {
        if i % 1_000_000 == 0 {
            println!("I: {}M Height: {}", i / 1_000_000, world.height());
        }
        match world.drop_one(i, false) {
            None => {
                i += 1;
            }
            Some((prev_i, prev_base_height)) => {
                if !jumped {
                    let i_step = i - prev_i;
                    let height_step = world.base_height - prev_base_height;
                    let n_loop = (n_iteration - (i + 1)) / i_step;
                    println!(
                        "At I {} found {} loops of step {} -> {}",
                        i,
                        n_loop,
                        i_step,
                        i + i_step * n_loop + 1
                    );
                    world.base_height += height_step * n_loop as i64;
                    i += i_step * n_loop + 1;
                    jumped = true;
                } else {
                    i += 1;
                }
            }
        }
    }
    println!("Height: {}", world.height());
    println!("Base_height: {}", world.base_height);

    //world.print();
    //world.drop_one(true);
    //world.print();
    //world.drop_one(true);
    //world.print();
    //world.drop_one(true);
    //world.print();
}
// Example 2 10M : 28s -> 15142861
// Example 2 100M release mode: 12s -> 151428577
// Input 2 10M : 53s -> 15371762
// Example 1 3068
// Input 1 3106
