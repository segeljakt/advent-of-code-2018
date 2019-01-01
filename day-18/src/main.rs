use std::io::Read;
use std::fs::File;
//use std::time;
//use std::thread;
use std::collections::HashMap;

const W: usize = 50;
const H: usize = 50;

#[derive(Copy,Clone,PartialEq,Eq)]
enum Acre {
    OpenGround, // => Trees
    Trees, // => Lumberyard
    Lumberyard, // => OpenGround
    OutOfBounds,
}

fn score(acres: &[[Acre; W+2]; H+2]) -> usize {
    let num_trees = acres
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|&&acre| acre == Acre::Trees)
        .count();
    let num_lumbrs = acres
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|&&acre| acre == Acre::Lumberyard)
        .count();
    num_trees*num_lumbrs
}

fn main() {

    let mut acres = parse();

    let mut scores: Vec<usize> = Vec::new();

    //println!("Initial state");
    //print(&acres);
    for minute in 1..=500 {
        let mut new_acres = acres.clone();
        for y in 1..H+1 {
            for x in 1..W+1 {
                let adj = [
                    acres[y-1][x-1],
                    acres[y-1][x  ],
                    acres[y-1][x+1],
                    acres[y  ][x-1],
                    acres[y  ][x+1],
                    acres[y+1][x-1],
                    acres[y+1][x  ],
                    acres[y+1][x+1],
                ];
                let trees = adj
                    .iter()
                    .filter(|&&acre| acre == Acre::Trees)
                    .count();
                let lumbr = adj
                    .iter()
                    .filter(|&&acre| acre == Acre::Lumberyard)
                    .count();

                match acres[y][x] {
                    Acre::OpenGround => {
                        if trees >= 3 {
                            new_acres[y][x] = Acre::Trees;
                        }
                    }
                    Acre::Trees => {
                        if lumbr >= 3 {
                            new_acres[y][x] = Acre::Lumberyard;
                        }
                    }
                    Acre::Lumberyard => {
                        if trees == 0 || lumbr == 0 {
                            new_acres[y][x] = Acre::OpenGround;
                        }
                    }
                    Acre::OutOfBounds => unreachable!()
                }

            }
        }
        acres = new_acres;
        //println!("After {} minutes..", minute);
        //print(&acres);
        scores.push(score(&acres));
    }
    'outer: for i in 1..scores.len() {
        for j in i+2..scores.len() {
            if scores[i] == scores[j] && scores[i-1] == scores[j-1] {
                println!("{} == {} && {} == {}", scores[i], scores[j], scores[i-1], scores[j-1]);
                println!("period is {} = {} - {}", j-i, j, i);
                let period = j-i;
                let start = i;
                let offset = (1_000_000_000 - start) % period;
                let result = scores[i+offset-1];
                println!("{} = (1_000_000_000 - {}) % {}", offset, start, period);
                println!("{} = scores[{}+{}-1]", result, start, offset);
                break 'outer;
            }
        }
    }



}

fn print(acres: &[[Acre; W+2]; H+2]) {
    //thread::sleep(time::Duration::from_millis(1000));
    for row in acres.iter() {
        for acre in row.iter() {
            match acre {
                Acre::OpenGround  => print!("."),
                Acre::Trees       => print!("|"),
                Acre::Lumberyard  => print!("#"),
                Acre::OutOfBounds => print!(" "),
            }
        }
        println!("");
    }
}

fn parse() -> [[Acre; W+2]; H+2] {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut acres = [[Acre::OutOfBounds; W+2]; H+2];

    for (y, line) in buf.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            acres[y+1][x+1] = match ch {
                '.' => Acre::OpenGround,
                '|' => Acre::Trees,
                '#' => Acre::Lumberyard,
                _ => unreachable!(),
            };
        }
    }
    acres
}
