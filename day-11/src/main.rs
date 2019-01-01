#![allow(unused)]

use std::fs::File;
use std::io::Read;
use scan_fmt::*;
use std::{thread, time};
use ndarray::*;

fn main() {
    let serial_number: i32 = 5177;
    let mut grid: [[i32; 300]; 300] = [[0; 300]; 300];
    for y in 1..=300 as i32 {
        for x in 1..=300 as i32 {
            let rack_id = x + 10;
            let power_level = rack_id * y;
            let power_level = power_level + serial_number;
            let power_level = power_level * rack_id;
            let power_level = (power_level % 1000) / 100;
            let power_level = power_level - 5;
            grid[y as usize-1][x as usize-1] = power_level;
        }
    }

    let mut max_square_power = 0;
    let mut max_y = 0;
    let mut max_x = 0;
    for y in 0..297 {
        for x in 0..297 {
            let mut square_power = 0;
            for square_y in 0..3 {
                for square_x in 0..3 {
                    square_power += grid[y+square_y][x+square_x];
                }
            }
            if square_power > max_square_power {
                max_square_power = square_power;
                max_y = y;
                max_x = x;
            }
        }
    }
    println!("{}: ({},{})", max_square_power, max_x+1, max_y+1);
    //let max_x = 33;
    //let max_y = 45;
    for y in max_y..max_y+3 {
        print!("[");
        print!("{}", grid[y][max_x]);
        for x in max_x+1..max_x+3 {
            print!(",{}", grid[y][x]);
        }
        println!("]");
    }


    
}
