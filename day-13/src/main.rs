use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
enum Turn {
    Rwd,
    Lwd,
    Fwd
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    U,
    D,
    L,
    R
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let input = buf.lines();
    let mut carts: Vec<((usize,usize),Dir,Turn)> = Vec::new();
    let grid = input.enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '>' => { carts.push(((x,y), Dir::R, Turn::Lwd)); '#' }
                '<' => { carts.push(((x,y), Dir::L, Turn::Lwd)); '#' }
                '^' => { carts.push(((x,y), Dir::U, Turn::Lwd)); '#' }
                'v' => { carts.push(((x,y), Dir::D, Turn::Lwd)); '#' }
                _ => c
            }
        }).collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();
    loop {
        let mut dead: Vec<usize> = Vec::new();
        for i in 0..carts.len() {
            let ((x, y), dir, mut turn) = carts[i];
            let new_dir: Dir = match grid[y][x] {
                '\\' => match dir {
                    Dir::U => Dir::L,
                    Dir::D => Dir::R,
                    Dir::L => Dir::U,
                    Dir::R => Dir::D,
                }
                '/' => match dir {
                    Dir::U => Dir::R,
                    Dir::D => Dir::L,
                    Dir::L => Dir::D,
                    Dir::R => Dir::U,
                }
                '+' => match turn {
                        Turn::Lwd => {
                            turn = Turn::Fwd;
                            match dir {
                                Dir::U => Dir::L,
                                Dir::D => Dir::R,
                                Dir::L => Dir::D,
                                Dir::R => Dir::U,
                            }
                        }
                        Turn::Fwd => {
                            turn = Turn::Rwd;
                            dir
                        }
                        Turn::Rwd => {
                            turn = Turn::Lwd;
                            match dir {
                                Dir::U => Dir::R,
                                Dir::D => Dir::L,
                                Dir::L => Dir::U,
                                Dir::R => Dir::D,
                            }
                        }
                    }
                _ => dir
            };
            let (new_x,new_y) = match new_dir {
                Dir::U => (x, y-1),
                Dir::D => (x, y+1),
                Dir::L => (x-1, y),
                Dir::R => (x+1, y),
            };
            let collisions = carts
                .iter()
                .enumerate()
                .filter(|(_, ((x,y),_,_))| *x == new_x && *y == new_y)
                .map(|(i,_)| i)
                .collect::<Vec<usize>>();
            for cart in collisions {
                if !dead.contains(&cart) {
                    dead.push(cart);
                }
                dead.push(i);
            }
            //print(&grid, &carts, None);
            carts[i] = ((new_x, new_y), new_dir, turn);
        }
        dead.sort_by(|a,b| b.cmp(&a)); // Descending sort
        for cart in dead {
            carts.remove(cart);
        }
        carts.sort_by_key(|((x,_),_,_)| *x);
        carts.sort_by_key(|((_,y),_,_)| *y);
        if carts.len() == 1 {
            println!("{:?}", carts);
            break;
        }
    }
}

fn print(
    grid: &Vec<Vec<char>>,
    carts: &Vec<((usize,usize),Dir,Turn)>,
    collision: Option<(usize,usize)>
) {
    println!("");
    thread::sleep(Duration::from_millis(500));
    let mut tmp_grid = grid.clone();
    for ((x,y), dir, _) in carts.iter() {
        tmp_grid[*y][*x] = match dir {
            Dir::U => '^',
            Dir::D => 'v',
            Dir::L => '<',
            Dir::R => '>',
        };
    }
    if let Some((x,y)) = collision {
        tmp_grid[y][x] = 'X';
    }
    for row in tmp_grid.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!("");
    }
}
