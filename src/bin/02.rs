use std::fs;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn parse(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("invalid move."),
        }
    }

    fn play(&self, other: &Self) -> u32 {
        match self {
            Move::Rock => match other {
                Move::Rock => 3 + 1,
                Move::Paper => 0 + 1,
                Move::Scissors => 6 + 1,
            },
            Move::Paper => match other {
                Move::Rock => 6 + 2,
                Move::Paper => 3 + 2,
                Move::Scissors => 0 + 2,
            },
            Move::Scissors => match other {
                Move::Rock => 0 + 3,
                Move::Paper => 6 + 3,
                Move::Scissors => 3 + 3,
            },
        }
    }

    fn plan(&self, other: &Self) -> Move {
        match self {
            Move::Rock => match other {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Move::Paper => match other {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors,
            },
            Move::Scissors => match other {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/02.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split(' ');
        let p1 = Move::parse(parts.next().unwrap());
        let p2 = Move::parse(parts.next().unwrap());
        total += p2.play(&p1);
    }
    println!("part1: {}", total);

    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split(' ');
        let p1 = Move::parse(parts.next().unwrap());
        let p2 = Move::parse(parts.next().unwrap());
        total += p2.plan(&p1).play(&p1);
    }
    println!("part2: {}", total);
}
