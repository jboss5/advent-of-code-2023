
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use hashlink::LinkedHashMap;

#[derive(Clone,Eq,PartialEq,Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |hash, ch| {
        ((hash + (ch as u32)) * 17) % 256
    })
}

#[allow(clippy::needless_range_loop)]
fn p2(filename: &String) -> u32 {
    let mut out = vec![LinkedHashMap::new(); 256];

    for l in get_lines(filename) {
        let line = l.unwrap();
        for sp in line.split(',').map(String::from) {
            if sp.contains('-') {
                let mut temp = sp.split('-');
                let lens = Lens { label: temp.next().unwrap().to_owned(), focal_length: 0 };
                let hash = hash(&lens.label) as usize;
                out[hash].remove(&lens.label);
            } else {
                let mut temp = sp.split('=');
                let lens = Lens { 
                    label: temp.next().unwrap().to_owned(), 
                    focal_length: temp.next().unwrap().parse::<u32>().unwrap() 
                };

                let label = lens.label.as_str();
                let hash = hash(label) as usize;
                out[hash].replace(label.to_string(), lens);
            }
        }
    }

    let mut sum = 0;
    for i in 0..out.len() {
        for (k,lens) in out[i].iter().enumerate() {
            // println!("adding {i} -> {k} -> entry {:?}", lens);
            sum += ((i as u32)+1) * ((k as u32)+1) * lens.1.focal_length;
        }
    }

    sum
}

#[allow(clippy::map_entry)]
fn p1(fileame: &String) -> u32 {
    let mut sum = 0;
    let mut hash_map: HashMap<String, u32> = HashMap::new();

    for l in get_lines(fileame) {
        let line = l.unwrap();
        for sp in line.split(',').map(String::from) {
            if hash_map.contains_key(&sp) {
                sum += hash_map.get(&sp).unwrap();
            } else {
                let hash = hash(&sp);
                hash_map.insert(sp, hash);
                sum += hash;
            }
        }
    }

    sum
}

fn main() {
    let filename = "input.txt".to_owned();
    println!("p1: {}", p1(&filename));
    println!("p2: {}", p2(&filename));
}
