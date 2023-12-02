
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part2() {
    let lines = get_lines("input.txt".to_owned());
    let mut game_power = Vec::<i64>::new();

    for line in lines {
        let ll = line.unwrap().to_owned();

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
                    "red" => if val > high_red { high_red = val; },
                    "green" => if val > high_green { high_green = val; },
                    "blue" => if val > high_blue { high_blue = val; },
                    _ => panic!("found unsupported cube")
                }
            }
        }

        println!("red: {}, blue: {}, green {}", high_red, high_blue, high_green);
        game_power.push((high_blue * high_green * high_red).into());
    }

    println!("{:?}", game_power);

    let mut sum = 0;
    for i in game_power {
        sum += i;
    }

    println!("Sum: {}", sum);
}

fn part1() {
    let lines = get_lines("input-sample.txt".to_owned());
    let mut not_possibilities = HashSet::<i32>::new();
    let mut id_list = HashSet::<i32>::new();

    for line in lines {
        let ll = line.unwrap().to_owned();
        let mut id_str = String::new();
        id_str.push_str(&ll[ll.find(' ').expect("")+1..ll.find(':').expect("")]);
        let id = id_str.parse::<i32>().expect("NaN");
        id_list.insert(id);

        for split in ll.split([':',';']) {
            let game = split.trim();

            if game.starts_with("Game") {
                continue;
            }

            for set in game.split(',') {
                let g = set.trim();
                let cubes = g.split(' ').collect::<Vec<&str>>();

                match cubes[1] {
                    "red" => if cubes[0].parse::<i32>().expect("NaN") > 12 { not_possibilities.insert(id); },
                    "green" => if cubes[0].parse::<i32>().expect("NaN") > 13 { not_possibilities.insert(id); },
                    "blue" => if cubes[0].parse::<i32>().expect("NaN") > 14 { not_possibilities.insert(id); },
                    _ => panic!("found unsupported cube")
                }
            }
        }
    }

    let mut sum = 0_i32;
    for i in not_possibilities {
        id_list.remove(&i);
    }

    for i in id_list {
        sum += i;
    }

    println!("Sum: {}", sum);
}

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("Error opening file");
    BufReader::new(file).lines()
}
