use scan_fmt::*;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Reverse;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Kind {
    Rocky,
    Narrow,
    Wet
}

#[derive(Hash,Ord,PartialOrd,Clone,Copy,Debug,Eq,PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}
type Erosion = usize;

#[derive(Clone,Copy,Debug)]
struct Region {
    kind: Kind,
    erosion: Erosion,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::Rocky  => { let _ = write!(f, "."); }
            Kind::Wet    => { let _ = write!(f, "="); }
            Kind::Narrow => { let _ = write!(f, "|"); }
        };
        Ok(())
    }
}

use std::fmt;

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                let _ = write!(f, "{}", self.regions.get(&Pos{x,y}).unwrap());
            }
            let _ = writeln!(f, "");
        }
        Ok(())
    }
}

#[derive(Debug,Hash,Eq,PartialEq,Clone,Copy,Ord,PartialOrd)]
enum Tool {
    Neither,
    Torch,
    Gear,
}

struct Cave {
    depth: usize,
    origin: Pos,
    target: Pos,
    regions: HashMap<Pos,Region>,
}

impl Region {
    fn new(erosion_level: usize) -> Region {
        match erosion_level % 3 {
            0 => Region { kind: Kind::Rocky,  erosion: erosion_level },
            1 => Region { kind: Kind::Wet,    erosion: erosion_level },
            2 => Region { kind: Kind::Narrow, erosion: erosion_level },
            _ => unreachable!()
        }
    }
    fn risk(&self) -> usize {
        match self.kind {
            Kind::Rocky  => 0,
            Kind::Wet    => 1,
            Kind::Narrow => 2,
        }
    }
    fn equippable(&self, tool: Tool) -> bool {
        match self.kind {
            Kind::Rocky  => vec![Tool::Torch, Tool::Gear].contains(&tool),
            Kind::Wet    => vec![Tool::Gear,  Tool::Neither].contains(&tool),
            Kind::Narrow => vec![Tool::Torch, Tool::Neither].contains(&tool),
        }
    }

}

impl Cave {
    fn from(input: &str) -> Cave {
        let mut input = input.lines();
        let depth = scan_fmt!(input.next().unwrap(), "depth: {}", usize);
        let (x,y) = scan_fmt!(input.next().unwrap(), "target: {},{}", usize, usize);
        let depth = depth.unwrap();
        let (x,y) = (x.unwrap(), y.unwrap());
        Cave {
            depth: depth,
            origin: Pos { x: 0, y: 0 },
            target: Pos { x, y },
            regions: HashMap::new(),
        }
    }

    fn get_region(&mut self, pos: Pos) -> Region {
        if let Some(&region) = self.regions.get(&pos) {
            region
        } else {
            let region = Region::new(self.get_erosion_level(pos));
            self.regions.insert(pos, region);
            region
        }

    }

    fn get_erosion_level(&mut self, pos: Pos) -> usize {
        (self.get_geologic_index(pos) + self.depth) % 20183
    }

    fn get_geologic_index(&mut self, Pos { x, y }: Pos) -> usize {
        match (x,y) {
            (x,y) if Pos {x,y} == self.origin => 0,
            (x,y) if Pos {x,y} == self.target => 0,
            (_,0) => x*16807,
            (0,_) => y*48271,
            (_,_) => {
                self.get_region(Pos {x: x-1, y}).erosion * self.get_region(Pos {x, y: y-1}).erosion
            }
        }
    }

    fn risk_level(&mut self) -> usize {
        let mut sum = 0;
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                sum += self.get_region(Pos { x, y }).risk();
            }
        }
        let origin = self.get_region(self.target).risk();
        let target = self.get_region(self.origin).risk();
        sum - origin - target
    }


    fn find_shortest_path(&mut self) -> usize {
        let mut frontier = BinaryHeap::new();
        let mut visited = HashMap::new();
        frontier.push(Reverse((0, self.origin, Tool::Torch )));
        let mut shortest_time = 0;

        while let Some(Reverse((time,pos,tool))) = frontier.pop() {
            if visited.contains_key(&(pos,tool)) && visited[&(pos,tool)] <= time {
                continue;
            }
            visited.insert((pos,tool), time);
            if pos == self.target && tool == Tool::Torch {
                shortest_time = time;
                break;
            }
            for &new_tool in &[Tool::Neither, Tool::Torch, Tool::Gear] {
                if self.get_region(pos).equippable(new_tool) {
                    frontier.push(Reverse((time+7, pos, new_tool)));
                }
            }
            for &(dx, dy) in &[(-1,0),(0,-1),(1,0),(0,1)] {
                if (dx < 0 && pos.x == 0) || (dy < 0 && pos.y == 0) {
                    continue;
                } else {
                    let new_pos = Pos {
                        x: (pos.x as i32 + dx) as usize,
                        y: (pos.y as i32 + dy) as usize
                    };
                    if self.get_region(new_pos).equippable(tool) {
                        frontier.push(Reverse((time+1, new_pos, tool)));
                    }
                    
                }
            }
        }
        shortest_time
    }
}
fn manhattan((x0,y0): (usize,usize), (x1,y1): (usize,usize)) -> usize {
    let w = usize::max(x1,x0) - usize::min(x1,x0);
    let h = usize::max(y1,y0) - usize::min(y1,y0);
    w+h
}

fn main() {
    let mut cave = Cave::from(include_str!("input"));
    //println!("{}", cave);
    println!("{}", cave.risk_level());
    println!("{:?}", cave.find_shortest_path());
}


#[test]
fn test1() {
    let mut cave = Cave::from(include_str!("test_input"));
    assert_eq!(cave.risk_level(), 114);
    assert_eq!(format!("{}", cave), ".=.|=.|.|=.\n\
                                     .|=|=|||..|\n\
                                     .==|....||=\n\
                                     =.|....|.==\n\
                                     =|..==...=.\n\
                                     =||.=.=||=|\n\
                                     |.=.===|||.\n\
                                     |..==||=.|=\n\
                                     .=..===..=|\n\
                                     .======|||=\n\
                                     .===|=|===.\n");
    assert_eq!(cave.find_shortest_path(), 45);
}

#[test]
fn test2() {
    let mut cave = Cave::from(include_str!("input"));
    assert_eq!(cave.risk_level(), 9899);
    assert_eq!(cave.regions.get(&cave.origin).unwrap().kind, Kind::Rocky);
    assert_eq!(cave.regions.get(&cave.target).unwrap().kind, Kind::Rocky);
}












