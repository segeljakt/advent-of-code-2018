#![feature(box_into_raw_non_null)]
#![allow(unused)]

use std::fs::File;
use std::io::Read;
use scan_fmt::*;
use std::ptr::NonNull;

#[derive(Debug, Clone, Copy)]
struct Node<T> {
    prev: NonNull<Node<T>>,
    val: T,
    next: NonNull<Node<T>>,
}

impl<T> Node<T> {
    fn new(val: T) -> NonNull<Node<T>> {
        let node = Node {
            prev: NonNull::dangling(),
            val,
            next: NonNull::dangling(),
        };
        let mut this = Box::into_raw_non_null(Box::new(node));
        unsafe {
            (*this.as_mut()).prev = this;
            (*this.as_mut()).next = this;
        };
        this
    }
}

trait DoublyLinkedList<T> {
    fn next(self) -> NonNull<Node<T>>;
    fn prev(self) -> NonNull<Node<T>>;
    fn insert_next(self, val: T);
    fn print(self);
    fn print_highest(self);
    fn val(&self) -> T;
    fn contains(&self, val: T) -> bool;
    fn remove_prev(self) -> T;
    fn print_circle(self, player: usize, highest: T);
}

use std::fmt::Display;
impl<T:Display+Eq+Clone+Copy> DoublyLinkedList<T> for NonNull<Node<T>> {
    fn prev(self) -> Self {
        unsafe {
            (*self.as_ref()).prev
        }
    }
    fn next(self) -> Self {
        unsafe {
            (*self.as_ref()).next
        }
    }
    fn insert_next(mut self, val: T) {
        unsafe {
            let mut new = Node::new(val);
            (*new.as_mut()).prev = self;
            (*new.as_mut()).next = (*self.as_mut()).next;
            (*(*self.as_mut()).next.as_mut()).prev = new;
            (*self.as_mut()).next = new;
        }
    }
    fn remove_prev(mut self) -> T {
        unsafe {
            let mut prev = (*self.as_mut()).prev;
            let mut prev_prev = (*prev.as_mut()).prev;
            (*prev_prev.as_mut()).next = self;
            (*self.as_mut()).prev = prev_prev;
            (*prev.as_mut()).val
        }
    }
    fn print(self) {
        unsafe {
            print!(" {} ", (*self.as_ref()).val);
        }
    }
    fn print_highest(self) {
        unsafe {
            print!("({})", (*self.as_ref()).val);
        }
    }
    fn val(&self) -> T {
        unsafe {
            (*self.as_ptr()).val
        }
    }
    fn contains(&self, val: T) -> bool {
        unsafe {
            (*self.as_ptr()).val == val
        }
    }
    fn print_circle(self, player: usize, highest: T) {
        let mut iter = self;
        let first_val = iter.val();
        print!("[{}] ", player);
        loop {
            if iter.contains(highest) {
                iter.print_highest();
            } else {
                iter.print();
            }
            iter = iter.next();
            if iter.contains(first_val) {
                break;
            }
        }
        println!("");
    }
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let input = buf.lines().next().unwrap();
    let (num_players, last_marble) = scan_fmt!(
        input, "{} players; last marble is worth {} points", usize, usize
    );
    let (num_players, last_marble) = (num_players.unwrap(), last_marble.unwrap());
    let mut score = vec![0; num_players];
    let mut marble = 0;
    let mut circle = Node::new(marble);
    let first = circle.clone();
    println!("[-] (0)");

    'outer: loop {
        for player in 0..num_players {
            marble += 1;
            if (marble % 23) != 0 {
                circle = circle.next();
                circle.insert_next(marble);
                circle = circle.next();
            } else {
                score[player] += marble;
                for i in 0..6 {
                    circle = circle.prev();
                }
                score[player] += circle.remove_prev();
            }
            //first.print_circle(player+1, marble);
            if marble == last_marble {
                break 'outer;
            }
        }
    }
    println!("{:?}", score.iter().max().unwrap());
}
