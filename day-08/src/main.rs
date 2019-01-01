use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut input = buf.lines().next().unwrap().split(" ")
        .map(|word| word.parse::<usize>().unwrap());
    println!("{}", parse_node(&mut input));
}

fn parse_node<'a, I>(input: &mut I) -> usize
    where I: Iterator<Item = usize>,
{
    let num_children = input.next().unwrap();
    let num_metadata = input.next().unwrap();

    let mut sum: usize = 0;

    for _ in 0..num_children {
        sum += parse_node(input);
    }

    for _ in 0..num_metadata {
        sum += input.next().unwrap();
    }

    sum

}
