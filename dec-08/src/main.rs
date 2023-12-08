
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn get_path(path: String) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    for ch in path.chars() {
        match ch {
            'R' => out.push(1),
            'L' => out.push(0),
            _ => panic!("Invalid path")
        }
    }

    out
}

fn p1(input: &Input) -> u32 {
    let start = "AAA".to_string();
    let end = "ZZZ".to_string();

    println!("path: {:?}", input.path);
    println!("start: {start}, end: {end}");

    let mut idx = 0;
    let mut current = &start;
    while current != &end {
        let p = input.path[idx % input.path.len()];
        current = &input.maps.get(current).expect("did not find path")[p];
        idx += 1;
    }

    idx as u32
}

fn find_steps_to_z(dest_map: &HashMap<String, Vec<String>>, start: &String, path: &Vec<usize>) -> u32 {
    let mut idx = 0;
    let mut current = start;
    while !current.ends_with('Z') {
        let p = path[idx % path.len()];
        current = &dest_map.get(current).expect("did not find path")[p];
        idx += 1;
    }

    idx as u32
}

fn p2(input: &Input) -> u64 {
    let starts: Vec<_> = input.maps.keys()
        .filter(|k| k.ends_with('A'))
        .collect();

    println!("starts: {:?}", &starts);
    starts.iter()
        .map(|current| find_steps_to_z(&input.maps, current, &input.path) as u64)
        .fold(1, num_integer::lcm)
}

struct Input {
    path: Vec<usize>,
    maps: HashMap<String, Vec<String>>,
}

fn parse_file(filename: &String) -> Input {
    let mut lines = get_lines(filename.to_string());
    let path = get_path(lines.next().expect("No line found").unwrap());
    lines.next(); // blank line
    let mut maps: HashMap<String, Vec<String>> = HashMap::new();

    for l in lines {
        let line = l.unwrap();
        let key = line[0..line.find(" = ").expect("line error")].to_string();
        let dests: Vec<String> = line[line.find('(').expect("line error")+1..line.len()-1]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        maps.insert(key, dests);
    }

    Input { path, maps }
}

fn main() {
    let filename = "input.txt".to_owned();
    let input = parse_file(&filename);

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
