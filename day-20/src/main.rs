#![allow(dead_code)]
#![allow(unused_imports)]
use std::io::Read;
use std::fs::File;
use pest::Parser;
use pest_derive::*;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "regex.pest"]
struct Regex;

type Pos = (i32,i32);

const ORIGIN_Y: i32 = 0;
const ORIGIN_X: i32 = 0;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Side {
    Door,
    Wall,
}

#[derive(Debug,Copy,Clone)]
struct Room {
    east:  Side,
    west:  Side,
    north: Side,
    south: Side,
}

impl Room {
    fn new() -> Room {
        Room {
            east:  Side::Wall,
            west:  Side::Wall,
            north: Side::Wall,
            south: Side::Wall,
        }
    }
}

fn read_input() -> String {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

use pest::iterators::Pair;

fn explore<'i>(seq: Pair<'i, Rule>, map: &mut HashMap<Pos, Room>, start: Pos) -> Vec<Pos> {
    let mut branches = vec![start];
    // The loop of madness
    for c in seq.into_inner() {
        let rule = c.as_rule();
        let inner = c.into_inner();
        branches = branches.iter().flat_map(|&(y,x)|
            match rule {
                Rule::east => {
                    map.entry((y,  x)).or_insert(Room::new()).east = Side::Door;
                    map.entry((y,x+1)).or_insert(Room::new()).west = Side::Door;
                    vec![(y,x+1)]
                }
                Rule::west => {
                    map.entry((y,  x)).or_insert(Room::new()).west = Side::Door;
                    map.entry((y,x-1)).or_insert(Room::new()).east = Side::Door;
                    vec![(y,x-1)]
                }
                Rule::north => {
                    map.entry((y,  x)).or_insert(Room::new()).north = Side::Door;
                    map.entry((y-1,x)).or_insert(Room::new()).south = Side::Door;
                    vec![(y-1,x)]
                }
                Rule::south => {
                    map.entry((y,  x)).or_insert(Room::new()).south = Side::Door;
                    map.entry((y+1,x)).or_insert(Room::new()).north = Side::Door;
                    vec![(y+1,x)]
                }
                Rule::group => {
                    inner.clone().flat_map(|option| match option.as_rule() {
                            Rule::seq => explore(option, map, (y, x)),
                            Rule::bar => vec![/*(y,x)*/], // Should not be here
                            _ => unreachable!()
                        }
                    ).collect()
                }
                _ => {
                    println!("Unknown rule {:?}", rule);
                    unreachable!()
                }
            }
        ).collect();
    }
    branches
}

impl Map {
    fn from(input: &str) -> Map {
        let mut regex = Regex::parse(Rule::regex, input).expect("Failed parsing regex");
        let seq = regex.next().unwrap().into_inner().next().unwrap();
        let mut map = HashMap::new();

        println!("Exploring..");
        explore(seq, &mut map, (ORIGIN_Y, ORIGIN_X));

        Map(map)
    }
}

use std::cmp::Reverse;

fn find_longest_path(Map(map): &Map) -> (usize, usize) {
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, (ORIGIN_Y, ORIGIN_X))));
    let mut visited = HashMap::new();
    visited.insert((ORIGIN_Y, ORIGIN_X), 0);
    let mut longest_path = 0;
    while let Some(Reverse((dist,(y,x)))) = frontier.pop() {
        longest_path = longest_path.max(dist);
        let room = map.get(&(y,x)).unwrap();
        let mut add_room = |pos| if !visited.contains_key(&pos) {
            frontier.push(Reverse((dist+1, pos)));
            visited.insert(pos, dist+1);
        };
        if room.east  == Side::Door { add_room((y,x+1)); }
        if room.west  == Side::Door { add_room((y,x-1)); }
        if room.north == Side::Door { add_room((y-1,x)); }
        if room.south == Side::Door { add_room((y+1,x)); }
    }
    let long_paths = visited.values().filter(|&&x| x >= 1000).count();
    (longest_path, long_paths)
}

fn main() {
    let input = read_input();
    let input = input.lines().next().unwrap();
    let map = Map::from(input);
    let (longest_path, long_paths) = find_longest_path(&map);

    println!("{}", map);
    println!("{}", longest_path);
    println!("{}", long_paths);
}

#[derive(Copy,Clone)]
enum Tile {
    HorizontalDoor,
    VerticalDoor,
    Wall,
    Room,
    Unknown,
    Origin,
}

struct Map(HashMap<Pos, Room>);

use std::fmt;

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Map(map) = self;
        let y0 = *map.keys().map(|(y,_)| y).min().unwrap().min(&ORIGIN_Y);
        let x0 = *map.keys().map(|(_,x)| x).min().unwrap().min(&ORIGIN_X);
        let y1 = *map.keys().map(|(y,_)| y).max().unwrap().max(&ORIGIN_Y);
        let x1 = *map.keys().map(|(_,x)| x).max().unwrap().max(&ORIGIN_X);
        let (h, w) = (((y1-y0+1)*2+1) as usize, ((x1-x0+1)*2+1) as usize);

        let mut grid = vec![vec![Tile::Unknown; w]; h];

        for ((y,x), room) in map.iter() {
            let (y,x) = (((y-y0)*2+1) as usize, ((x-x0)*2+1) as usize);
            grid[y][x] = Tile::Room;
            grid[y-1][x-1] = Tile::Wall;
            grid[y-1][x+1] = Tile::Wall;
            grid[y+1][x-1] = Tile::Wall;
            grid[y+1][x+1] = Tile::Wall;
            grid[y][x+1] = if room.east == Side::Door { Tile::VerticalDoor } else { Tile::Wall };
            grid[y][x-1] = if room.west == Side::Door { Tile::VerticalDoor } else { Tile::Wall };
            grid[y-1][x] = if room.north == Side::Door { Tile::HorizontalDoor } else { Tile::Wall };
            grid[y+1][x] = if room.south == Side::Door { Tile::HorizontalDoor } else { Tile::Wall };
        }

        grid[((ORIGIN_Y-y0)*2+1) as usize][((ORIGIN_X-x0)*2+1) as usize] = Tile::Origin;

        for i in 0..grid.len() {
            for tile in grid[i].iter() {
                match tile {
                    Tile::HorizontalDoor => { let _ = write!(f, "-"); },
                    Tile::VerticalDoor   => { let _ = write!(f, "|"); },
                    Tile::Wall           => { let _ = write!(f, "#"); },
                    Tile::Room           => { let _ = write!(f, "."); },
                    Tile::Unknown        => { let _ = write!(f, " "); },
                    Tile::Origin         => { let _ = write!(f, "X"); },
                };
            }
            if i < grid.len()-1 {
                let _ = writeln!(f, "");
            }
        }
        Ok(())
    }
}


#[test]
fn test1() {
    let map = Map::from("^ENWWW(NEEE|SSE(EE|N))$");
    assert_eq!(format!("{}", map), "#########\n\
                                    #.|.|.|.#\n\
                                    #-#######\n\
                                    #.|.|.|.#\n\
                                    #-#####-#\n\
                                    #.#.#X|.#\n\
                                    #-#-#####\n\
                                    #.|.|.|.#\n\
                                    #########")
}

#[test]
fn test2() {
    let map = Map::from("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
    assert_eq!(format!("{}", map), "###########\n\
                                    #.|.#.|.#.#\n\
                                    #-###-#-#-#\n\
                                    #.|.|.#.#.#\n\
                                    #-#####-#-#\n\
                                    #.#.#X|.#.#\n\
                                    #-#-#####-#\n\
                                    #.#.|.|.|.#\n\
                                    #-###-###-#\n\
                                    #.|.|.#.|.#\n\
                                    ###########")
}

#[test]
fn test3() {
    let map = Map::from("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
    assert_eq!(format!("{}", map), "#############\n\
                                    #.|.|.|.|.|.#\n\
                                    #-#####-###-#\n\
                                    #.#.|.#.#.#.#\n\
                                    #-#-###-#-#-#\n\
                                    #.#.#.|.#.|.#\n\
                                    #-#-#-#####-#\n\
                                    #.#.#.#X|.#.#\n\
                                    #-#-#-###-#-#\n\
                                    #.|.#.|.#.#.#\n\
                                    ###-#-###-#-#\n\
                                    #.|.#.|.|.#.#\n\
                                    #############");
    let (longest_path, _) = find_longest_path(&map);
    assert_eq!(longest_path, 23);
}

#[test]
fn test4() {
    let map = Map::from("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
    assert_eq!(format!("{}", map), "###############\n\
                                    #.|.|.|.#.|.|.#\n\
                                    #-###-###-#-#-#\n\
                                    #.|.#.|.|.#.#.#\n\
                                    #-#########-#-#\n\
                                    #.#.|.|.|.|.#.#\n\
                                    #-#-#########-#\n\
                                    #.#.#.|X#.|.#.#\n\
                                    ###-#-###-#-#-#\n\
                                    #.|.#.#.|.#.|.#\n\
                                    #-###-#####-###\n\
                                    #.|.#.|.|.#.#.#\n\
                                    #-#-#####-#-#-#\n\
                                    #.#.|.|.|.#.|.#\n\
                                    ###############");
    let (longest_path, _) = find_longest_path(&map);
    assert_eq!(longest_path, 31);
}
