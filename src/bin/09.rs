use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn chase(&mut self, other: Self) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        if dx.abs() > 1 || dy.abs() > 1 {
            self.x += dx.clamp(-1, 1);
            self.y += dy.clamp(-1, 1);
        }
    }
}

enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Move {
    fn parse(s: &str) -> Self {
        let mut parts = s.split(' ');
        let dir = parts.next().unwrap();
        let count = parts.next().unwrap().parse().unwrap();
        match dir {
            "U" => Move::Up(count),
            "D" => Move::Down(count),
            "L" => Move::Left(count),
            "R" => Move::Right(count),
            _ => panic!("invalid dir."),
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/09.txt").unwrap();
    let moves: Vec<Move> = input.lines().map(|line| Move::parse(line)).collect();

    part1(&moves);
    part2(&moves);
}

fn part1(moves: &[Move]) {
    let mut tail_visited: HashSet<Pos> = HashSet::new();
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };

    for m in moves {
        match m {
            Move::Up(count) => {
                for _ in 0..*count {
                    head.y -= 1;
                    tail.chase(head);
                    tail_visited.insert(tail);
                }
            }
            Move::Down(count) => {
                for _ in 0..*count {
                    head.y += 1;
                    tail.chase(head);
                    tail_visited.insert(tail);
                }
            }
            Move::Left(count) => {
                for _ in 0..*count {
                    head.x -= 1;
                    tail.chase(head);
                    tail_visited.insert(tail);
                }
            }
            Move::Right(count) => {
                for _ in 0..*count {
                    head.x += 1;
                    tail.chase(head);
                    tail_visited.insert(tail);
                }
            }
        }
    }
    println!("part1: {}", tail_visited.len());
}

fn part2(moves: &[Move]) {
    let mut tail_visited: HashSet<Pos> = HashSet::new();
    let mut rope = [Pos { x: 0, y: 0 }; 10];

    for m in moves {
        match m {
            Move::Up(count) => {
                for _ in 0..*count {
                    rope[0].y -= 1;
                    for r in 1..rope.len() {
                        rope[r].chase(rope[r - 1]);
                    }
                    tail_visited.insert(rope[9]);
                }
            }
            Move::Down(count) => {
                for _ in 0..*count {
                    rope[0].y += 1;
                    for r in 1..rope.len() {
                        rope[r].chase(rope[r - 1]);
                    }
                    tail_visited.insert(rope[9]);
                }
            }
            Move::Left(count) => {
                for _ in 0..*count {
                    rope[0].x -= 1;
                    for r in 1..rope.len() {
                        rope[r].chase(rope[r - 1]);
                    }
                    tail_visited.insert(rope[9]);
                }
            }
            Move::Right(count) => {
                for _ in 0..*count {
                    rope[0].x += 1;
                    for r in 1..rope.len() {
                        rope[r].chase(rope[r - 1]);
                    }
                    tail_visited.insert(rope[9]);
                }
            }
        }
    }
    println!("part2: {}", tail_visited.len());
}
