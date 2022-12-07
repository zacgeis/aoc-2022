use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/05.txt").unwrap();
    let mut iter = input.lines();

    let mut stacks: Vec<Vec<char>> = vec![];
    let mut instructions: Vec<(u32, usize, usize)> = vec![];
    for _ in 0..9 {
        stacks.push(vec![]);
    }
    for _ in 0..8 {
        let line = iter.next().unwrap();
        let chars: Vec<char> = line.chars().collect();
        for i in 0..9 {
            match chars[1 + i * 4] {
                ' ' => continue,
                c => {
                    stacks[i].push(c);
                }
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    iter.next();
    iter.next();
    for line in iter {
        let chunks = line.split(' ').collect::<Vec<&str>>();
        let count: u32 = chunks[1].parse().unwrap();
        let from = chunks[3].parse::<usize>().unwrap() - 1;
        let to = chunks[5].parse::<usize>().unwrap() - 1;
        instructions.push((count, from, to));
    }

    part1(stacks.clone(), &instructions);
    part2(stacks, &instructions);
}

fn part1(mut stacks: Vec<Vec<char>>, instructions: &Vec<(u32, usize, usize)>) {
    for (count, from, to) in instructions {
        for _ in 0..*count {
            let temp = stacks[*from].pop().unwrap();
            stacks[*to].push(temp);
        }
    }
    print!("part1: ");
    for stack in &stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}

fn part2(mut stacks: Vec<Vec<char>>, instructions: &Vec<(u32, usize, usize)>) {
    for (count, from, to) in instructions {
        let mut hold = vec![];
        for _ in 0..*count {
            hold.push(stacks[*from].pop().unwrap());
        }
        for _ in 0..*count {
            stacks[*to].push(hold.pop().unwrap());
        }
    }
    print!("part2: ");
    for stack in &stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}
