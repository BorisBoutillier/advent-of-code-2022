use std::{
    collections::HashSet,
    env,
    fs::{self},
    hash::Hash,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Wall,
    Floor,
}
#[derive(Clone, Debug)]
struct World {
    blocks: Vec<Vec<Tile>>,
    cube: bool,
    cube_size: i32,
    seen: HashSet<(String, Dir)>,
}
impl World {
    fn new(cube: bool) -> World {
        World {
            blocks: vec![],
            cube,
            cube_size: 0,
            seen: HashSet::new(),
        }
    }
    fn get_face_pos(&self, x: i32, y: i32) -> (String, i32, i32) {
        if self.cube_size == 50 {
            //  AABB
            //  AABB
            //  CC
            //  CC
            //DDEE
            //DDEE
            //FF
            //FF
            if y < self.cube_size {
                if x < 2 * self.cube_size {
                    (String::from("A"), x - self.cube_size, y)
                } else {
                    (String::from("B"), x - 2 * self.cube_size, y)
                }
            } else if y < 2 * self.cube_size {
                (String::from("C"), x - self.cube_size, y - self.cube_size)
            } else if y < 3 * self.cube_size {
                if x < self.cube_size {
                    (String::from("D"), x, y - 2 * self.cube_size)
                } else {
                    (
                        String::from("E"),
                        x - self.cube_size,
                        y - 2 * self.cube_size,
                    )
                }
            } else {
                (String::from("F"), x, y - 3 * self.cube_size)
            }
        } else {
            todo!()
        }
    }
    fn add_block_line(&mut self, line: &str) {
        let blocks = line
            .chars()
            .map(|c| match c {
                ' ' => Tile::Empty,
                '.' => Tile::Floor,
                '#' => Tile::Wall,
                _ => panic!(),
            })
            .collect::<Vec<_>>();
        self.blocks.push(blocks);
    }
    fn check_cube_size(&mut self) {
        let n_y = self.blocks.len() as i32;
        self.cube_size = if n_y % 3 == 0 { n_y / 3 } else { n_y / 4 };
        assert!(self.cube_size == 4 || self.cube_size == 50);
    }
    fn wrap(&self, x: i32, y: i32, dir: Dir) -> (i32, i32, Dir) {
        let (mut new_x, mut new_y, mut new_dir) = (x, y, dir);
        if self.cube {
            if self.cube_size == 4 {
                match (dir, x, y) {
                    (Dir::East, _, y) if y >= 0 && y < self.cube_size => {
                        assert_eq!(x, self.cube_size * 3 - 1);
                        new_x = self.cube_size * 4 - 1;
                        new_y = self.cube_size * 3 - 1 - y;
                        new_dir = Dir::West;
                    }
                    (Dir::East, _, y) if y >= self.cube_size && y < self.cube_size * 2 => {
                        assert_eq!(x, self.cube_size * 3 - 1);
                        new_x = self.cube_size * 4 - 1 - (y - self.cube_size);
                        new_y = self.cube_size * 2;
                        new_dir = Dir::South;
                    }
                    (Dir::East, _, y) => {
                        assert_eq!(x, self.cube_size * 4 - 1);
                        new_x = self.cube_size * 3 - 1;
                        new_y = self.cube_size * 3 - 1 - y;
                        new_dir = Dir::West;
                    }
                    (Dir::West, _, y) if y >= 0 && y < self.cube_size => {
                        assert_eq!(x, self.cube_size * 2);
                        new_x = self.cube_size + y;
                        new_y = self.cube_size;
                        new_dir = Dir::West;
                    }
                    (Dir::West, _, y) if y >= self.cube_size && y < self.cube_size * 2 => {
                        assert_eq!(x, 0);
                        new_x = self.cube_size * 4 - 1 - (y - self.cube_size);
                        new_y = self.cube_size * 3 - 1;
                        new_dir = Dir::North;
                    }
                    (Dir::West, _, y) => {
                        assert_eq!(x, self.cube_size * 2);
                        new_x = self.cube_size * 2 - 1 - (y - 2 * self.cube_size);
                        new_y = self.cube_size * 2 - 1;
                        new_dir = Dir::West;
                    }
                    (Dir::North, x, y) if y == 0 => {
                        assert!(x >= self.cube_size * 2 && x < self.cube_size * 3);
                        new_x = self.cube_size - 1 - (x - 2 * self.cube_size);
                        new_y = self.cube_size;
                        new_dir = Dir::South;
                    }
                    (Dir::North, x, y) if y == self.cube_size && x < self.cube_size => {
                        new_x = self.cube_size * 3 - 1 - x;
                        new_y = 0;
                        new_dir = Dir::South;
                    }
                    (Dir::North, x, y) if y == self.cube_size && x < 2 * self.cube_size => {
                        new_x = self.cube_size * 2;
                        new_y = x - self.cube_size;
                        new_dir = Dir::East;
                    }
                    (Dir::North, x, y) => {
                        assert!(y >= 2 * self.cube_size);
                        assert!(x >= 3 * self.cube_size && x < 4 * self.cube_size);
                        new_x = self.cube_size * 3 - 1;
                        new_y = 2 * self.cube_size - 1 - (x - self.cube_size * 3);
                        new_dir = Dir::East;
                    }
                    (Dir::South, x, y) if y == self.cube_size * 2 - 1 && x < self.cube_size => {
                        new_x = self.cube_size * 3 - 1 - x;
                        new_y = self.cube_size * 3 - 1;
                        new_dir = Dir::North;
                    }
                    (Dir::South, x, y) if y == self.cube_size * 2 - 1 && x < 2 * self.cube_size => {
                        new_x = self.cube_size * 2;
                        new_y = self.cube_size * 3 - 1 - (x - self.cube_size);
                        new_dir = Dir::East;
                    }
                    (Dir::South, x, y) if y == self.cube_size * 3 - 1 && x < 3 * self.cube_size => {
                        assert!(x >= 2 * self.cube_size);
                        new_x = self.cube_size - 1 - (x - self.cube_size * 2);
                        new_y = self.cube_size * 2 - 1;
                        new_dir = Dir::North;
                    }
                    (Dir::South, x, y) if y == self.cube_size * 3 - 1 => {
                        assert!(x >= 3 * self.cube_size);
                        new_x = 0;
                        new_y = self.cube_size * 2 - 1 - (x - self.cube_size * 3);
                        new_dir = Dir::East;
                    }
                    _ => panic!("{} {} {:?}", x, y, dir),
                }
            } else {
                match (dir, x, y) {
                    (Dir::East, x, y) if y < self.cube_size => {
                        assert_eq!(x, self.cube_size * 3 - 1);
                        new_x = self.cube_size * 2 - 1;
                        new_y = self.cube_size * 3 - 1 - y;
                        new_dir = Dir::West;
                    }
                    (Dir::East, x, y) if y < 2 * self.cube_size => {
                        assert_eq!(x, self.cube_size * 2 - 1);
                        new_x = self.cube_size * 2 + (y - self.cube_size);
                        new_y = self.cube_size - 1;
                        new_dir = Dir::North;
                    }
                    (Dir::East, x, y) if y < 3 * self.cube_size => {
                        assert_eq!(x, self.cube_size * 2 - 1);
                        new_x = self.cube_size * 3 - 1;
                        new_y = self.cube_size - 1 - (y - self.cube_size * 2);
                        new_dir = Dir::West;
                    }
                    (Dir::East, x, y) => {
                        assert_eq!(x, self.cube_size - 1);
                        new_x = self.cube_size + (y - self.cube_size * 3);
                        new_y = self.cube_size * 3 - 1;
                        new_dir = Dir::North;
                    }
                    (Dir::West, x, y) if y < self.cube_size => {
                        assert_eq!(x, self.cube_size);
                        new_x = 0;
                        new_y = self.cube_size * 3 - 1 - y;
                        new_dir = Dir::East;
                    }
                    (Dir::West, x, y) if y < 2 * self.cube_size => {
                        assert_eq!(x, self.cube_size);
                        new_x = y - self.cube_size;
                        new_y = self.cube_size * 2;
                        new_dir = Dir::South;
                    }
                    (Dir::West, x, y) if y < 3 * self.cube_size => {
                        assert_eq!(x, 0);
                        new_x = self.cube_size;
                        new_y = self.cube_size - 1 - (y - 2 * self.cube_size);
                        new_dir = Dir::East;
                    }
                    (Dir::West, x, y) => {
                        assert_eq!(x, 0);
                        new_x = self.cube_size + y - 3 * self.cube_size;
                        new_y = 0;
                        new_dir = Dir::South;
                    }
                    (Dir::North, x, y) if x < self.cube_size => {
                        assert_eq!(y, self.cube_size * 2);
                        new_x = self.cube_size;
                        new_y = self.cube_size + x;
                        new_dir = Dir::East;
                    }
                    (Dir::North, x, y) if x < 2 * self.cube_size => {
                        assert_eq!(y, 0);
                        new_x = 0;
                        new_y = self.cube_size * 3 + x - self.cube_size;
                        new_dir = Dir::East;
                    }
                    (Dir::North, x, y) => {
                        assert_eq!(y, 0);
                        new_x = x - 2 * self.cube_size;
                        new_y = self.cube_size * 4 - 1;
                        new_dir = Dir::North;
                    }
                    (Dir::South, x, y) if x < self.cube_size => {
                        assert_eq!(y, self.cube_size * 4 - 1);
                        new_x = x + 2 * self.cube_size;
                        new_y = 0;
                        new_dir = Dir::South;
                    }
                    (Dir::South, x, y) if x < 2 * self.cube_size => {
                        assert_eq!(y, self.cube_size * 3 - 1);
                        new_x = self.cube_size - 1;
                        new_y = self.cube_size * 3 + (x - self.cube_size);
                        new_dir = Dir::West;
                    }
                    (Dir::South, x, y) => {
                        assert_eq!(y, self.cube_size - 1);
                        new_x = 2 * self.cube_size - 1;
                        new_y = self.cube_size * 2 - 1 - (x - 2 * self.cube_size);
                        new_dir = Dir::West;
                    }
                }
            }
        } else {
            if dir == Dir::East || dir == Dir::West {
                //println!("Wrap from {} {} {:?} ", x, y, dir);
                new_x = self.get_first_x(y, dir);
                //println!("   -> {} {} ", new_x, new_y);
            } else {
                //println!("Wrap from {} {} {:?} ", x, y, dir);
                new_y = self.get_first_y(x, dir);
                //println!("   -> {} {} ", new_x, new_y);
            }
        }
        (new_x, new_y, new_dir)
    }
    fn get_first_x(&self, y: i32, dir: Dir) -> i32 {
        let y = y as usize;
        match dir {
            Dir::East => {
                self.blocks[y]
                    .iter()
                    .enumerate()
                    .find(|(_, &t)| t != Tile::Empty)
                    .unwrap()
                    .0 as i32
            }
            Dir::West => {
                self.blocks[y]
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, &t)| t != Tile::Empty)
                    .unwrap()
                    .0 as i32
            }
            _ => panic!(),
        }
    }
    fn get_first_y(&self, x: i32, dir: Dir) -> i32 {
        let x = x as usize;
        match dir {
            Dir::South => {
                self.blocks
                    .iter()
                    .enumerate()
                    .find(|(_, l)| x < l.len() && l[x] != Tile::Empty)
                    .unwrap()
                    .0 as i32
            }
            Dir::North => {
                self.blocks
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, l)| x < l.len() && l[x] != Tile::Empty)
                    .unwrap()
                    .0 as i32
            }
            _ => panic!(),
        }
    }
    fn forward(&mut self, x: i32, y: i32, dir: Dir) -> (i32, i32, Dir) {
        let (mut new_x, mut new_y) = match dir {
            Dir::East => (x + 1, y),
            Dir::West => (x - 1, y),
            Dir::South => (x, y + 1),
            Dir::North => (x, y - 1),
        };
        let mut new_dir = dir;
        let mut tile = if new_x < 0
            || new_y < 0
            || new_y as usize >= self.blocks.len()
            || new_x as usize >= self.blocks[new_y as usize].len()
        {
            Tile::Empty
        } else {
            self.blocks[new_y as usize][new_x as usize]
        };
        if tile == Tile::Empty {
            (new_x, new_y, new_dir) = self.wrap(x, y, dir);
            if self.cube_size == 50 {
                let face_pos = self.get_face_pos(x, y);
                let face = face_pos.0.clone();
                if !self.seen.contains(&(face, dir)) {
                    self.seen.insert((face_pos.0.clone(), dir));
                    println!(
                        "WRAP {},{} {:?} {:?} -> {},{} {:?} {:?} ",
                        x,
                        y,
                        face_pos,
                        dir,
                        new_x,
                        new_y,
                        self.get_face_pos(new_x, new_y),
                        new_dir,
                    );
                }
            }
            tile = self.blocks[new_y as usize][new_x as usize];
        }
        match tile {
            Tile::Floor => (new_x, new_y, new_dir),
            Tile::Wall => (x, y, dir),
            Tile::Empty => panic!(
                "From {} {} {:?} -> {} ,{} with {:?}",
                x, y, dir, new_x, new_y, tile
            ),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Dir {
    West,
    East,
    South,
    North,
}
impl Dir {
    fn apply(&self, action: Action) -> Dir {
        match action {
            Action::Forward(_) => *self,
            Action::CW => match self {
                Dir::East => Dir::South,
                Dir::South => Dir::West,
                Dir::West => Dir::North,
                Dir::North => Dir::East,
            },
            Action::CCW => match self {
                Dir::East => Dir::North,
                Dir::North => Dir::West,
                Dir::West => Dir::South,
                Dir::South => Dir::East,
            },
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Action {
    Forward(u32),
    CW,
    CCW,
}
impl Action {
    fn from_string(line: &str) -> Vec<Action> {
        let mut actions = vec![];
        let mut num = String::from("");
        for c in line.chars() {
            if c.is_numeric() {
                num += &c.to_string();
            } else {
                if !num.is_empty() {
                    let count = num.parse::<u32>().unwrap();
                    actions.push(Action::Forward(count));
                    num = String::from("");
                }
                match c {
                    'R' => actions.push(Action::CW),
                    'L' => actions.push(Action::CCW),
                    _ => panic!(),
                }
            }
        }
        if !num.is_empty() {
            let count = num.parse::<u32>().unwrap();
            actions.push(Action::Forward(count));
        }
        actions
    }
}
#[derive(Clone, Debug)]
struct Player {
    y: i32,
    x: i32,
    facing: Dir,
    actions: Vec<Action>,
}
impl Player {
    fn new(actions: &[Action], world: &World) -> Player {
        let x = world.blocks[0]
            .iter()
            .enumerate()
            .find(|(_, &t)| t == Tile::Floor)
            .unwrap()
            .0;
        Player {
            y: 0,
            x: x as i32,
            facing: Dir::East,
            actions: actions.to_vec(),
        }
    }
    fn advance(&mut self, world: &mut World) -> bool {
        if self.actions.is_empty() {
            return false;
        }
        let action = self.actions.remove(0);
        if let Action::Forward(x) = action {
            for _ in 0..x {
                (self.x, self.y, self.facing) = world.forward(self.x, self.y, self.facing);
            }
        } else {
            self.facing = self.facing.apply(action);
        }
        true
    }
    fn password(&self) -> i32 {
        1000 * (self.y + 1)
            + 4 * (self.x + 1)
            + match self.facing {
                Dir::East => 0,
                Dir::South => 1,
                Dir::West => 2,
                Dir::North => 3,
            }
    }
    fn _actions_to_string(&self) -> String {
        let mut s = String::from("");
        for action in self.actions.iter() {
            let ss = match action {
                Action::Forward(f) => f.to_string(),
                Action::CW => String::from("R"),
                Action::CCW => String::from("L"),
            };
            s += &ss;
        }
        s
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

    let mut read_world = true;
    let mut world = World::new(part == "2");
    let mut actions = vec![];
    for line in fs::read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            assert!(read_world);
            read_world = false;
        } else if read_world {
            world.add_block_line(line);
        } else {
            actions = Action::from_string(line);
        }
    }
    world.check_cube_size();
    println!("World Cube:{} {}", world.cube, world.cube_size);
    let mut player = Player::new(&actions, &world);
    //println!("{}", player.actions_to_string());
    //println!("Start -> {:?}", player);
    loop {
        if !player.advance(&mut world) {
            break;
        } else {
            //println!("  {:?}", player);
        }
    }
    println!("End -> {} {} {:?}", player.x, player.y, player.facing);
    println!("Password: {}", player.password());
}
