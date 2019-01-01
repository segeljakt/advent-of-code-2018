use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use scan_fmt::*;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let lines: std::str::Lines<'_> = buf.lines();
    let mut grid: HashMap<(i32,i32), Vec<i32>> = HashMap::new();
    let mut overlaps: HashMap<i32, bool> = HashMap::new();

    lines.map(|line|
        scan_fmt!(line, "#{} @ {},{}: {}x{}", i32, i32, i32, i32, i32)
    )
    .map(|x|
        (x.0.unwrap(), x.1.unwrap(), x.2.unwrap(), x.3.unwrap(), x.4.unwrap())
    )
    .for_each(|(id, x, y, w, h)| {
        for i in 0..w {
            for j in 0..h {
                let pos = (i+x, j+y);
                if let Some(ids) = grid.get_mut(&pos) {
                    ids.push(id);
                    for id in ids {
                        if let Some(is_overlapping) = overlaps.get_mut(&id) {
                            *is_overlapping = true;
                        } else {
                            overlaps.insert(*id, true);
                        }
                    }
                } else {
                    grid.insert(pos, vec![id]);
                    if overlaps.get(&id) == None {
                        overlaps.insert(id, false);
                    }
                };
            }
        }
    });

    let overlapping_cells = grid.values().filter(|cell| {
        cell.len() > 1
    });

    let non_overlapping_ids = overlaps.keys().zip(overlaps.values())
        .filter(|(_, is_overlapping)|
            **is_overlapping == false
        )
        .map(|(id, _)| *id)
        .collect::<Vec<i32>>();

    println!("{}", overlapping_cells.count());
    println!("{:?}", non_overlapping_ids);
}
