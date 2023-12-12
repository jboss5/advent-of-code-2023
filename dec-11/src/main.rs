
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use priority_queue::DoublePriorityQueue;

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

struct Output {
    galaxy_map: Vec<Vec<usize>>,
    galaxy_list: HashMap<usize,Coord>, // hashmap
}

fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn add_rows(vec: &mut Vec<Vec<usize>>, len: usize, count: i32) {
    for _i in 0..count {
        vec.push(vec![0; len]);
    }
}

fn compute_galaxy_map_p2(filename: &String, count: i32) -> Output {
    let mut out = Output { galaxy_map: vec![], galaxy_list: HashMap::new() };
    let mut found_galaxies = 1;

    for (_i,l) in get_lines(filename).enumerate() {
        let line = l.unwrap();
        let mut arr = vec![];
        let mut expand = true;
        for ch in line.chars() {
            if ch == '#' {
                arr.push(found_galaxies);
                found_galaxies += 1;
                expand = false;
            } else {
                arr.push(0);
            }
        }

        let len = arr.len();
        out.galaxy_map.push(arr);

        if expand { add_rows(&mut out.galaxy_map, len, count); }
    }

    let mut col_expand = vec![];
    for col in 0..out.galaxy_map[0].len() {
        let mut expand = true;
        for h in 0..out.galaxy_map.len() {
            if out.galaxy_map[h][col] != 0 { expand = false; }
        }

        if expand { col_expand.push(col); }
    }

    for (i, col) in col_expand.iter().enumerate() {
        for vec in out.galaxy_map.iter_mut() {
            for _j in 0..count {
                vec.insert(*col+(i*count as usize), 0);
            }
        }
    }

    for x in 0..out.galaxy_map.len() {
        for y in 0..out.galaxy_map[x].len() {
            let val = out.galaxy_map[x][y];
            if val != 0 { 
                out.galaxy_list.insert(val, Coord { y, x }); 
            }
        }
    }

    out
}

#[allow(dead_code)]
fn print_map(out: &Output) {
    println!("map:");
    for v in &out.galaxy_map {
        for i in v {
            print!("{i}");
        }

        println!();
    }
}

#[allow(dead_code)]
fn print_coords(out: &Output) {
    println!("coords:");
    for v in &out.galaxy_list {
        println!("num: {}, ({},{})", v.0, v.1.x, v.1.y);
    }
}

fn get_neighbors(coord: &Coord, grid: &Vec<Vec<usize>>) -> Vec<Coord> {
    let mut out = vec![];
    let x = coord.x;
    let y = coord.y;

    if y >= 1 { out.push(Coord { y: (y-1), x }); }
    if y <= grid[0].len()-2 { out.push(Coord { y: (y+1), x }); }
    if x >= 1 { out.push(Coord { y, x: (x-1) }); }
    if x <= grid.len()-2 { out.push(Coord { y, x: (x+1) }); }

    out
}

fn calc_distance(current: Coord, end_coord: Coord) -> i32 {
    i32::abs(current.x as i32 - end_coord.x as i32) + i32::abs(current.y as i32 - end_coord.y as i32)
}

/* not used, but maybe viz later */
#[allow(dead_code)]
fn shortest_path(p1: &Output) -> HashMap<(&Coord, &Coord), i32> {
    let mut out = HashMap::new();

    // loop 1 -> 9 not 0 -> 8
    for start_galaxy_num in 1..p1.galaxy_list.len()+1 {
        
        let start_coord = p1.galaxy_list.get(&start_galaxy_num).unwrap();
        // println!("== TOP {start_galaxy_num}");

        for end_galaxy_num in start_galaxy_num+1..p1.galaxy_list.len()+1 {
            // println!("== STARTING {end_galaxy_num}");
            if end_galaxy_num == start_galaxy_num { continue; }

            let mut queue: DoublePriorityQueue<Coord, i32> = DoublePriorityQueue::new();
            let mut visited = HashMap::new();
            let mut path = vec![];
            let final_coord = p1.galaxy_list.get(&end_galaxy_num).unwrap();
            queue.push(*start_coord, 1);

            while !queue.is_empty() {
                let current = queue.pop_min().unwrap();
                // println!("visiting: {:?}", current);

                path.push(current);

                if current.0 == *final_coord {
                    break;
                }

                for neighbor in get_neighbors(&current.0, &p1.galaxy_map) {
                    // println!("--- found neighbor {:?}", neighbor);
                    visited.entry(neighbor).or_insert_with(|| {
                        let priority = calc_distance(neighbor, *final_coord);
                        // println!("priority: {priority}");
                        queue.push(neighbor, priority);
                        current.0
                    });
                }
            }

            // println!("Going {:?} to {:?} with visited size of {}", start_coord, final_coord, path.len()-1);
            out.insert((start_coord,final_coord), (path.len()-1) as i32);
        }
    }

    out
}

fn man_dist(p1: Output) -> u64 {
    let mut sum = 0;

    for start_galaxy_num in 1..p1.galaxy_list.len()+1 {
        
        let start_coord = p1.galaxy_list.get(&start_galaxy_num).unwrap();
        // println!("== TOP {start_galaxy_num}");

        for end_galaxy_num in start_galaxy_num+1..p1.galaxy_list.len()+1 {
            let final_coord = p1.galaxy_list.get(&end_galaxy_num).unwrap();

            sum += calc_distance(*start_coord, *final_coord) as u64;
        }
    }

    sum
}

fn main() {
    let filename = "input.txt".to_owned();
    let mut galaxy_output = compute_galaxy_map_p2(&filename, 1);
    let sum = man_dist(galaxy_output);
    println!("p1 total sum: {sum}");


    galaxy_output = compute_galaxy_map_p2(&filename, 0);
    let sum = man_dist(galaxy_output);
    galaxy_output = compute_galaxy_map_p2(&filename, 9);
    let p2_sum = man_dist(galaxy_output);
    let mut final_sum = p2_sum;
    let mut diff = p2_sum - sum;
    for _i in 0..5 {
        diff *= 10; 
        final_sum += diff;
    }

    println!("p2 total sum: {final_sum}");
}
