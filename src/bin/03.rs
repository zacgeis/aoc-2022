use std::collections::HashSet;
use std::fs;

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        1 + c as u32 - 'a' as u32
    } else {
        27 + c as u32 - 'A' as u32
    }
}

fn main() {
    let input = fs::read_to_string("inputs/03.txt").unwrap();

    let mut rucksacks: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        rucksacks.push(line.chars().collect());
    }

    let mut total: u32 = 0;
    for rucksack in &rucksacks {
        let (first, second) = rucksack.split_at(rucksack.len() / 2);
        let first: HashSet<char> = first.iter().map(|c| c.clone()).collect();
        let second: HashSet<char> = second.iter().map(|c| c.clone()).collect();
        total += first
            .intersection(&second)
            .map(|c| priority(*c))
            .sum::<u32>()
    }
    println!("part1: {}", total);

    let mut total: u32 = 0;
    for chunk in rucksacks.chunks(3) {
        let mut iter = chunk.iter();
        let mut set: HashSet<char> = iter.next().unwrap().iter().map(|c| c.clone()).collect();
        for rucksack in iter {
            set = set
                .intersection(&rucksack.iter().map(|c| c.clone()).collect())
                .map(|c| c.clone())
                .collect();
        }
        total += set.iter().map(|c| priority(*c)).sum::<u32>()
    }
    println!("part2: {}", total);
}
