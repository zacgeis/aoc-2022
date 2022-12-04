use std::fs;

struct Range(u32, u32);

impl Range {
    fn parse(s: &str) -> Self {
        let mut parts = s.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        Range(start, end)
    }
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.0
    }
}

fn main() {
    let input = fs::read_to_string("inputs/04.txt").unwrap();

    let mut ranges = vec![];
    for line in input.lines() {
        let mut parts = line.split(',');
        let r1 = Range::parse(parts.next().unwrap());
        let r2 = Range::parse(parts.next().unwrap());
        ranges.push((r1, r2));
    }

    let mut total = 0;
    for (r1, r2) in &ranges {
        if r1.contains(r2) || r2.contains(r1) {
            total += 1;
        }
    }
    println!("part1: {}", total);

    let mut total = 0;
    for (r1, r2) in &ranges {
        if r1.overlaps(r2) || r2.overlaps(r1) {
            total += 1;
        }
    }
    println!("part2: {}", total);
}
