use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct DigEntry {
    direction: Direction,
    distance: usize,
    rgb: String
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Coord {
    x: i64,
    y: i64
}

fn get_input(filename: &String) -> Vec<DigEntry> {
    let mut out = Vec::<DigEntry>::new();
    for l in BufReader::new(File::open(filename).expect("Error opening file")).lines() {
        let inp = l.unwrap();
        let line = inp.split(' ').collect::<Vec<&str>>();
        out.push(DigEntry { 
            direction:     
                match line[0] {
                    "R" => Direction::EAST,
                    "L" => Direction::WEST,
                    "D" => Direction::SOUTH,
                    "U" => Direction::NORTH,
                    _ => panic!("invalid input: {}", line[0]),
                }, 
            distance: line[1].to_owned().parse::<usize>().expect("invalid distance in input"), 
            rgb: line[2].to_owned().to_string() 
        });
    }

    out
}

fn process(dig_info: &Vec<DigEntry>) -> Vec<Coord> {
    let mut grid: Vec<Coord> = vec![];
    let mut x = 0_i64;
    let mut y = 0_i64;

    for entry in dig_info {
        match entry.direction {
            Direction::NORTH => y += entry.distance as i64,
            Direction::SOUTH => y -= entry.distance as i64,
            Direction::EAST => x += entry.distance as i64,
            Direction::WEST => x -= entry.distance as i64,
        }

        grid.push(Coord { x, y });
    }

    grid
}

fn correct_dig(grid: &Vec<DigEntry>) -> Vec<DigEntry> {
    let mut out: Vec<DigEntry> = vec![];
    for entry in grid {
        let fixed_rgb = entry.rgb.clone()[2..entry.rgb.len()-1].to_string();
        let hex = &fixed_rgb[0..fixed_rgb.len()-1].to_string();
        let dir = fixed_rgb.clone().pop().unwrap();
        let new_distance = i64::from_str_radix(hex.as_str(), 16).unwrap();
        let new_dir = match dir {
            '0' => Direction::EAST,
            '1' => Direction::SOUTH,
            '2' => Direction::WEST,
            '3' => Direction::NORTH,
            _ => panic!("invalid direction {}", dir)
        };

        out.push(DigEntry { rgb: String::new(), direction: new_dir, distance: new_distance as usize });
    }

    out
}

fn calc_area(coords: &Vec<Coord>) -> i64 {
    let mut area = 0_i64;
    for i in 0..coords.len() {
        let x_calc = coords[i].x * coords[(i+1)%coords.len()].y;
        let y_calc = coords[i].y * coords[(i+1)%coords.len()].x;
        area += x_calc - y_calc;
    }

    area
}

fn p1(dig_info: &Vec<DigEntry>) -> i64 {
    let mut coords = process(&dig_info);
    let mut outside = 0_usize;
    dig_info.iter().for_each(|e| outside += e.distance);
    coords.reverse();

    ((outside as i64 + calc_area(&coords))/2)+1
}

fn p2(dig_info: &Vec<DigEntry>) -> i64 {
    let corrected_dig_info = correct_dig(&dig_info);
    let mut coords = process(&corrected_dig_info);
    let mut outside = 0_usize;
    corrected_dig_info.iter().for_each(|e| outside += e.distance);
    coords.reverse();

    ((outside as i64 + calc_area(&coords))/2)+1
}

fn main() {
    let filename = "input.txt".to_owned();
    let dig_info = get_input(&filename);
    println!("Part1: {}\nPart2: {}", p1(&dig_info), p2(&dig_info));
}
