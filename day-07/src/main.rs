use std::fs::File;
use std::io::Read;
use scan_fmt::*;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let edges: Vec<(char,char)> = lines.map(|line| {
        let (src, dst) = scan_fmt!(
            line,
            "Step {} must be finished before step {} can begin.",
            char, char
        );
        (src.unwrap(), dst.unwrap())
    }).collect();

    let out_edges: HashMap<char,Vec<char>> = edges
        .iter()
        .fold(HashMap::new(), |mut acc, (src, dst)| {
            acc.entry(*dst).or_insert(Vec::new());
            acc.entry(*src).or_insert(Vec::new()).push(*dst);
            acc
        });

    let in_edges: HashMap<char,Vec<char>> = edges
        .iter()
        .fold(HashMap::new(), |mut acc, (src, dst)| {
            acc.entry(*src).or_insert(Vec::new());
            acc.entry(*dst).or_insert(Vec::new()).push(*src);
            acc
        });

    let mut fan_ins: HashMap<char, usize> = in_edges
        .iter()
        .map(|(dst, srcs)| (*dst, srcs.len()))
        .collect();

    let mut available: Vec<char> = fan_ins
        .iter()
        .filter(|(_, num_srcs)| **num_srcs == 0)
        .map(|(dst, _)| *dst)
        .collect();

    loop {
        available.sort();
        available.reverse();
        //println!("Available: {:?}", available);
        if let Some(src) = available.pop() {
            //println!("Picked: {}", src);
            print!("{}", src);
            out_edges
                .get(&src)
                .unwrap()
                .iter()
                .for_each(|dst| {
                    let fan_in = fan_ins.get_mut(dst).unwrap();
                    *fan_in -= 1;
                    if *fan_in == 0 {
                        available.push(*dst)
                    }
                });
        } else {
            break;
        };
    }
    println!("");

}
