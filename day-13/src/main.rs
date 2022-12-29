use std::{cmp::Ordering, env, fs};

use json::JsonValue;

#[derive(Debug, Clone)]
enum Entry {
    Value(u32),
    List(Vec<Entry>),
}
impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl Eq for Entry {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Entry::Value(u), Entry::Value(v)) => u.partial_cmp(v),
            (Entry::Value(_), Entry::List(v)) => vec![self.clone()].partial_cmp(v),
            (Entry::List(u), Entry::Value(_)) => u.partial_cmp(&vec![other.clone()]),
            (Entry::List(u), Entry::List(v)) => u.partial_cmp(v),
        }
    }
}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Entry {
    fn from_json(js: &JsonValue) -> Entry {
        match js {
            JsonValue::Null => panic!(),
            JsonValue::Short(_) => panic!(),
            JsonValue::String(_) => panic!(),
            JsonValue::Number(_) => Entry::Value(js.as_u32().unwrap()),
            JsonValue::Boolean(_) => panic!(),
            JsonValue::Object(_) => panic!(),
            JsonValue::Array(v) => Entry::List(v.iter().map(Entry::from_json).collect::<Vec<_>>()),
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

    let mut entries = vec![];
    for line in fs::read_to_string(filename).unwrap().lines() {
        if !line.is_empty() {
            entries.push(Entry::from_json(&json::parse(line).unwrap()));
        }
    }

    if part == "1" {
        let mut count = 0;
        for (i, e) in entries.chunks(2).enumerate() {
            //println!("Compare:");
            //println!("   {:?}", e[0]);
            //println!("   {:?}", e[1]);
            if e[0] < e[1] {
                //println!("     = right order");
                count += i + 1;
            }
        }
        println!("Count: {}", count);
    } else {
        let code0 = Entry::List(vec![Entry::Value(2)]);
        let code1 = Entry::List(vec![Entry::Value(6)]);
        entries.push(code0.clone());
        entries.push(code1.clone());
        entries.sort();
        let i0 = entries.iter().position(|c| c == &code0).unwrap() + 1;
        let i1 = entries.iter().position(|c| c == &code1).unwrap() + 1;
        println!("First: {:?}", entries.first());
        println!("Last: {:?}", entries.last());
        println!("I0: {}", i0);
        println!("I1: {}", i1);
        println!("Result {}", i0 * i1);
    }
}
