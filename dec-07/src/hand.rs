
use core::fmt;
use std::collections::HashMap;
use std::cmp::Ordering;

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

impl Hand {
    pub const fn new(hand_type: HandType, cards: Vec<u32>, bid: u32) -> Self {
        Hand { hand_type, cards, bid }
    }
}

impl Eq for Hand { }

impl PartialEq for Hand {
    fn eq(&self, h2: &Self) -> bool {
        self.bid == h2.bid && self.cards.eq(&h2.cards) && self.hand_type == h2.hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, h2: &Self) -> Option<Ordering> {
        Some(self.cmp(h2))
    }
}

impl Ord for Hand {
    fn cmp(&self, h2: &Self) -> Ordering {
        if self.hand_type != h2.hand_type {
            match self.hand_type {
                HandType::FivOk => Ordering::Greater,
                HandType::FourOk => if h2.hand_type != HandType::FivOk { Ordering::Greater } else { Ordering::Less },
                HandType::FullHouse => if h2.hand_type != HandType::FivOk && h2.hand_type != HandType::FourOk { Ordering::Greater } else { Ordering::Less },
                HandType::ThreeOk => if h2.hand_type != HandType::FivOk && h2.hand_type != HandType::FourOk && h2.hand_type != HandType::FullHouse { Ordering::Greater } else { Ordering::Less },
                HandType::TP => if h2.hand_type != HandType::OP && h2.hand_type != HandType::HC { Ordering::Less } else { Ordering::Greater},
                HandType::OP => if h2.hand_type != HandType::HC { Ordering::Less } else { Ordering::Greater },
                HandType::HC => std::cmp::Ordering::Less
            }
        } else {
            for (i, val) in self.cards.iter().enumerate() {
                if *val == h2.cards[i] { continue; }
                if *val > h2.cards[i] { return Ordering::Greater; }
                return Ordering::Less;
            }
    
            panic!("totally equal, can this happen?");
        }
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