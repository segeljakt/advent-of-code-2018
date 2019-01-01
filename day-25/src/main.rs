use scan_fmt::*;

#[derive(Hash,Ord,PartialOrd,Eq,PartialEq)]
struct Point(i32,i32,i32,i32);

fn manhattan(Point(x0,y0,z0,w0): &Point, Point(x1,y1,z1,w1): &Point) -> i32 {
    (x0-x1).abs() + (y0-y1).abs() + (z0-z1).abs() + (w0-w1).abs()
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x,y,z,w) = scan_fmt!(line, "{},{},{},{}", i32,i32,i32,i32);
            Point(x.unwrap(),y.unwrap(),z.unwrap(),w.unwrap())
        })
        .collect()
}

fn dfs(src: usize, edges: &Edges, visited: &mut Visited) {
    visited[src] = true;
    for &dst in &edges[src] {
        if !visited[dst] {
            dfs(dst, edges, visited);
        }
    }
}

type Edges = Vec<Vec<usize>>;
type Visited = Vec<bool>;

fn find_constellations(points: &Vec<Point>) -> usize {
    let mut edges = vec![Vec::new(); points.len()];
    let mut visited = vec![false; points.len()];
    for i in 0..points.len() {
        for j in 0..points.len() {
            if manhattan(&points[i], &points[j]) <= 3 {
                edges[i].push(j);
            }
        }
    }
    let mut num_constellations = 0;
    for src in 0..edges.len() {
        if !visited[src] {
            num_constellations += 1;
            dfs(src, &edges, &mut visited);
        }
    }
    num_constellations
}

fn main() {
    let points = parse(include_str!("input"));
    println!("{}", find_constellations(&points));
}

#[test]
fn test1() {
    let points = parse(include_str!("example_input_1"));
    assert_eq!(find_constellations(&points), 2);
}

#[test]
fn test2() {
    let points = parse(include_str!("example_input_2"));
    assert_eq!(find_constellations(&points), 4);
}

#[test]
fn test3() {
    let points = parse(include_str!("example_input_3"));
    assert_eq!(find_constellations(&points), 3);
}

#[test]
fn test4() {
    let points = parse(include_str!("example_input_4"));
    assert_eq!(find_constellations(&points), 8);
}
