#![allow(unused)]

use std::fs::File;
use std::io::Read;
use scan_fmt::*;
use std::{thread, time};

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let input = buf.lines();
    let mut points: Vec<(i32,i32,i32,i32)> = input
        .enumerate()
        .map(|(num, line)| {
            let x = scan_fmt!(
                line,
                "position=<{}, {}> velocity=<{}, {}>",
                i32, i32, i32, i32
            );
            (x.0.unwrap(), x.1.unwrap(), x.2.unwrap(), x.3.unwrap())
        }).collect();

    let mut matrix = [[' '; 80]; 25];
    for i in 1.. {

        let min_x = points.iter().map(|p| p.0).min().unwrap();
        let min_y = points.iter().map(|p| p.1).min().unwrap();
        let max_x = points.iter().map(|p| p.0).max().unwrap();
        let max_y = points.iter().map(|p| p.1).max().unwrap();

        let h = max_y - min_y;
        let w = max_x - min_x;

        if h > 0 && h < 25 && w > 0 && w < 80 {
            for (x, y, vx, vy) in points.iter() {
                matrix[(*y-min_y) as usize][(*x-min_x) as usize] = '#';
            }

            for y in 0..25 {
                for x in 0..80 {
                    print!("{}", matrix[y][x]);
                }
                println!("");
            }
            println!("-----------------------------------");
            thread::sleep(time::Duration::from_millis(1000));

            for (x, y, vx, vy) in points.iter() {
                matrix[(*y-min_y) as usize][(*x-min_x) as usize] = ' ';
            }
        }
        let mut new_points = Vec::new();
        for (x, y, vx, vy) in points.iter() {
            new_points.push((*x+*vx, *y+*vy, *vx, *vy));
        }
        points = new_points;
        println!("{}", i);
    }
}
