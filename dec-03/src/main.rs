
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("Error opening file");
    BufReader::new(file).lines()
}

fn main() {
    let filename = "input.txt".to_owned();
    let v = p1(filename.clone());
    p2(v, filename.clone());
}

fn p2(number_output: NumberOutput, filename: String) {
    let lines = get_lines(filename);
    let mut parts = Vec::<Vec<char>>::new();
    let mut stars = Vec::<Star>::new();

    lines.for_each(|l| {
        parts.push(l.unwrap().trim().to_owned().chars().collect());
    });  

    for (i, item) in parts.iter().enumerate() {
        for (i2, ch) in item.iter().enumerate() {
            if ch == &'*' {
                stars.push(Star { y: i, x: i2, numbers: Vec::<Number>::new() });
            }
        }
    }

    let mut sum: u128 = 0;
    for star in stars {
        let mut found_parts = Vec::<Number>::new();
        let star_x = star.x as i32;
        let star_y = star.y as i32;

        for part in &number_output.out_numbers  {
            let part_y = part.y as i32;
            let part_end = part.end as i32;
            let part_start = part.start as i32;

            if part_y == star_y-1 || part_y == star_y+1 {
                if star_x >= part_start-1 && star_x <= part_end+1 {
                    found_parts.push(Number { y: part.y, start: part.start, end: part.end });
                }
            } else if part_y == star_y && (star_x == part_start-1 || star_x == part_end+1) {
                found_parts.push(Number { y: part.y, start: part.start, end: part.end });
            }
        }

        if found_parts.len() == 2 {
            let num1 = build_number(&number_output.out_parts, &found_parts[0]);
            let num2 = build_number(&number_output.out_parts, &found_parts[1]);
            sum += u128::from(num1 * num2);
        }
    }

    println!("sum {}", sum);
}

fn p1(filename: String) -> NumberOutput {
    let lines = get_lines(filename);
    let mut parts = Vec::<Vec<char>>::new();

    lines.for_each(|l| {
        parts.push(l.unwrap().trim().to_owned().chars().collect());
    });  

    let mut out_parts = Vec::<Number>::new();
    for (i, item) in parts.iter().enumerate() {
        let mut is_part = false;
        let y = i as i32;
        let mut start_idx: usize = 0;

        for (i2, ch) in item.iter().enumerate() {
            let x = i2 as i32;
            if !ch.is_numeric() { 
                if is_part { 
                    let n = Number { y:i, start: start_idx, end: i2-1};
                    out_parts.push(n); 
                    is_part = false;
                }
                
                start_idx = i2+1;
                continue; 
            }

            if check_index(&parts,y-1,x-1) { is_part = true; }
            if check_index(&parts,y-1,x) { is_part = true; }
            if check_index(&parts,y-1,x+1) { is_part = true; }
            if check_index(&parts,y+1,x-1) { is_part = true; }
            if check_index(&parts,y+1,x) { is_part = true; }
            if check_index(&parts,y+1,x+1) { is_part = true; }
            if check_index(&parts, y, x-1) { is_part = true; }
            if check_index(&parts, y, x+1) { is_part = true; }
        }

        if is_part {
            out_parts.push(Number { y:i, start: start_idx, end: item.len() }); 
        } else {
            let x = item.len() as i32;
            if item[item.len()-1].is_numeric() {
                if check_index(&parts,y-1,x) { is_part = true; }
                if check_index(&parts,y+1,x) { is_part = true; }
                if check_index(&parts,y+1,x-1) { is_part = true; }
                if check_index(&parts,y-1,x-1) { is_part = true; }
                if check_index(&parts, y, x-1) { is_part = true; }
    
                if is_part {
                    out_parts.push(Number { y:i, start: start_idx, end: item.len() }); 
                }
            }
        }
    }

    let mut output = Vec::<u64>::new();

    out_parts.iter().for_each(|out| {
        let v = &parts[out.y];
        let end = out.end.max(v.len());
        let v2: Vec<char> = parts[out.y][out.start..end].to_vec();          
        let mut str = String::new();

        for (_i, ch) in v2.iter().enumerate() {
            if !ch.is_numeric() { 
                if !str.is_empty() { break; }
                else {  continue; }
            }

            str.push(ch.to_owned());
        }

        output.push(str.parse::<u64>().unwrap());
    });

    println!("Part 1: {}", output.iter().sum::<u64>());
    NumberOutput { out_parts: parts, out_numbers: out_parts }
}

fn build_number(arr: &Vec<Vec<char>>, number: &Number) -> u64 {
    let v = &arr[number.y];
    let end = number.end.max(v.len());
    let v2: Vec<char> = v[number.start..end].to_vec();            
    let mut str = String::new();

    for ch in v2 {
        if !ch.is_numeric() { 
            if !str.is_empty() { break; }
            else { continue; }
        }

        str.push(ch.to_owned());
    }

    str.parse::<u64>().unwrap()
}

fn is_char_symbol(ch: char) -> bool {
    ch != '.' && !ch.is_numeric()
}

fn check_index(arr: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    if y < 0 || x < 0 || y >= arr.len() as i32 || x >= arr[y as usize].len() as i32 { return false; }
    is_char_symbol(arr[y as usize][x as usize])
}

#[derive(Copy, Clone)]
struct Number {
    y: usize,
    start: usize,
    end: usize
}

struct Star {
    y: usize,
    x: usize,
    numbers: Vec<Number>
}

struct NumberOutput {
    out_parts: Vec<Vec<char>>,
    out_numbers: Vec<Number>
}