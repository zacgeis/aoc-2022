use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Node {
    File(String, u64),
    Dir(String, HashMap<String, Node>, u64),
}

impl Node {
    fn new_dir(name: String) -> Self {
        Node::Dir(name, HashMap::new(), 0)
    }
    fn new_file(name: String, size: u64) -> Self {
        Node::File(name, size)
    }
}

#[derive(Debug)]
enum Action {
    Cd(String),
    Dir(String),
    File(String, u64),
}

fn main() {
    let input = fs::read_to_string("inputs/07.txt").unwrap();
    let mut actions = vec![];
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir = line.split(' ').skip(2).next().unwrap();
            actions.push(Action::Cd(dir.to_string()));
        } else if line.starts_with("$ ls") {
            // do nothing.
        } else if line.starts_with("dir ") {
            let dir = line.split(' ').skip(1).next().unwrap();
            actions.push(Action::Dir(dir.to_string()));
        } else {
            let mut parts = line.split(' ');
            let size = parts.next().unwrap().parse().unwrap();
            let name = parts.next().unwrap();
            actions.push(Action::File(name.to_string(), size));
        }
    }

    let mut actions_iter = actions.iter();
    actions_iter.next();
    let mut root = Node::new_dir("/".to_string());
    build(&mut actions_iter, &mut root);
    count(&mut root);
    println!("part1: {}", part1(&root));

    let mut sizes = vec![];
    part2(&root, &mut sizes);
    let total_size = sizes.iter().max().unwrap();
    let size_to_free = total_size - 40000000;
    println!(
        "part2: {}",
        sizes
            .iter()
            .filter(|size| **size >= size_to_free)
            .min()
            .unwrap()
    );
}

fn build<'a>(actions: &mut impl Iterator<Item = &'a Action>, cwd: &mut Node) {
    loop {
        match actions.next() {
            None => return,
            Some(Action::Cd(dir)) => {
                if dir == ".." {
                    return;
                } else {
                    match cwd {
                        Node::Dir(_, children, _) => {
                            build(actions, children.get_mut(dir).unwrap());
                        }
                        _ => panic!("not a dir."),
                    }
                }
            }
            Some(Action::Dir(dir)) => match cwd {
                Node::Dir(_, children, _) => {
                    children.insert(dir.to_string(), Node::new_dir(dir.to_string()));
                }
                _ => panic!("not a dir."),
            },
            Some(Action::File(name, size)) => match cwd {
                Node::Dir(_, children, _) => {
                    children.insert(name.to_string(), Node::new_file(name.to_string(), *size));
                }
                _ => panic!("not a dir."),
            },
        }
    }
}

fn count(node: &mut Node) -> u64 {
    match node {
        Node::File(_, size) => *size,
        Node::Dir(_, children, total_size) => {
            for child in children.values_mut() {
                *total_size += count(child)
            }
            *total_size
        }
    }
}

fn part1(node: &Node) -> u64 {
    match node {
        Node::Dir(_, children, total_size) => {
            let mut total = 0;
            for child in children.values() {
                total += part1(child)
            }
            if *total_size <= 100000 {
                total += total_size;
            }
            total
        }
        _ => 0,
    }
}

fn part2(node: &Node, values: &mut Vec<u64>) {
    match node {
        Node::Dir(_, children, total_size) => {
            values.push(*total_size);
            for child in children.values() {
                part2(child, values);
            }
        }
        _ => (),
    }
}
