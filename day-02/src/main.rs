use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let ids: std::str::Lines<'_> = buf.lines();

    let (twice, thrice) = ids.clone().fold((0, 0), |(twice, thrice), id| {
        let mut found: HashMap<char, i32> = HashMap::new();
        for letter in id.chars() {
            match found.get(&letter) {
                Some(count) => found.insert(letter, count+1),
                None => found.insert(letter, 1),
            };
        };
        let twice = match found.values().filter(|count| **count == 2).next() {
            Some(_) => twice + 1,
            None => twice,
        };
        let thrice = match found.values().filter(|count| **count == 3).next() {
            Some(_) => thrice + 1,
            None => thrice,
        };
        (twice, thrice)
    });

    println!("{} = {} * {}", twice*thrice, twice, thrice);

    let ids: Vec<&str> = ids.collect();
    for y in 0..ids.len() {
        for x in y+1..ids.len() {
            let mut diff = 0;
            for (cx, cy) in ids[y].chars().zip(ids[x].chars()) {
                if cy != cx {
                    diff += 1
                }
            };
            if diff == 1 {
                let mut same: String = String::new();
                for (cy, cx) in ids[y].chars().zip(ids[x].chars()) {
                    if cx == cy {
                        same.push(cy);
                    }
                };
                println!("{}", same);
            }
        }
    }


}
