use std::fs;

struct Cell {
    height: u8,
    visible: bool,
    view_score: i32,
}

struct Grid {
    data: Vec<Cell>,
    width: i32,
    height: i32,
}

impl Grid {
    fn get(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(&mut self.data[(y * self.width as i32 + x) as usize])
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/08.txt").unwrap();
    let mut width = 0;
    let mut height = 0;
    let mut data = vec![];
    for line in input.lines() {
        for c in line.chars() {
            data.push(Cell {
                height: c.to_digit(10).unwrap() as u8,
                visible: false,
                view_score: 0,
            });
        }
        if width == 0 {
            width = data.len() as i32;
        }
        height += 1;
    }

    let mut grid = Grid {
        data,
        width,
        height,
    };

    part1(&mut grid);
    part2(&mut grid);
}

fn part1(grid: &mut Grid) {
    for x in 0..grid.width {
        mark_line(grid, x, 0, 0, 1);
        mark_line(grid, x, grid.height - 1, 0, -1);
    }
    for y in 0..grid.height {
        mark_line(grid, 0, y, 1, 0);
        mark_line(grid, grid.width - 1, y, -1, 0);
    }

    let total = grid.data.iter().filter(|cell| cell.visible).count();
    println!("part1: {:?}", total);
}

fn mark_line(grid: &mut Grid, mut x: i32, mut y: i32, dx: i32, dy: i32) {
    let mut last_height = None;
    while let Some(cell) = grid.get(x, y) {
        if last_height.is_none() || cell.height > last_height.unwrap() {
            cell.visible = true;
            last_height = Some(cell.height);
        }
        x += dx;
        y += dy;
    }
}

fn part2(grid: &mut Grid) {
    for x in 0..grid.width {
        for y in 0..grid.height {
            let view_score = view(grid, x, y, 1, 0)
                * view(grid, x, y, -1, 0)
                * view(grid, x, y, 0, 1)
                * view(grid, x, y, 0, -1);
            grid.get(x, y).unwrap().view_score = view_score;
        }
    }
    let max_view_score = grid.data.iter().map(|cell| cell.view_score).max().unwrap();
    println!("part2: {}", max_view_score);
}

fn view(grid: &mut Grid, mut x: i32, mut y: i32, dx: i32, dy: i32) -> i32 {
    let height = grid.get(x, y).unwrap().height;
    let mut total = 0;
    loop {
        x += dx;
        y += dy;
        match grid.get(x, y) {
            None => break,
            Some(cell) => {
                total += 1;
                if cell.height >= height {
                    break;
                }
            }
        }
    }
    total
}
