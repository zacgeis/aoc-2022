use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    let mut values: Vec<u32> = vec![];
    let mut elf: Vec<u32> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            values.push(elf.iter().sum());
            elf = vec![];
        } else {
            elf.push(line.parse().unwrap());
        }
    }
    values.sort();
    println!("part1: {}", values[values.len() - 1]);
    println!("part2: {}", values.iter().rev().take(3).sum::<u32>());
}
