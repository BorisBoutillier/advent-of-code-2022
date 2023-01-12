use std::{env, fs};
struct World {
    winds: Vec<i32>,
    wind_id: usize,
    rocks: Vec<Vec<(i32, i32)>>,
    rock_id: usize,
    rock_nb: u32,
    // true if filled, false if empty
    state: Vec<Vec<bool>>,
}

impl World {
    const WIDTH: i32 = 7;
    fn new(winds: &[i32]) -> World {
        let rocks: Vec<Vec<(i32, i32)>> = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        ];

        World {
            winds: winds.to_vec(),
            wind_id: 0,
            rocks,
            rock_id: 0,
            rock_nb: 0,
            state: vec![],
        }
    }
    fn print(&self) {
        for row in self.state.iter().rev() {
            println!(
                "|{}|",
                row.iter()
                    .map(|&c| if c { '#' } else { '.' })
                    .collect::<String>()
            );
        }
        println!(
            "+{}+",
            vec!['-'; World::WIDTH as usize].iter().collect::<String>()
        );
        println!();
        println!("Dropped: {}", self.rock_nb);
    }
    fn height(&self) -> i32 {
        self.state.len() as i32
    }
    fn set_height(&mut self, height: i32) {
        assert!(height >= 0);
        for _ in 0..(height - self.height()) {
            self.state.push(vec![false; World::WIDTH as usize]);
        }
    }
    fn is_empty(&self, &(x, y): &(i32, i32)) -> bool {
        //println!(
        //    "Check {},{} {} {} {}",
        //    x,
        //    y,
        //    (0..World::WIDTH).contains(&x),
        //    y >= 0,
        //    (y >= self.height() || !self.state[y as usize][x as usize])
        //);
        (0..World::WIDTH).contains(&x)
            && y >= 0
            && (y >= self.height() || !self.state[y as usize][x as usize])
    }
    fn drop_one(&mut self, debug: bool) {
        let height = self.height();
        let mut rock = self.rocks[self.rock_id]
            .iter()
            .map(|(x, y)| (x + 2, y + height + 3))
            .collect::<Vec<_>>();
        self.rock_id = (self.rock_id + 1) % self.rocks.len();
        if debug {
            println!("Rock start");
        }
        loop {
            if debug {
                println!("Rock: {:?}", rock);
            }
            // Apply winds
            let wind = self.winds[self.wind_id];
            self.wind_id = (self.wind_id + 1) % self.winds.len();
            let new_rock = rock.iter().map(|&(x, y)| (x + wind, y)).collect::<Vec<_>>();
            if new_rock.iter().all(|pos| self.is_empty(pos)) {
                rock = new_rock;
            }
            if debug {
                println!("Wind:{:2} {:?}", wind, rock);
            }
            // Apply fall
            let new_rock = rock.iter().map(|&(x, y)| (x, y - 1)).collect::<Vec<_>>();
            if !(new_rock.iter().all(|pos| self.is_empty(pos))) {
                break;
            }
            rock = new_rock;
        }
        if debug {
            println!("Rock end: {:?}", rock);
        }
        self.set_height(rock.iter().map(|&(_, y)| y + 1).max().unwrap());
        rock.iter().for_each(|&(x, y)| {
            let x = x as usize;
            let y = y as usize;
            assert!(!self.state[y][x]);
            self.state[y][x] = true;
        });

        self.rock_nb += 1;
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

    for _ in 0..2022 {
        world.drop_one(false);
    }
    println!("Height: {}", world.height());
    //world.print();
    //world.drop_one(true);
    //world.print();
    //world.drop_one(true);
    //world.print();
}
