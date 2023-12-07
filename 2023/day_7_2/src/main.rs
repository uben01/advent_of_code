use std::{error::Error, fs::File, io::{BufReader, BufRead}, collections::HashMap};
use std::cmp::Ordering;
use Ordering::{Greater, Less, Equal};

use crate::{Kind::{FiveOfAKind, FourOfAKind, FullHouse, ThreeOfAKind, TwoPair, OnePair, HighCard}};

#[derive(PartialEq, PartialOrd, Debug)]
enum Kind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug)]
struct Hand {
    kind: Kind,
    original: String
}

impl Hand {
    fn create(hand: String) -> Hand {
        let mut map: HashMap<char, usize> = HashMap::new();    
        for card in hand.chars() {
            map
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        } 

        let kind = Hand::kind(map);

        Hand { 
            kind,
            original: hand
        }
    }

    fn map_card_to_value(card: char) -> usize {
        return card.to_digit(10).unwrap_or_else(|| {
            if card == 'T' {
                return 10;
            } 
            if card == 'J' {
                return 1;
            } 
            if card == 'Q' {
                return 12;
            } 
            if card == 'K' {
                return 13;
            } 
            
            return 14;
        }) as usize;
    }

    fn kind(map: HashMap<char, usize>) -> Kind {
        let mut vector: Vec<(char, usize)> = map.iter().map(|a| (*a.0, *a.1)).collect();
        vector.sort_by(|a, b| b.1.cmp(&a.1));
        
        let number_of_jokers =  *map.get(&'J').unwrap_or_else(|| &0);

        if vector[0].0 == 'J' {
            if number_of_jokers == 5 {
                return FiveOfAKind;
            }

            vector[1].1 += number_of_jokers;
            vector.remove(0);
        } else {
            vector[0].1 += number_of_jokers;
            vector = vector.iter().filter(|e| e.0 != 'J').map(|a| (a.0, a.1)).collect();
        }

        if vector[0].1 == 5 {
            return FiveOfAKind;
        }
        if vector[0].1 == 4 {
            return FourOfAKind;
        }
        if vector[0].1 == 3 && vector[1].1 == 2 {
            return FullHouse;
        }
        if vector[0].1 == 3 {
            return ThreeOfAKind;
        }
        if vector[0].1 == 2 && vector[1].1 == 2 {
            return TwoPair;
        }
        if vector[0].1 == 2 {
            return OnePair;
        }
        return HighCard;
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.original == other.original; 
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left_kind = &self.kind;
        let right_kind = &other.kind;

        if left_kind < right_kind {
            Some(Less)
        } else if left_kind > right_kind {
            Some(Greater)
        } else {
            for i in 0..self.original.len() {
                let left = Hand::map_card_to_value(self.original.chars().nth(i).unwrap());
                let right = Hand::map_card_to_value(other.original.chars().nth(i).unwrap());

                let ordering = right.cmp(&left);
                if ordering != Equal {
                    return Some(ordering);
                }
            }
            Some(Equal)
        }
    }
}

struct HandBid {
    hand: Hand,
    bid: u64,
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut hand_bids: Vec<HandBid> = vec![];
    for line in buff.lines() {
        let line = &line.unwrap();
        let split: Vec<&str> = line.split(' ').collect();
        let hand = split.get(0).unwrap().to_string();
        let bid: u64 = split.get(1).unwrap().to_string().parse().unwrap();

        let hand = Hand::create(hand);
        hand_bids.push(HandBid { hand, bid });
    }

    hand_bids.sort_by(|a, b| {
            *(&b.hand.partial_cmp(&a.hand).unwrap())
        }
    );

    let mut sum: u64 = 0;
    for (idx, hand_bid) in hand_bids.iter().enumerate() {
        sum += (idx as u64 + 1) * hand_bid.bid;
    }
    println!("The sum is: {sum}");
    return Ok(());
}