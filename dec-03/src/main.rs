
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

    for l in lines {
        let mut line = l.unwrap().to_owned();
        line = line.trim().to_owned();
        let mut depth = Vec::<char>::new();
        for ch in line.chars() {
            depth.push(ch);
        }

        parts.push(depth);
    }

    for (i, item) in parts.iter().enumerate() {
        let y: i32 = TryFrom::try_from(i).expect("NaN");

        for (i2, ch) in item.iter().enumerate() {
            if ch == &'*' {
                stars.push(Star { y: i, x: i2, numbers: Vec::<Number>::new() });
            }
        }
    }

    let mut sum: u128 = 0;
    for star in stars {
        let mut found_parts = Vec::<Number>::new();
        let star_x: i32 = TryFrom::try_from(star.x).expect("NaN");
        let star_y: i32 = TryFrom::try_from(star.y).expect("NaN");

        for part in &number_output.out_numbers  {
            let part_y: i32 = TryFrom::try_from(part.y).expect("NaN");
            let part_end: i32 = TryFrom::try_from(part.end).expect("NaN");
            let part_start: i32 = TryFrom::try_from(part.start).expect("NaN");

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

    for l in lines {
        let mut line = l.unwrap().to_owned();
        line = line.trim().to_owned();
        let mut depth = Vec::<char>::new();
        for ch in line.chars() {
            depth.push(ch);
        }

        parts.push(depth);
    }

    let mut out_parts = Vec::<Number>::new();
    for (i, item) in parts.iter().enumerate() {
        let mut is_part = false;
        let y: i32 = TryFrom::try_from(i).expect("NaN");
        let mut start_idx: usize = 0;

        for (i2, ch) in item.iter().enumerate() {
            let x: i32 = TryFrom::try_from(i2).expect("NaN");
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
            let n = Number { y:i, start: start_idx, end: item.len() };
            out_parts.push(n); 
        } else {
            let x: i32 = TryFrom::try_from(item.len()).expect("NaN");
            if item[item.len()-1].is_numeric() {
                if check_index(&parts,y-1,x) { is_part = true; }
                if check_index(&parts,y+1,x) { is_part = true; }
                if check_index(&parts,y+1,x-1) { is_part = true; }
                if check_index(&parts,y-1,x-1) { is_part = true; }
                if check_index(&parts, y, x-1) { is_part = true; }
    
                if is_part {
                    let n = Number { y:i, start: start_idx, end: item.len() };
                    out_parts.push(n); 
                }
            }
        }
    }

    let mut sum: u64 = 0;
    let mut output = Vec::<u64>::new();

    for out in &out_parts {
        let v = parts.get(out.y).expect("OB");
        let end = out.end.max(v.len());
        let v2: Vec<char> = v[out.start..end].to_vec();            
        let mut str = String::new();

        for (_i, ch) in v2.iter().enumerate() {
            if !ch.is_numeric() { 
                if !str.is_empty() { break; }
                else {  continue; }
            }

            str.push(ch.to_owned());
        }

        output.push(str.parse::<u64>().unwrap());
    }

    for num in output {
        sum += num;
    }

    println!("Part 1: {}", sum);
    NumberOutput { out_parts: parts, out_numbers: out_parts }
}

fn build_number(arr: &Vec<Vec<char>>, number: &Number) -> u64 {
    let v = arr.get(number.y).expect("OB");
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
    let vec1 = arr.get(y as usize).expect("out of bounds");
    let ch = vec1.get(x as usize).expect("out of bounds");
    is_char_symbol(*ch)
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