
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn transpose(strings: &Vec<String>) -> Vec<String> {
    let mut out = vec![];
    for i in 0..strings[0].len() {
        let mut col = String::new();
        for v in strings {
            col.push(v.chars().nth(i).unwrap());
        }

        out.push(col);
    }

    out
}

fn calc_mirror_sum(rows: &Vec<String>, cols: &Vec<String>) -> u32 {
    println!("Calcing: r{}, c{}", rows.len(), cols.len());

    let mut sum = 0_u32;
    for idx in 1..rows.len() {
        if rows[idx-1] == rows[idx] {
            let mut inc = idx+1;
            let mut dec = (idx as i32)-2;
            let mut mirror = true;
            while mirror && inc < rows.len() && dec >= 0 {
                if rows[inc] != rows[dec as usize] { 
                    mirror = false; 
                }

                inc += 1;
                dec -= 1;
            }

            if mirror {
                sum += (idx as u32) *100;
                break;
            }
        }
    }

    for idx in 1..cols.len() {
        if cols[idx-1] == cols[idx] {
            let mut inc = idx+1;
            let mut dec = (idx as i32) - 2;
            let mut mirror = true;
            while mirror && inc < cols.len() && dec >= 0 {
                if cols[inc] != cols[dec as usize] { mirror = false; }

                inc += 1;
                dec -= 1;
            }
 
            if mirror {
                sum += idx as u32;
                break;
            }
        }
    }
    
    sum
}

fn p1(filename: &String) -> u32 {
    let mut row_strings = vec![];
    let mut col_strings = vec![];

    let mut sum = 0_u32;
    for l in get_lines(filename) {
        let line = l.unwrap();
        if line.is_empty() { 
            col_strings = transpose(&row_strings);
            sum += calc_mirror_sum(&row_strings, &col_strings);
            row_strings = vec![];
            col_strings = vec![];
            continue;
        } else {
            row_strings.push(line);
        }
    }

    col_strings = transpose(&row_strings);
    sum += calc_mirror_sum(&row_strings, &col_strings);

    sum
}

fn main() {
    let filename = "input.txt".to_owned();
    println!("p1: {}", p1(&filename));
}
