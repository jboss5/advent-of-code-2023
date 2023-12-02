
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashSet;

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("Error opening file");
    BufReader::new(file).lines()
}

fn main() {
    let lines = get_lines("input.txt".to_owned());
    let mut id_list = HashSet::<i32>::new();
    let mut p2_sum = 0;
    
    for line in lines {
        let ll = line.unwrap().to_owned();
        let mut id_str = String::new();
        id_str.push_str(&ll[ll.find(' ').expect("")+1..ll.find(':').expect("")]);

        let id = id_str.parse::<i32>().expect("NaN");
        id_list.insert(id);

        let mut high_blue = 0;
        let mut high_red = 0;
        let mut high_green = 0;

        for split in ll.split([':',';']) {
            let game = split.trim();

            if game.starts_with("Game") {
                continue;
            }

            for set in game.split(',') {
                let g = set.trim();
                let cubes = g.split(' ').collect::<Vec<&str>>();
                let val = cubes[0].parse::<i32>().expect("NaN");

                match cubes[1] {
                    "red" => {
                        if val > 12 { id_list.remove(&id); }
                        high_red = val.max(high_red);
                    },
                    "green" => {
                        if val > 13 { id_list.remove(&id); }
                        high_green = val.max(high_green);
                    },
                    "blue" => {
                        if val > 14 { id_list.remove(&id); }
                        high_blue = val.max(high_blue);
                    },
                    _ => panic!("found unsupported cube")
                }
            }
        }

        p2_sum += high_blue * high_green * high_red;
    }

    let mut p1_sum = 0_i32;
    for i in id_list {
        p1_sum += i;
    }

    println!("Part 1 Sum: {}", p1_sum);
    println!("Part 2 Sum: {}", p2_sum);
}
