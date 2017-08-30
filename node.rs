#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
}

impl fmt::Display for Node<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value: value, children: vec![] }
    }
    fn add(&mut self, child: Node<T>) {
        self.children.push(child);
    }
}

fn main() {
    let mut test: Node<char> = Node { value: 'a', children: vec![] };
    let mut child0: Node<char> = Node { value: 'b', children: vec![] };
    let mut child1: Node<char> = Node { value: 'c', children: vec![] };
    let mut child2: Node<char> = Node { value: 'd', children: vec![] };
    child0.add(child2);
    test.add(child1);
    test.add(child0);
    dfs(test);
}

fn dfs(start: Node<char>) {
    let mut fringe = vec![];
    fringe.push(start);
    while !fringe.is_empty() {
        let n = fringe.pop().unwrap();
        println!("{:?}", n.value);
        for child in n.children {
            fringe.push(child);
        }
    }
}

fn bfs(start: Node<char>) {
    let mut fringe = vec![];
    fringe.push(start);
    while !fringe.is_empty() {
        let n = fringe.pop().unwrap();
        println!("{:?}", n.value);
        for child in n.children {
            fringe.insert(0, child);
        }
    }
}
