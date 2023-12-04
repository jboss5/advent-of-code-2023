

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use regex::Regex;

struct Card {
    id: u32,
    num_wins: u32,
    total: u32
}

fn get_lines(filename: String) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("Error opening file");
    BufReader::new(file).lines()
}

fn main() {
    let lines = get_lines("input.txt".to_owned());
    let mut p1_sum: u32 = 0;
    let mut card_list: Vec<Card> = Vec::new();

    for l in lines {
        let mut line = l.unwrap().to_owned();

        // replace all multiple spaces with a single
        let reg = Regex::new(r"[ ]+").unwrap();
        line = reg.replace_all(&line.to_string()," ").to_string();
        
        // grab card id
        let mut card_split = line.split(':');
        let card: Vec<&str> = card_split.next().to_owned().expect("Mismatched input").trim().split(' ').collect();
        let id = card[1].trim().parse::<u32>().expect("ID not a number");

        // split winning cards vs hand
        let mut split = card_split.next().expect("Mismatched input").trim().split('|');
        let winners: HashSet<&str> = split.next().to_owned().expect("Mismatched winners").trim().split(' ').collect();

        // create card
        let mut c = Card { id, total: 0, num_wins: 0 };

        // find all matching winners for this card
        split.next().to_owned().expect("Mismatched numbers").trim().split(' ').for_each(|num| {
            if winners.contains(num) {
                c.total = if c.total == 0 { 1 } else { c.total*2 };
                c.num_wins += 1;
            }
        });
        
        println!("Card id [{}] winning total [{}] # of wins [{}]", c.id, c.total, c.num_wins);
        p1_sum += c.total;
        card_list.push(c);
    }

    // all originals are 1 each
    let mut scratchcards: Vec<u32> = vec![1; card_list.len()];

    // loop through each card
    for card in card_list {

        // loop through each card the # of times it was "copied"
        for _i in 0..scratchcards[(card.id-1) as usize] {

            // find all "copies" this card generates
            for j in card.id..(card.id + card.num_wins) {
                scratchcards[j as usize] += 1;
            }
        }
    }

    println!("\n\ntotal sum p1: {}", p1_sum);
    println!("total sum p2: {}", scratchcards.iter().sum::<u32>());
}
