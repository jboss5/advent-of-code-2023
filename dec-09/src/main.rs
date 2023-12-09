
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
struct Output {
    p1: i64,
    p2: i64,
}

fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn contains_all_zeroes(list: &Vec<i64>) -> bool {
    for num in list {
        if *num != 0 { return false; }
    }

    true
}

fn runit(filename: &String) -> Output {
    let mut out = Output { p1: 0, p2: 0 };
    for l in get_lines(filename) {
        let line = l.unwrap();
        let history: Vec<i64> = line.split_whitespace()
            .map(|s| s.to_string().parse::<i64>().expect("NaN"))
            .collect();

        let mut diff_list: Vec<i64> = vec![1; 1];
        let mut tracker: Vec<Vec<i64>> = vec![history];

        while !contains_all_zeroes(&diff_list) {
            let current = tracker.last().expect("array error");
            diff_list = vec![];
            for i in 0..current.len()-1 {
                diff_list.push(current[i+1]-current[i]);
            }

            tracker.push(diff_list.clone());
        }

        for i in (0..tracker.len()-1).rev() {
            if i == tracker.len()-1 {
                tracker[i].push(0);
                tracker[i].insert(0,0);
            } else {
                let ex_val_p1 = tracker[i].last().expect("NaN") + tracker[i+1].last().expect("NaN");
                let ex_val_p2 = tracker[i].first().expect("NaN") - tracker[i+1].first().expect("NaN");
                tracker[i].push(ex_val_p1);
                tracker[i].insert(0, ex_val_p2);
            }
        }

        out.p1 += tracker[0].last().expect("cannot find base #");
        out.p2 += tracker[0].first().expect("cannot find first val");
    }

    out
}

fn main() {
    println!("results: {:?}", runit(&"input.txt".to_owned()));
}
