
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Tile {
    coord: (i32, i32),
    direction: Direction,
}

fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn build_grid(filename: &String) -> Vec<Vec<char>> {
    let mut out = vec![];
    for l in get_lines(filename) {
        out.push(l.unwrap().chars().collect());
    }

    out
}

fn get_tiles(grid: &mut Vec<Vec<char>>, start: (i32,i32), power_grid: &mut Vec<Vec<char>>, direction: Direction) -> u32 {
    if start.0 < 0 || start.0 >= grid.len() as i32 { return 0; }
    if start.1 < 0 || start.1 >= grid[0].len() as i32 { return 0; }

    let s = power_grid[start.0 as usize][start.1 as usize];
    if direction == Direction::East && s == '>' { return 0; }
    if direction == Direction::West && s == '<' { return 0; }
    if direction == Direction::North && s == '^' { return 0; }
    if direction == Direction::South && s == 'v' { return 0; }

    let mut sum = 0;
    let mut queue = vec![];
    queue.push(Tile { coord: start, direction });

    while let Some(current) = queue.pop() {
        if current.coord.0 < 0 || current.coord.0 >= grid.len() as i32 { continue; }
        if current.coord.1 < 0 || current.coord.1 >= grid[0].len() as i32 { continue; }

        let ch = grid[current.coord.0 as usize][current.coord.1 as usize];
        match ch {
            '|' => {
                match current.direction {
                    Direction::East | Direction::West => {
                        get_tiles(grid, (current.coord.0+1,current.coord.1), power_grid, Direction::South);
                        get_tiles(grid, (current.coord.0-1,current.coord.1), power_grid, Direction::North);
                    },
                    Direction::North => { queue.push(Tile { coord: (current.coord.0-1, current.coord.1), direction: Direction::North }); },
                    Direction::South => { queue.push(Tile { coord: (current.coord.0+1, current.coord.1), direction: Direction::South }); },
                }
            }
            '/' => {
                match current.direction {
                    Direction::East => { queue.push(Tile { coord: (current.coord.0-1, current.coord.1), direction: Direction::North }); },
                    Direction::West => { queue.push(Tile { coord: (current.coord.0+1, current.coord.1), direction: Direction::South }); },
                    Direction::North => { queue.push(Tile { coord: (current.coord.0, current.coord.1+1), direction: Direction::East }); },
                    Direction::South => { queue.push(Tile { coord: (current.coord.0, current.coord.1-1), direction: Direction::West }); },
                }
            },
            '\\' => {
                match current.direction {
                    Direction::East => { queue.push(Tile { coord: (current.coord.0+1, current.coord.1), direction: Direction::South }); },
                    Direction::West => { queue.push(Tile { coord: (current.coord.0-1, current.coord.1), direction: Direction::North }); },
                    Direction::North => { queue.push(Tile { coord: (current.coord.0, current.coord.1-1), direction: Direction::West }); },
                    Direction::South => { queue.push(Tile { coord: (current.coord.0, current.coord.1+1), direction: Direction::East }); },
                }
            },
            '-' => {
                match current.direction {
                    Direction::East => { queue.push(Tile { coord: (current.coord.0, current.coord.1+1), direction: Direction::East }); },
                    Direction::West => { queue.push(Tile { coord: (current.coord.0, current.coord.1-1), direction: Direction::West }); },
                    _ => {
                         get_tiles(grid, (current.coord.0,current.coord.1+1), power_grid, Direction::East);
                         get_tiles(grid, (current.coord.0,current.coord.1-1), power_grid, Direction::West);
                    },
                }
            },
            '.' | '#' => {
                match current.direction {
                    Direction::East => { queue.push(Tile { coord: (current.coord.0, current.coord.1+1), direction: Direction::East }); },
                    Direction::West => { queue.push(Tile { coord: (current.coord.0, current.coord.1-1), direction: Direction::West }); },
                    Direction::North => { queue.push(Tile { coord: (current.coord.0-1, current.coord.1), direction: Direction::North }); },
                    Direction::South => { queue.push(Tile { coord: (current.coord.0+1, current.coord.1), direction: Direction::South }); },
                }
            },
            _ => panic!("nope")
        }

        let new_ch = match current.direction {
            Direction::East => '>',
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
        };

        power_grid[current.coord.0 as usize][current.coord.1 as usize] = new_ch;
    }

    for h in 0..power_grid.len() {
        for w in 0..power_grid[h].len() {
            let ch = power_grid[h][w];
            match ch {
                '^' | 'v' | '>' | '<' => sum +=1,
                _ => {}
            }
        }
    }

    sum
}

fn main() {
    let filename = "input.txt".to_owned();
    let orig_grid = build_grid(&filename);
    let mut power_grid = orig_grid.clone();
    let mut grid = orig_grid.clone();
    let tiles = get_tiles(&mut grid, (0,0), &mut power_grid, Direction::East);
    println!("p1: {tiles}");

    let mut p2_highest = 0;

    // r -> east
    for r in 0..orig_grid.len() {
        p2_highest = p2_highest.max(get_tiles(&mut orig_grid.clone(), (r as i32,0), &mut orig_grid.clone(), Direction::East));
    }

    // r.len() -> west
    for r in (0..orig_grid.len()-1).rev() {
        p2_highest = p2_highest.max(get_tiles(&mut orig_grid.clone(), (r as i32,(orig_grid[0].len()-1) as i32), &mut orig_grid.clone(), Direction::West));
    }

    // col -> south
    for c in 0..orig_grid[0].len() {
        p2_highest = p2_highest.max(get_tiles(&mut orig_grid.clone(), (0,c as i32), &mut orig_grid.clone(), Direction::South));
    }

    // col.len() -> north
    for c in (0..orig_grid[0].len()-1).rev() {
        p2_highest = p2_highest.max(get_tiles(&mut orig_grid.clone(), ((orig_grid.len()-1) as i32,c as i32), &mut orig_grid.clone(), Direction::North));
    }

    println!("p2: {p2_highest}");
}
