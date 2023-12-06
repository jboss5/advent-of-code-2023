
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

struct Input {
    p1: Vec<u64>,
    p2: u64
}

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn tokenize(input: String) -> Input {
    let start = input.find(':').unwrap_or(0);
    let substr = input[start+1..input.len()].trim();
    let vals = substr.split_whitespace()
        .map(|tkn| tkn.to_string().parse::<u64>().expect("NaN"))
        .collect();

    Input { p1: vals, p2: substr.replace(' ', "").parse().expect("NaN") }
}

fn compute_distances(start: u64, end: u64, cmp: u64) -> u64 {
    let mut possible_wins: u64 = 0;

    (start..end).for_each(|i| {
        let fnl_dist = (end - i) * i;
        if fnl_dist > cmp { 
            possible_wins += 1; 
        }
    });

    possible_wins
}

fn main() {
    let mut lines = get_lines("input.txt".to_owned());
    let times = tokenize(lines.next().unwrap().expect("Error reading time line"));
    let dists = tokenize(lines.next().unwrap().expect("Error reading distance line"));
    let mut p1: u64 = 1;

    for (i, time) in times.p1.iter().enumerate() {
        p1 *= compute_distances(1, *time, dists.p1[i]);
    }

    println!("Part 1 compute: {}", p1);
    println!("Part 2 compute: {}", compute_distances(1, times.p2, dists.p2));
}
