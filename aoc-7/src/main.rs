use std::env;
use std::fs;
use regex::Regex;
use std::fmt::Display;
use std::cmp;
use std::ops::Index;

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug, Copy, Clone)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl Card {
    fn get(c: char) -> Card {
        match c {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::CT,
            'J' => Card::CJ,
            'Q' => Card::CQ,
            'K' => Card::CK,
            'A' => Card::CA,
            _ => panic!("Invalid card"),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Card::C2 => "2",
                Card::C3 => "3",
                Card::C4 => "4",
                Card::C5 => "5",
                Card::C6 => "6",
                Card::C7 => "7",
                Card::C8 => "8",
                Card::C9 => "9",
                Card::CT => "T",
                Card::CJ => "J",
                Card::CQ => "Q",
                Card::CK => "K",
                Card::CA => "A",
            }
        ).expect("Failed to write card");
        Ok(())
    }
}

#[derive(Eq)]
struct CardCollection {vector: Vec<CardEntry> }

impl CardCollection {
    fn len(&self) -> usize {
        self.vector.len()
    }
}

impl PartialEq for CardCollection {
    fn eq(&self, other: &CardCollection) -> bool {
        self.vector == other.vector
    }
}

impl Index<usize> for CardCollection {
    type Output = CardEntry;
    fn index(&self, index: usize) -> &CardEntry {
        &self.vector[index]
    }
}

impl CardCollection {
    fn from(cards: &Vec<Card>) -> CardCollection {
        CardCollection { vector: CardEntry::from(cards) }
    }
}

#[derive(Eq)]
struct CardEntry {
    card: Card,
    count: usize,
}

impl PartialOrd for CardEntry {
    fn partial_cmp(&self, other: &CardEntry) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for CardEntry {
    fn cmp(&self, other: &CardEntry) -> cmp::Ordering {
        if self.count > other.count {
            cmp::Ordering::Greater
        } else if self.count < other.count {
            cmp::Ordering::Less
        } else {
            self.card.cmp(&other.card)
        }
    }
}

impl PartialEq for CardEntry {
    fn eq(&self, other: &CardEntry) -> bool {
        self.card == other.card && self.count == other.count
    }
}


impl CardEntry {
    fn new(card: Card, count: usize) -> CardEntry {
        CardEntry { card: card, count: count }
    }

    fn from(cards: &Vec<Card>) -> Vec<CardEntry> {
        let mut result: Vec<CardEntry> = Vec::new();
        for card in cards {
            if let Some(entry) = result.iter_mut().find(|x| x.card == *card) {
                entry.count += 1;
            } else {
                result.push(CardEntry::new(*card, 1));
            }
        }
        result.sort();
        result
    }
}

#[derive(Eq)]
struct Hand {
    cards: CardCollection,
    card_vector: Vec<Card>,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> cmp::Ordering {
        if self.cards[0].count > other.cards[0].count {
            return cmp::Ordering::Greater;
        } else if self.cards[0].count < other.cards[0].count {
            return cmp::Ordering::Less;
        } else {
            if self.cards.len() == 1 {
                return self.card_vector[0].cmp(&other.card_vector[0]);
            } else if self.cards[1].count > other.cards[1].count {
                return cmp::Ordering::Greater;
            } else if self.cards[1].count < other.cards[1].count {
                return cmp::Ordering::Less;
            } else {
                for i in 0..5 {
                    if self.card_vector[i].cmp(&other.card_vector[i]) != cmp::Ordering::Equal {
                        return self.card_vector[i].cmp(&other.card_vector[i]);
                    }
                }
                return cmp::Ordering::Equal;
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cards == other.cards
    }
}

impl Hand {
    fn new(line: &str) -> Hand {
        let cards = Regex::new(r"(?<cards>[2|3|4|5|6|7|8|9|T|J|Q|K|A]+)\s+(?<bid>\d+)").unwrap();
        let result = cards.captures(line).unwrap();
        let cards: Vec<Card> = result["cards"].chars().map(Card::get).collect();
        if cards.len() != 5 {
            panic!("Invalid number of cards");
        }
        let bid = result["bid"].parse::<usize>().unwrap();
        let card_collection = CardCollection::from(&cards);
        return Hand { cards: card_collection, card_vector: cards, bid: bid };
    }
}

impl Display for CardCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vector.iter().for_each(|entry| {
            let card = &entry.card;
            let count = &entry.count;
            for _ in 0..*count {
                write!(f, "{},", *card).expect("Failed to write CardCollection");
            }
        });
        Ok(())
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cards: {:?} Bid: {:0>3}", self.card_vector, self.bid).expect("Failed to write Hand");
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();

    let mut hands: Vec<Hand> = lines.iter().filter(|line| !line.is_empty()).map(|line| Hand::new(line)).collect();
    hands.sort();
    hands.reverse();
    let mut total_score = 0;
    hands.iter().enumerate().for_each(|(i, hand)| {
        let score = hand.bid * (i+1);
        println!("{} -> {}", hand, score);
        total_score += score;
    });
    println!("Total score: {}", total_score);
}