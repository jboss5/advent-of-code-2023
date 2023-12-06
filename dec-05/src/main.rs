
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::sync::atomic::AtomicU64;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug,PartialEq)]
enum Category {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumdity,
    HumidityToLocation
}

#[derive(Debug)]
struct SeedMap {
    dest: u64,
    src: u64,
    offset: u64
}

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn get_category(string: &String) -> Category {
    match string {
        line if line.starts_with("seeds:") => Category::Seeds,
        line if line.starts_with("seed-to-soil ") => Category::SeedToSoil,
        line if line.starts_with("soil-to-fertilizer ") => Category::SoilToFertilizer,
        line if line.starts_with("fertilizer-to-water ") => Category::FertilizerToWater,
        line if line.starts_with("water-to-light ") => Category::WaterToLight,
        line if line.starts_with("light-to-temperature ") => Category::LightToTemp,
        line if line.starts_with("temperature-to-humidity ") => Category::TempToHumdity,
        line if line.starts_with("humidity-to-location ") => Category::HumidityToLocation,
        _ => panic!("invalid line")
    }
}

fn tokenize(line: &str) -> SeedMap {
    let tokenized: Vec<&str> = line.split(' ').collect();
    let dest = tokenized[0].parse::<u64>().expect("NaN");
    let src = tokenized[1].parse::<u64>().expect("NaN");
    let offset = tokenized[2].parse::<u64>().expect("NaN");
    SeedMap { dest, src, offset }
}

fn compute_offset(map: &Vec<SeedMap>, src: u64) -> u64 {
    for item in map {
        let src_range = item.src..(item.src+item.offset);

        if src_range.contains(&src) {
            return (src-item.src)+item.dest;
        }
    }

    src
}

fn main() {
    let filename = "input.txt";
    let mut seed_list_p1 = Vec::<u64>::new();
    let mut seed_list_p2 = Vec::<u64>::new();
    let mut seed_to_soil = Vec::<SeedMap>::new();
    let mut soil_to_fert = Vec::<SeedMap>::new();
    let mut fert_to_water = Vec::<SeedMap>::new();
    let mut water_to_light = Vec::<SeedMap>::new();
    let mut light_to_temp = Vec::<SeedMap>::new();
    let mut temp_to_humd = Vec::<SeedMap>::new();
    let mut humd_to_location = Vec::<SeedMap>::new();

    let mut lines = get_lines(filename.to_owned()).peekable();
    while let Some(l) = lines.next() {
        let mut line = l.unwrap();
        if line.is_empty() { continue; }
        let category = get_category(&line);
        if category != Category::Seeds { line = lines.next().expect("Error reading input").unwrap(); }

        while !line.is_empty() {
            match category {
                Category::Seeds => {
                    let start = line.find(':').unwrap_or(0);
                    let mut seed_line = &line[start+1..line.len()];
                    seed_line = seed_line.trim();
                    seed_line.split(' ').for_each(|i| seed_list_p1.push(i.to_string().parse::<u64>().expect("NaN")));

                    let mut i = 0;
                    while i < seed_list_p1.len() {
                        let num = seed_list_p1[i];
                        for j in num..num+seed_list_p1[i+1] {
                            seed_list_p2.push(j);
                        }
                        i += 2;
                    }
                },
                Category::SeedToSoil => { seed_to_soil.push(tokenize(&line)); },
                Category::SoilToFertilizer => { soil_to_fert.push(tokenize(&line)); },
                Category::FertilizerToWater => { fert_to_water.push(tokenize(&line)); },
                Category::WaterToLight => { water_to_light.push(tokenize(&line)); },
                Category::LightToTemp => { light_to_temp.push(tokenize(&line)); },
                Category::TempToHumdity => { temp_to_humd.push(tokenize(&line)); },
                Category::HumidityToLocation => { humd_to_location.push(tokenize(&line)); }
            }

            if lines.peek().is_some() { line = lines.next().expect("Error reading line2").unwrap(); }
            else { line = "".to_string(); }
        }
    }

    let mut min_loc = AtomicU64::new(u64::MAX);

    seed_list_p1.into_par_iter().for_each(|seed| {
        let soil = compute_offset(&seed_to_soil, seed);
        let fert = compute_offset(&soil_to_fert, soil);
        let water = compute_offset(&fert_to_water, fert);
        let light = compute_offset(&water_to_light, water);
        let temp = compute_offset(&light_to_temp, light);
        let hum = compute_offset(&temp_to_humd, temp);
        let loc = compute_offset(&humd_to_location, hum);
        min_loc.fetch_min(loc, std::sync::atomic::Ordering::Relaxed);
    });

    println!("lowest loc: {}", min_loc.get_mut());
    min_loc = AtomicU64::new(u64::MAX);

    seed_list_p2.into_par_iter().for_each(|seed|{
        let soil = compute_offset(&seed_to_soil, seed);
        let fert = compute_offset(&soil_to_fert, soil);
        let water = compute_offset(&fert_to_water, fert);
        let light = compute_offset(&water_to_light, water);
        let temp = compute_offset(&light_to_temp, light);
        let hum = compute_offset(&temp_to_humd, temp);
        let loc = compute_offset(&humd_to_location, hum);
        min_loc.fetch_min(loc, std::sync::atomic::Ordering::Relaxed);
    });

    println!("lowest loc: {}", min_loc.get_mut());
}

