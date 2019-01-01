use std::fs::File;
use std::io::Read;
use scan_fmt::*;
use std::collections::HashSet;
use std::collections::HashMap;

struct IdDistance(usize, i32);

impl std::cmp::PartialEq for IdDistance {
    fn eq(&self, other: &IdDistance) -> bool {
        self.1 == other.1
    }
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let coords = lines.map(|line| {
        let (x, y) = scan_fmt!(line, "{}, {}", i32, i32);
        (y.unwrap(), x.unwrap())
    });

    let h = 1 + coords.clone().map(|coord| coord.0).max().unwrap() as usize;
    let w = 1 + coords.clone().map(|coord| coord.1).max().unwrap() as usize;

    println!("{}x{}", h, w);

    let mut cells: Vec<Option<usize>> = Vec::with_capacity(w * h);

    for i in 0..h as i32 {
        for j in 0..w as i32 {
            let mut dups: HashMap<i32, (usize, i32)> = HashMap::new();
            coords
                .clone()
                .map(|(y,x)| (i-y).abs() + (j-x).abs())
                .enumerate()
                .for_each(|(id, distance)|
                    (*dups.entry(distance).or_insert((id, 0))).1 += 1
                );
            let id = dups
                .iter()
                .map(|(distance, (id, count))| (*distance, *id, *count))
                .min_by_key(|(distance, _, _)| *distance)
                .filter(|(_, _, count)| *count < 2)
                .map(|(_, id, _)| id);
            cells.push(id);
            //println!("{}x{}: {:?}", i, j, dups)
            //if let Some(id) = id {
                //print!("{}", id);
            //} else {
                //print!(".");
            //}
        }
        //println!("");
    }

    let mut infinite = HashSet::new();

    for i in 0..h {
        if let Some(id) = cells[i*w]         { infinite.insert(id); }
        if let Some(id) = cells[i*w + w-1]   { infinite.insert(id); }
    }

    for i in 0..w {
        if let Some(id) = cells[i]           { infinite.insert(id); }
        if let Some(id) = cells[i + (h-1)*w] { infinite.insert(id); }
    }

    println!("{:?}", infinite);

    let mut counts: HashMap<usize,i32> = HashMap::new();

    cells
        .into_iter()
        .filter(|cell| cell.is_some())
        .map(|cell| cell.unwrap())
        .filter(|id| !infinite.contains(id))
        .for_each(|id| *counts.entry(id).or_insert(0) += 1);

    println!("{:?}", counts);
    let largest_area = counts.values().max().unwrap();

    println!("{}", largest_area);
}
