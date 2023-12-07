use std::{error::Error, fs::File, io::{BufReader, BufRead}, collections::HashMap};
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use crate::Kind::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

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
        let mut vector: Vec<usize> = map.iter().map(|a| *a.1).collect();
        vector.sort_by(|a, b| b.cmp(&a));

        if vector[0] == 5 {
            return FiveOfAKind;
        }
        if vector[0] == 4 {
            return FourOfAKind;
        }
        if vector[0] == 3 && vector[1] == 2 {
            return FullHouse;
        }
        if vector[0] == 3 {
            return ThreeOfAKind;
        }
        if vector[0] == 2 && vector[1] == 2 {
            return TwoPair;
        }
        if vector[0] == 2 {
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
            Some(Ordering::Less)
        } else if left_kind > right_kind {
            Some(Ordering::Greater)
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
        println!("{:?}", hand_bid.hand);
        println!("{:?}", hand_bid.bid);
        sum += (idx as u64 + 1) * hand_bid.bid;
    }
    println!("The sum is: {sum}");
    return Ok(());
}
