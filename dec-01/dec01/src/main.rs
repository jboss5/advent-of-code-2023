
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let p1_sum = part1("input-sample.txt".to_owned());
    println!("Part 1 sum: {}", p1_sum);

    let p2_sum = part2("input-p2.txt".to_owned());
    println!("Part 2 sum: {}", p2_sum);
}

fn get_num(d1: char, d2: char) -> i64 {
    let mut str = String::new();
    str.push(d1);
    str.push(d2);

    str.to_string().parse::<i64>().unwrap()
}

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("Error opening file");
    BufReader::new(file).lines()
}

fn part1(filename: String) -> i64 {
    let mut sum = 0_i64;
    
    for line in get_lines(filename.to_string()) {
        sum += sum_digits(line.unwrap());
    }

    sum
}

fn sum_digits(digit_str: String) -> i64 {
    let mut digits = Vec::new();
    let mut sum = 0_i64;

    for ch in digit_str.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        }
    }

    match digits.len() {
        1 => sum += get_num(digits[0], digits[0]),
        2 => sum += get_num(digits[0], digits[1]),
        _ => sum += get_num(digits[0], digits[digits.len()-1])
    }

    sum
}

fn part2(filename: String) -> i64 {
    let mut sum = 0_i64;

    for line in get_lines(filename.to_string()) {
        let line_str = line.unwrap();
        let ll = get_text_digits(line_str.to_owned());
        sum += sum_digits(ll);
    }

    sum    
}

fn contains_digit(substr: &str) -> char {
    let p2_values: HashMap<&str, char> = [("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
                                         ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')]
                        .iter()
                        .cloned()
                        .collect();

    for (k,v) in p2_values.into_iter() {
        if substr.contains(k) {
            return v;
        }
    }

    'E'
}

fn get_text_digits(line: String) -> String {
    let mut new_line = line.clone();

    // eighthree
    // sevenine

    // rust std::String.insert does a replace if it's at string's capacity
    // hackily add a -1/+1 to start/end in order to not overwrite some combos
    for i in 1..new_line.len() {
        let substr = &new_line[0..i];
        
        let repl = contains_digit(substr);
        if repl != 'E' {
            new_line.insert(i-1, repl);
            break;
        }
    }

    for i in (1..new_line.len()).rev() {
        let substr = &new_line[i..new_line.len()];

        let repl = contains_digit(substr);
        if repl != 'E' {
            new_line.insert(i+1, repl);
            break;
        }
    }
    
    new_line
}