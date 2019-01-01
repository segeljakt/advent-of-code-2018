#![allow(unused)]

use std::fs::File;
use std::io::Read;
use scan_fmt::*;

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut input = buf.lines();

    let initial_state = scan_fmt!(
        input.next().unwrap(),
        "initial state: {}",
        String
    ).unwrap().chars().collect::<Vec<char>>();

    let offset = 50;
    let mut state = Vec::new();
    (0..offset).for_each(|_| state.push('.') );
    state.extend(initial_state);
    (0..offset).for_each(|_| state.push('.') );

    input.next();

    let transitions = input.map(|line| {
        let (from, to) = scan_fmt!(line, "{} => {}", String, char);
        (from.unwrap().chars().collect(), to.unwrap())
    }).collect::<Vec<(Vec<char>, char)>>();

    transitions.iter().for_each(|(from,to)| {
        from.iter().for_each(|c| print!("{}", c));
        println!(" => {}", *to);
    });
    let mut new_state: Vec<char> = vec!['.'; state.len()];
    let len = state.len();
    print!("[{}]: ", 0);
    state.iter().for_each(|c| print!("{}", *c));
    println!("");
    for _ in 1000 {
        for _ in 1000 {
            for _ in 5000 {
                for (from, to) in transitions.iter() {
                    for x in 0..len-from.len()+1 {
                        let mut transform = true;
                        for (i, c) in from.iter().enumerate() {
                            if state[x+i] != *c {
                                transform = false;
                                break;
                            }
                        }
                        if transform {
                            new_state[x+2] = *to;
                        }
                    }
                }
                state = new_state.clone();
                print!("[{}]: ", iteration+1);
                new_state.iter().for_each(|c| print!("{}", *c));
                println!("");
                new_state = vec!['.'; state.len()];
            }
        }
    }
    println!("{}", state
             .iter()
             .enumerate()
             .filter(|(_, x)| **x == '#')
             .map(|(p, _)| { println!("{}", p as i32 - 20); p as i32 - offset })
             .sum::<i32>());
}
