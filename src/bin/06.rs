use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/06.txt").unwrap();
    let chars: Vec<char> = input.chars().collect();
    println!("part1: {}", solve(&chars, 4));
    println!("part2: {}", solve(&chars, 14));
}

fn solve(chars: &[char], window_length: usize) -> u32 {
    let mut set = HashSet::new();
    let mut pos = 0;
    for window in chars.windows(window_length) {
        set.clear();
        for c in window {
            set.insert(c);
        }
        if set.len() == window_length {
            break;
        }
        pos += 1;
    }
    return pos + window_length as u32;
}
