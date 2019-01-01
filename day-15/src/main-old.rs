/// Derived from https://github.com/BlockCat/adventofcode2018/blob/master/src/day15.rs#L35
use std::fs::File;
use std::io::Read;
use std::collections::{ BinaryHeap, HashMap, HashSet, VecDeque };
use std::cmp::Reverse;
use self::Terrain::*;
use self::Race::*;
use std::thread;
use std::time;

#[derive(Debug,Eq,PartialEq)]
enum Terrain {
    Wall,
    Open,
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Unit {
    race: Race,
    hp: Hp,
    pos: Pos,
}

#[derive(Debug,Clone,Eq,PartialEq)]
enum Race {
    Elf,
    Goblin,
}

type Dist = u32;
type Grid = Vec<Vec<Terrain>>;
type Pos = (usize,usize);
type Hp = i32;

#[derive(Debug,Clone,Copy)]
enum Dir {
    N, E, S, W,
}

use std::ops::Add;
impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, dir: Dir) -> Pos {
        let (x, y) = self;
        //println!("{:?}, {:?}", self, dir);
        match dir {
            Dir::N => (x, y.saturating_sub(1)),
            Dir::S => (x, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x.saturating_sub(1), y)
        }
    }
}

fn parse() -> (Grid, Vec<Unit>) {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut units: Vec<Unit> = Vec::new();
    let grid: Vec<Vec<Terrain>> = buf
        .lines()
        .enumerate()
        .map(|(y, line)| line
             .chars()
             .enumerate()
             .map(|(x, c)| match c {
                 '#' => Terrain::Wall,
                 'G' => {
                     units.push(Unit {
                         race: Goblin,
                         hp: 200,
                         pos: (x, y)
                     });
                     Terrain::Open
                 }
                 'E' => {
                     units.push(Unit {
                         race: Elf,
                         hp: 200,
                         pos: (x, y)
                     });
                     Terrain::Open
                 }
                 _ => Terrain::Open,
             }).collect()
        ).collect();
    (grid, units)
}

fn get_positions(unit: &Unit, units: &Vec<Unit>) -> (HashSet<Pos>, HashSet<Pos>) {
    let unit_positions: HashSet<Pos> = units
        .iter()
        .filter(|x| *x != unit && x.hp > 0)
        .map(|x| x.pos)
        .collect();
    let enemy_positions: HashSet<Pos> = units
        .iter()
        .filter(|x| x.race != unit.race && x.hp > 0)
        .map(|x| x.pos)
        .collect();
    (unit_positions, enemy_positions)
}

fn get_enemy<'a>(unit: &Unit, units: &'a mut Vec<Unit>) -> &'a mut Unit {
    units.iter_mut()
        .filter(|enemy|
            enemy.race != unit.race &&
            enemy.hp > 0 &&
            ((enemy.pos.0 as i32 - unit.pos.0 as i32).abs() +
             (enemy.pos.1 as i32 - unit.pos.1 as i32).abs()) == 1
        )
        .min_by_key(|enemy| (enemy.hp, enemy.pos.1, enemy.pos.0))
        .unwrap()
}

fn dijkstra(
    (start_x, start_y): Pos,
    grid: &Grid,
    unit_positions: HashSet<Pos>,
    enemy_positions: HashSet<Pos>,
) -> Vec<Pos> {

    let mut unexplored: BinaryHeap<Reverse<(Dist, usize, usize)>> = BinaryHeap::new();
    let mut visited: HashMap<Pos,Option<Dir>> = HashMap::new();
    unexplored.push(Reverse((0, start_y, start_x)));
    visited.insert((start_x, start_y), None);

    let mut destinations = Vec::new();
    let mut max_dist = 1000;

    while let Some(Reverse((dist, y, x))) = unexplored.pop() {

        if dist > max_dist {
            continue;
        }

        for dir in vec![Dir::N, Dir::W, Dir::E, Dir::S] {
            let new_pos = (x,y) + dir;
            let new_dist = dist + 1;
            let is_too_long = new_dist > max_dist;
            let is_visited = visited.contains_key(&new_pos);
            let is_open = grid[new_pos.1][new_pos.0] == Open;
            let is_enemy = enemy_positions.contains(&new_pos);
            let is_other = unit_positions.contains(&new_pos);

            if !is_too_long {
                if is_enemy {
                    if !is_visited {
                        visited.insert(new_pos, Some(dir));
                    }
                    destinations.push(new_pos);
                    max_dist = new_dist;
                    continue;
                } else if is_open && !is_visited && !is_other {
                    visited.insert(new_pos, Some(dir));
                    unexplored.push(Reverse((new_dist, new_pos.1, new_pos.0)));
                }

            }
        }
    }

    match destinations.iter().min_by_key(|&(x,y)| (y,x)) {
        Some(dest) => {
            let mut dest = *dest;
            let mut path: VecDeque<Pos> = VecDeque::new();
            'paths: loop {
                path.push_front(dest);
                match visited[&dest] {
                    Some(Dir::N) => dest = dest + Dir::S,
                    Some(Dir::E) => dest = dest + Dir::W,
                    Some(Dir::S) => dest = dest + Dir::N,
                    Some(Dir::W) => dest = dest + Dir::E,
                    None => break path.into(),
                }
            }
        }
        None => {
            vec![(start_x, start_y)]
        }
    }
}

fn main() {
    let (grid, mut units) = parse();
    let mut elves = units.iter().filter(|u| u.race == Elf).count();
    let mut goblins = units.iter().filter(|u| u.race == Goblin).count();

    'rounds: for round in 0.. {
        units.sort_by_key(|unit| (unit.pos.1, unit.pos.0));
        'turns: for i in 0..units.len() {
            if units[i].hp <= 0 {
                continue;
            }
            let (unit_positions, enemy_positions) = get_positions(&units[i], &units);
            let path = dijkstra(units[i].pos, &grid, unit_positions, enemy_positions);

            if path.len() > 2 {
                units[i].pos = path[1];
            }
            //print(&grid, &units, units[i].pos);

            let unit = units[i].clone();

            if path.len() == 2 || path.len() == 3 {
                let enemy = get_enemy(&unit, &mut units);
                enemy.hp -= 3;
                
                if enemy.hp <= 0 {
                    match enemy.race {
                        Elf => elves -= 1,
                        Goblin => goblins -= 1,
                    }
                }

                if elves == 0 || goblins == 0 {
                    let sum = units
                        .iter()
                        .filter(|ally| ally.race == unit.race && ally.hp >= 0)
                        .map(|e| e.hp as usize)
                        .sum::<usize>();

                    let round = if i == units.iter().rposition(|u| u.hp > 0).unwrap() {
                        round + 1
                    } else {
                        round
                    };
                    println!("");
                    println!("{} = {} * {}", round * sum, round, sum);
                    return;
                }
            }
        }
    }
}

fn print(grid: &Grid, units: &Vec<Unit>, current: Pos) {
    thread::sleep(time::Duration::from_millis(20));
    let mut screen: Vec<Vec<char>> = Vec::new();
    for row in grid {
        let mut line: Vec<char> = Vec::new();
        for terrain in row {
            match terrain {
                Wall => line.push('#'),
                Open => line.push('.'),
            };
        }
        screen.push(line);
    }
    for unit in units {
        if unit.hp > 0 {
            let (x,y) = unit.pos;
            match unit.race {
                Elf => screen[y][x] = 'E',
                Goblin => screen[y][x] = 'G',
            }
        }
    }
    screen[current.1][current.0] = 'X';
    // Print
    for unit in units {
        print!("{}, ", unit.hp);
    }
    println!("");
    for line in screen {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
}
