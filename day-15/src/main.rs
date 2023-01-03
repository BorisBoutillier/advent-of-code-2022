use std::{env, fs, ops::RangeInclusive};

#[derive(Debug)]
struct SensorData {
    s_x: i32,
    s_y: i32,
    b_x: i32,
    b_y: i32,
    dist: i32,
}
impl SensorData {
    fn new(s_x: i32, s_y: i32, b_x: i32, b_y: i32) -> SensorData {
        let dist = (s_x - b_x).abs() + (s_y - b_y).abs();
        SensorData {
            s_x,
            s_y,
            b_x,
            b_y,
            dist,
        }
    }
    fn not_present(&self, y: i32, beacon_as_present: bool) -> Option<RangeInclusive<i32>> {
        let y_dist = self.dist - (self.s_y - y).abs();
        if y_dist > 0 {
            let mut min_x = self.s_x - y_dist;
            if beacon_as_present && y == self.b_y && min_x == self.b_x {
                min_x += 1;
            }
            let mut max_x = self.s_x + y_dist;
            if beacon_as_present && y == self.b_y && max_x == self.b_x {
                max_x -= 1;
            }
            Some(min_x..=max_x)
        } else {
            None
        }
    }
}
fn trim_coords(c: char) -> bool {
    !(c == '-' || c.is_numeric())
}
fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!(
            "Expecting an input file path and a part number ( 1 or 2), found {:?}",
            args
        );
    }
    let is_example = args[1].contains("example");
    let part = args[2].clone();
    if part != "1" && part != "2" {
        panic!("part number must be 1 or 2, not {}", part)
    }

    let filename = args[1].clone();

    let mut sensors = vec![];
    for line in fs::read_to_string(filename).unwrap().lines() {
        let content = line.split_whitespace().collect::<Vec<_>>();
        let s_x = content[2].trim_matches(trim_coords).parse::<i32>().unwrap();
        let s_y = content[3].trim_matches(trim_coords).parse::<i32>().unwrap();
        let b_x = content[8].trim_matches(trim_coords).parse::<i32>().unwrap();
        let b_y = content[9].trim_matches(trim_coords).parse::<i32>().unwrap();
        sensors.push(SensorData::new(s_x, s_y, b_x, b_y));
    }

    if part == "1" {
        let res_y = if is_example { 10 } else { 2000000 };
        let (ranges, min_x, max_x) = not_present(&sensors, res_y, true);
        let mut count = 0u32;
        for x in min_x..=max_x {
            for range in ranges.iter() {
                if range.contains(&x) {
                    count += 1;
                    break;
                }
            }
        }
        println!("Ranges: {:?}", ranges);
        println!("Count: {}", count);
    } else {
        let max = if is_example { 20 } else { 4000000 };
        let mut possibilities = vec![];
        for y in 0..=max {
            let (ranges, _, _) = not_present(&sensors, y, false);
            //println!("Y: {}   => {:?}", y, ranges);
            for range in ranges.iter() {
                if range.start() > &0 && range.start() < &max {
                    possibilities.push((range.start() - 1, y));
                    break;
                }
                if range.end() > &0 && range.end() < &max {
                    possibilities.push((range.end() + 1, y));
                    break;
                }
            }
        }
        assert!(possibilities.len() == 1, "More than one possibility found");
        let tuning_frequency = possibilities[0].0 as u64 * 4000000 + possibilities[0].1 as u64;
        println!("Beacon tuning frequency: {}", tuning_frequency);
    }
}

fn not_present(
    sensors: &[SensorData],
    y: i32,
    beacons_as_present: bool,
) -> (Vec<RangeInclusive<i32>>, i32, i32) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut ranges = vec![];
    for sensor in sensors {
        if let Some(range) = sensor.not_present(y, beacons_as_present) {
            min_x = min_x.min(*range.start());
            max_x = max_x.max(*range.end());
            ranges.push(range);
        }
    }
    ranges.sort_by(|r0, r1| r0.start().cmp(r1.start()));
    loop {
        let mut new = None;
        let mut rem_idx = 0;
        for (i, range) in ranges.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if *(ranges[i - 1].end()) + 1 >= *range.start() {
                let start = ranges[i - 1].start();
                let end = range.end().max(ranges[i - 1].end());
                new = Some(*start..=*end);
                rem_idx = i;
                break;
            }
        }
        if let Some(range) = new {
            ranges[rem_idx - 1] = range;
            ranges.remove(rem_idx);
        } else {
            break;
        }
    }
    (ranges, min_x, max_x)
}
