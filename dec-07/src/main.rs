
pub mod hand;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use hand::Hand;

fn tokenize_cards(string: &str) -> Vec<u32> {
    let mut out: Vec<u32> = vec![];

    for ch in string.chars() {
        match ch {
            'A' => out.push(14),
            'K' => out.push(13),
            'Q' => out.push(12),
            'J' => out.push(11),
            'T' => out.push(10),
            _ => out.push(ch.to_string().parse::<u32>().expect("NaN"))
        }        
    }

    out
}

fn tokenize_cards_p2(string: &str) -> Vec<u32> {
    let mut out: Vec<u32> = vec![];

    for ch in string.chars() {
        match ch {
            'A' => out.push(14),
            'K' => out.push(13),
            'Q' => out.push(12),
            'J' => out.push(1),
            'T' => out.push(10),
            _ => out.push(ch.to_string().parse::<u32>().expect("NaN"))
        }        
    }

    out
}

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn main() {
    let lines = get_lines("input.txt".to_owned());
    let mut hands: Vec<Hand> = Vec::new();
    let mut hands_p2: Vec<Hand> = Vec::new();

    for l in lines {
        let line = l.unwrap();
        let delim = line.find(' ').expect("missing delim");
        let cards = tokenize_cards(&line[0..delim]);
        let cards_p2 = tokenize_cards_p2(&line[0..delim]);
        let bid = line[delim+1..line.len()].parse::<u32>().expect("NaN");
        hands.push(Hand::new(hand::compute_type(&cards), cards, bid));
        hands_p2.push(Hand::new(hand::compute_type(&cards_p2), cards_p2, bid));
    }

    hands.sort();
    hands_p2.sort();

    let mut sum: u32 = 0;
    for (i, h) in hands.iter().enumerate() {
        sum += h.bid * (i as u32 + 1);
    }

    println!("p1 sum: {}", sum);

    sum = 0;
    for (i, h) in hands_p2.iter().enumerate() {
        sum += h.bid * (i as u32 + 1);
    }

    println!("p2 sum: {}", sum);
}
