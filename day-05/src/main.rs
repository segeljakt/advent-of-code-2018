use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let initial_chars: Vec<char> = buf.lines().next().unwrap().chars().collect();

    let mut min = std::i32::MAX;
    for (lower, upper) in ('a' as i32 ..'z' as i32).zip('A' as i32 ..'Z' as i32)  {
        let mut chars: Vec<char> = initial_chars
            .clone()
            .into_iter()
            .filter(|c| *c as i32 != lower && *c as i32 != upper)
            .collect();
        let mut new: Vec<char> = Vec::new();
        let mut changed = true;
        while changed {
            let mut iter = chars.into_iter();
            new.push(iter.next().unwrap());
            changed = false;
            while let Some(c) = iter.next() {
                if new.len() > 0 {
                    let last = new.last().unwrap();
                    if (*last as i32 - c as i32).abs() == 32 {
                        new.pop();
                        changed = true;
                    } else {
                        new.push(c);
                    }
                } else {
                    new.push(c);
                }
            }
            chars = new;
            new = Vec::new();
        }
        min = i32::min(chars.len() as i32, min);
    }
    println!("{}", min);

}
