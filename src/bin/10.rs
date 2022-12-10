use std::fs;

enum Op {
    Noop,
    Addx(i64),
}

impl Op {
    fn parse(s: &str) -> Self {
        if s == "noop" {
            Op::Noop
        } else if s.starts_with("addx") {
            let mut parts = s.split(' ');
            parts.next();
            let value = parts.next().unwrap().parse().unwrap();
            Op::Addx(value)
        } else {
            panic!("unexpected op.");
        }
    }
}

struct Vm {
    total: i64,
    x: i64,
    cycle: i64,
}

impl Vm {
    fn new() -> Self {
        Self { total: 0, x: 1, cycle: 0 }
    }

    fn run(&mut self, ops: &[Op], cycle_limit: i64) -> i64 {
        'outer: loop {
            for op in ops {
                if self.cycle > cycle_limit {
                    break 'outer;
                }
                match op {
                    Op::Noop => self.tick(),
                    Op::Addx(value) => {
                        self.tick();
                        self.tick();
                        self.x += value;
                    }
                }
            }
        }
        self.total
    }

    fn tick(&mut self) {
        self.cycle += 1;
        if self.cycle == 20 || (self.cycle > 20 && (self.cycle - 20) % 40 == 0) {
            self.total += self.x * self.cycle;
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/10.txt").unwrap();
    let ops: Vec<Op> = input.lines().map(|line| Op::parse(line)).collect();

    let mut vm = Vm::new();
    println!("part1: {}", vm.run(&ops, 220));
}
