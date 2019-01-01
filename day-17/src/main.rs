use scan_fmt::*;
use std::io::Read;
use std::fs::File;
use std::time;
use std::thread;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Tile {
    Clay,
    Sand,
    Still,
    Spring,
    Drip,
}

fn parse() -> (Vec<Vec<Tile>>, (usize, usize), (usize,usize)) {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut tiles = Vec::new();
    for line in buf.lines() {
        let (x, y0, y1) = scan_fmt!(line, "x={}, y={}..{}", usize, usize, usize);
        if x.is_some() {
            let (x, y0, y1) = (x.unwrap(), y0.unwrap(), y1.unwrap());
            for y in y0..=y1 {
                tiles.push((y,x));
            }
        } else {
            let (y, x0, x1) = scan_fmt!(line, "y={}, x={}..{}", usize, usize, usize);
            let (y, x0, x1) = (y.unwrap(), x0.unwrap(), x1.unwrap());
            for x in x0..=x1 {
                tiles.push((y,x));
            }
        }
    }
    let (spring_y, spring_x) = (1, 500);
    let y0: usize = *tiles.iter().map(|(y,_)| y).max().unwrap().min(&spring_y);
    let x0: usize = *tiles.iter().map(|(_,x)| x).min().unwrap().min(&spring_x)-2;
    let y1: usize = *tiles.iter().map(|(y,_)| y).max().unwrap().max(&spring_y);
    let x1: usize = *tiles.iter().map(|(_,x)| x).max().unwrap().max(&spring_x)+2;
    let h = y1-y0+1;
    let w = x1-x0+1;
    let mut grid = vec![vec![Tile::Sand; w]; h];
    for (y,x) in tiles {
        grid[y-y0][x-x0] = Tile::Clay;
    }
    let (spring_y, spring_x) = (spring_y-y0, spring_x-x0);
    grid[spring_y][spring_x] = Tile::Spring;
    (grid, (h,w), (spring_y,spring_x))
}

fn saturate(
    mut grid: Vec<Vec<Tile>>,
    (h, _w): (usize, usize),
    (spring_y, spring_x): (usize,usize),
) -> Vec<Vec<Tile>> {
    let mut drips: Vec<(usize,usize)> = Vec::new();
    for _ in 0..5000 { // Go big or go home
        grid[spring_y+1][spring_x] = Tile::Drip;
        drips.push((spring_y+1, spring_x));
        //print(&grid);
        drips = drips
            .into_iter()
            .flat_map(|(y,x)|
                if y+1 < h {
                    match grid[y+1][x] {
                        Tile::Sand => { // Move vertically
                            grid[y][x] = Tile::Sand;
                            grid[y+1][x] = Tile::Drip;
                            vec![(y+1,x)]
                        }
                        Tile::Still | Tile::Clay => { // Move horizontally
                            let mut a = 0;
                            let left_is_walled = loop {
                                a += 1;
                                if grid[y+1][x-a] == Tile::Sand {
                                    break false;
                                }
                                if grid[y][x-a] == Tile::Clay || grid[y][x-a] == Tile::Still {
                                    break true;
                                }
                            };
                            let mut b = 0;
                            let right_is_walled = loop {
                                b += 1;
                                if grid[y+1][x+b] == Tile::Sand {
                                    break false;
                                }
                                if grid[y][x+b] == Tile::Clay || grid[y][x+b] == Tile::Still {
                                    break true;
                                }
                            };
                            if left_is_walled && right_is_walled {
                                for i in x-a+1..x+b {
                                    grid[y][i] = Tile::Still;
                                }
                                vec![]
                            } else {
                                grid[y][x] = Tile::Sand;
                                let mut new_drips = Vec::new();
                                if grid[y][x-1] == Tile::Sand {
                                    grid[y][x-1] = Tile::Drip;
                                    new_drips.push((y,x-1));
                                }
                                if grid[y][x+1] == Tile::Sand {
                                    grid[y][x+1] = Tile::Drip;
                                    new_drips.push((y,x+1));
                                }
                                new_drips
                            }
                        }
                        _ => vec![]
                    }
                } else {
                    grid[y][x] = Tile::Sand;
                    vec![]
                }
            )
            .collect();
    }
    return grid;
}

fn main() {
    let (grid, dim, spring) = parse();
    let saturated = saturate(grid, dim, spring);
    print(&saturated);
    let count: usize = saturated
        .iter()
        .map(|row| row
            .iter()
            .filter(|&&tile| tile == Tile::Drip || tile == Tile::Still)
            .count()
        )
        .sum();
    println!("Number of watery tiles: {}", count);
    let still_count: usize = saturated
        .iter()
        .map(|row| row
            .iter()
            .filter(|&&tile| tile == Tile::Still)
            .count()
        )
        .sum();
    println!("Number of still watery tiles: {}", still_count);
}

fn print(grid: &Vec<Vec<Tile>>) {
    thread::sleep(time::Duration::from_millis(100));
    for row in grid {
        for tile in row {
            match tile {
                Tile::Clay   => print!("#"),
                Tile::Sand   => print!("."),
                Tile::Still  => print!("~"),
                Tile::Spring => print!("+"),
                Tile::Drip   => print!("|"),
            }
        }
        println!("");
    }
    println!("");
}
