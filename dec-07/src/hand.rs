
use core::fmt;
use std::collections::HashMap;

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum HandType {
    FivOk,
    FourOk,
    FullHouse,
    ThreeOk,
    TP,
    OP,
    HC
}

#[derive(Debug,Clone)]
pub struct Hand {
    pub(crate) hand_type: HandType,
    pub(crate) cards: Vec<u32>,
    pub(crate) bid: u32,
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn compute_type(cards: &[u32]) -> HandType {
    let mut map = HashMap::<u32, u32>::new();

    for c in cards {
        if map.contains_key(c) {
            let mut v = *map.get(c).expect("Not found");
            v += 1;
            map.insert(*c, v);
        } else {
            map.insert(*c, 1);
        }
    }

    let joker = 1_u32;
    match map.len() {
        1 => { HandType::FivOk },
        2 => { 
            let mut max = u32::MIN;
            for (_idx,val) in map.iter().enumerate() {
                max = max.max(*val.1);
            }

            let has_joker = map.contains_key(&joker);
            if has_joker {
                HandType::FivOk
            } else if max == 3 { 
                if map.contains_key(&joker) { HandType::FourOk }
                else { HandType::FullHouse }
            } 
            else { HandType::FourOk }
         },
        3 => { 
            let mut is_three = false;
            for (_idx, val) in map.iter().enumerate() {
                if *val.1 == 3 { is_three = true }
            }

            let mut joker_sum: u32 = 0;
            if map.contains_key(&joker) {
                joker_sum = *map.get(&joker).expect("NaN");
            }

            if is_three { 
                if joker_sum > 0 { HandType::FourOk }
                else { HandType::ThreeOk }
            } 
            else if joker_sum == 2 { HandType::FourOk }
            else if joker_sum > 0 { HandType::FullHouse }
            else { HandType::TP }
         },
        4 => { 
            if map.contains_key(&joker) { HandType::ThreeOk }
            else { HandType::OP }
        },
        _ => { 
            if map.contains_key(&joker) { HandType::OP }
            else { HandType::HC }
        }
    }
}

impl Hand {

    pub const fn new(hand_type: HandType, cards: Vec<u32>, bid: u32) -> Self {
        Hand { hand_type, cards, bid }
    }

    pub fn print(&self) {
        println!("cards: {:?}", self.cards);
    }
}