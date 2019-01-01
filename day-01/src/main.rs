use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut changes = buf.lines().collect::<Vec<&str>>().into_iter().cycle();
    let mut found: HashMap<i32,()> = HashMap::new();
    let mut freq: i32 = 0;
    let dup = changes.find_map(|change| {
        freq += change.parse::<i32>().unwrap();
        match found.contains_key(&freq) {
            true => Some(freq),
            false => {
                found.insert(freq, ());
                None
            }
        }
    }).unwrap();
    println!("dup: {:?}", dup);
}
