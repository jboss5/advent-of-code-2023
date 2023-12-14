
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn p2(grid: &mut Vec<Vec<char>>, cycles: usize) -> u64 {
    let mut sum = 0;

    for _i in 0..cycles {
        // north
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] != 'O' { continue; }

                let mut idx = (row as i32)-1;
                while idx >= 0 {
                    let ix = idx as usize;
                    if grid[ix][col] == '.' {
                        grid[ix+1][col] = '.';
                        grid[ix][col] = 'O';
                    } else {
                        break;
                    }

                    idx -= 1;
                }
            }
        }

        //west
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] != 'O' { continue; }

                let mut idx = (col as i32)-1;
                while idx >= 0 {
                    let ix = idx as usize;
                    if grid[row][ix] == '.' {
                        grid[row][ix+1] = '.';
                        grid[row][ix] = 'O';
                    } else {
                        break;
                    }

                    idx -= 1;
                }
            }
        }

        //south
        for row in (0..grid.len()).rev() {
            for col in (0..grid[row].len()).rev() {
                if grid[row][col] != 'O' { continue; }

                let mut idx = row+1;
                while idx < grid.len() {
                    if grid[idx][col] == '.' {
                        grid[idx-1][col] = '.';
                        grid[idx][col] = 'O';
                    } else {
                        break;
                    }

                    idx += 1;
                }
            }
        }

        //east
        for row in (0..grid.len()).rev() {
            for col in (0..grid[row].len()).rev() {
                if grid[row][col] != 'O' { continue; }

                let mut idx = col+1;
                while idx < grid[row].len() {
                    if grid[row][idx] == '.' {
                        grid[row][idx-1] = '.';
                        grid[row][idx] = 'O';
                    } else {
                        break;
                    }

                    idx += 1;
                }
            }
        }
    }

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            // print!("{}", grid[row][col]);
            if grid[row][col] == 'O' {
                sum += (grid.len() - row) as u64;
            }
        }
        // println!();
    }
    // println!();

    sum
}

fn p1(grid: &mut Vec<Vec<char>>) -> u64 {
    let mut sum = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] != 'O' { continue; }

            let mut idx = (row as i32)-1;
            while idx >= 0 {
                let ix = idx as usize;
                if grid[ix][col] == '.' {
                    grid[ix+1][col] = '.';
                    grid[ix][col] = 'O';
                } else {
                    break;
                }

                idx -= 1;
            }
        }
    }

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            // print!("{}", grid[row][col]);
            if grid[row][col] == 'O' {
                sum += (grid.len() - row) as u64;
            }
        }
        // println!();
    }
    // println!();

    sum
}

fn main() {
    let filename = "input-sample.txt".to_owned();
    let mut grid = vec![];
    for l in get_lines(&filename) {
        let line = l.unwrap();
        grid.push(line.chars().collect());
    }

    println!("p1: {}", p1(&mut grid.clone()));

    // the 1,000,000,000 answer "cycles" every 7 cycles, 1000 == a "7th" cycle
    // maybe code this later, detect when the cycle has repeated itself and that's your number
    // just lucky that 1000 = 7th cycle
    println!("p2: {}", p2(&mut grid.clone(), 1000));
}
