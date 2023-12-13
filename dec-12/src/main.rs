
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::sync::atomic::AtomicI32;

use itertools::{Itertools, repeat_n};
use regex::Regex;

fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn p1(filename: &String) -> u64 {
    // let mut out = HashMap::new();

    // for l in get_lines(filename) {
    //     let line = l.unwrap();
    //     let space = line.find(' ').unwrap();
    //     let input = line[0..space].to_string();
    //     let guide = line[space+1..line.len()]
    //         .split(',')
    //         .map(|ch| ch.to_string().parse::<u32>().unwrap())
    //         .collect::<Vec<u32>>();

    //     let mut count: u64 = 0;

    //     out.insert(line, count);
    // }

    // out.iter()
    // .fold(0, |mut k, v| { 
    //     k += *v.1; 
    //     k
    // })

    1
}

fn main() { 
    let filename = "input-sample.txt".to_owned();
    // let p1_sum = p1(&filename);
    // let v = vec!['?','#','#','#','?','?','?','?','?','?','?','?'];
    let v = vec!['#','.'];
    // let v = vec!['?','?','?'];
    // for p in v.iter().permutations(3).unique() {
    //     println!("{:?}", p);
    // }

    // ###.
    // .###.
    // .###

    // ##.
    // .##.
    // .##

    // #.
    // .#.
    // .#

    // ^\.?###\.+##\.+#\.*$

    //?###???????? 3,2,1

    // 1. split on whitespace
    // 2. turn 1st index into array of what we expect
    // 3. build regular expression from those numbers
    // 4. run below logic

    // let mut sum = 0;
    let mut sum = AtomicI32::new(0);

    // let s = "?###???????? 3,2,1".to_owned();

    for l in get_lines(&filename) {
        let s = l.unwrap();
        let space = s.find(' ').unwrap();
        let input = s[0..space].to_string();
        let mut guide = s[space+1..s.len()]
            .split(',')
            .map(|ch| ch.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut given_hashes = vec![];
        for (i,ch) in input.chars().enumerate() {
            if ch != '?' {
                given_hashes.push((i,ch));
            }
        }

        let mut regex = r"^\.*".to_string();
        let len = guide.len();
        let end = r"$";
        for (i, num) in guide.iter_mut().enumerate() {
            regex.push_str("[#]{");
            regex.push_str(&num.to_string());
            if i == len-1 { regex.push_str(r"}\.*"); }
            else { regex.push_str(r"}\.+"); }
        }

        regex.push_str(end);

        println!("{regex}");

        // let re = Regex::new(r"^\.*[#]{3}\.+[#]{2}\.+[#]{1}\.*$").unwrap();
        let re = Regex::new(&regex).unwrap();
        
        println!("input: {input}");

        for p in repeat_n(v.iter(),input.len()).multi_cartesian_product() {
            // let s = p.iter().map(|p1| **p1).collect();
            let str = String::from_iter(p);
            // println!("{str}");

            // if re.is_match(&str) && vec![3,2,1] == get_hashes(&str) {



            if re.is_match(&str) && guide == str.split('.')
                                                        .filter(|s| !s.is_empty())
                                                        .map(|s| s.len() as u32)
                                                        .collect::<Vec<u32>>()
                                            && hashes_match(&given_hashes, str.clone()) {

                println!("{:?}", str);
                sum.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    println!("p1: {}", sum.get_mut());
}

fn hashes_match(given: &Vec<(usize,char)>, str: String) -> bool {

    for idx in given {
        let ch = str.chars().nth(idx.0).unwrap();
        
        if idx.1 != ch { return false; }
    }

    true
}

fn get_hashes(str: &str) -> Vec<usize> {
    let mut v = vec![];

    for s in str.split('.') {
        if !s.is_empty() { v.push(s.len()); }
    }

    v
}

