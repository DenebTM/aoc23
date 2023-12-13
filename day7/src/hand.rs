use std::{collections::HashMap, fmt::Display, iter::zip};

use crate::{card::Card, hand_type::HandType};

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug)]
pub struct Hand(pub Vec<Card>);
impl Hand {
    pub fn get_type(&self) -> HandType {
        let mut card_counts: HashMap<Card, i32> = HashMap::new();
        for card in &self.0 {
            let count = if card_counts.contains_key(&card) {
                *card_counts.get(&card).unwrap() + 1
            } else {
                1
            };
            card_counts.insert(card.clone(), count);
        }

        match card_counts.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => match card_counts.values().filter(|count| **count == 3).count() {
                1 => HandType::ThreeOfKind,
                0 => HandType::TwoPairs,
                _ => HandType::Unknown,
            },
            2 => match card_counts.values().filter(|count| **count == 3).count() {
                1 => HandType::FullHouse,
                0 => HandType::FourOfKind,
                _ => HandType::Unknown,
            },
            1 => HandType::FiveOfKind,

            _ => HandType::Unknown,
        }
    }
}

impl<'a> TryFrom<&'a str> for Hand {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let cards: Result<Vec<Card>, String> = value.chars().map(|c| Card::try_from(c)).collect();
        Ok(Hand(cards?))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        let mut self_sorted = self.0.clone();
        self_sorted.sort();
        let mut other_sorted = other.0.clone();
        other_sorted.sort();
        if self_sorted == other_sorted {
            return Equal;
        }

        match self.get_type().cmp(&other.get_type()) {
            Less => Less,
            Greater => Greater,

            Equal => {
                for (self_card, other_card) in zip(&self.0, &other.0) {
                    let comp = self_card.cmp(other_card);
                    if comp != Equal {
                        return comp;
                    }
                }

                Equal
            }
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            write!(f, "{}", c).unwrap();
        }
        write!(f, "")
    }
}
