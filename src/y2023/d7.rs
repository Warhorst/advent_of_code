use std::cmp::Ordering;
use std::collections::HashMap;

use HandType::*;

pub fn solve_7a(input: &str) -> usize {
    let mut hands = input.lines().map(Hand::from_line).collect::<Vec<_>>();

    hands.sort();

    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}

pub fn solve_7b(input: &str) -> usize {
    let mut hands = input.lines().map(Hand::from_line_with_joker).collect::<Vec<_>>();

    hands.sort();

    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    joker_active: bool,
}

impl Hand {
    fn from_line(line: &str) -> Self {
        let split = line.split(" ").collect::<Vec<_>>();
        let cards = split[0].chars().collect();
        let bid = split[1].parse::<usize>().unwrap();

        Hand {
            cards,
            bid,
            joker_active: false,
        }
    }

    fn from_line_with_joker(line: &str) -> Self {
        let mut hand = Hand::from_line(line);
        hand.joker_active = true;
        hand
    }

    fn get_hand_type(&self) -> HandType {
        if self.joker_active {
            self.get_hand_type_with_joker()
        } else {
            self.get_hand_type_without_joker()
        }
    }

    fn get_hand_type_with_joker(&self) -> HandType {
        let mut cards_amount = HashMap::new();

        for card in &self.cards {
            cards_amount.entry(*card).and_modify(|val| *val += 1).or_insert(1);
        }

        let (most_common_card, _) = cards_amount
            .iter()
            .map(|(k, v)| (*k, *v))
            .filter(|(k, _)| *k != 'J')
            .max_by(|(_, v0), (_, v1)| v0.cmp(v1))
            .unwrap_or(('J', 5));

        cards_amount.clear();

        for mut card in self.cards.clone() {
            if card == 'J' {
                card = most_common_card;
            }

            cards_amount.entry(card).and_modify(|val| *val += 1).or_insert(1);
        }

        if cards_amount.keys().count() == 1 {
            return Five;
        }

        if cards_amount.keys().count() == 2 && cards_amount.values().any(|n| *n == 4) {
            return Four;
        }

        if cards_amount.keys().count() == 2 && cards_amount.values().any(|n| *n == 3) {
            return FullHouse;
        }

        if cards_amount.keys().count() == 3 && cards_amount.values().filter(|n| **n == 2).count() == 2 {
            return TwoPair;
        }

        if cards_amount.keys().len() <= 3 && cards_amount.values().any(|n| *n == 3) {
            return Three;
        }

        if cards_amount.keys().count() <= 4 && cards_amount.values().filter(|n| **n == 2).count() == 1 {
            return OnePair;
        }

        High
    }

    fn get_hand_type_without_joker(&self) -> HandType {
        let mut cards_amount = HashMap::new();

        for card in &self.cards {
            cards_amount.entry(*card).and_modify(|val| *val += 1).or_insert(1);
        }

        if cards_amount.keys().count() == 1 {
            return Five;
        }

        if cards_amount.keys().count() == 2 && cards_amount.values().any(|n| *n == 4) {
            return Four;
        }

        if cards_amount.keys().count() == 2 && cards_amount.values().any(|n| *n == 3) {
            return FullHouse;
        }

        if cards_amount.keys().count() == 3 && cards_amount.values().filter(|n| **n == 2).count() == 2 {
            return TwoPair;
        }

        if cards_amount.keys().len() <= 3 && cards_amount.values().any(|n| *n == 3) {
            return Three;
        }

        if cards_amount.keys().count() <= 4 && cards_amount.values().filter(|n| **n == 2).count() == 1 {
            return OnePair;
        }

        High
    }

    fn get_card_values(&self) -> Vec<usize> {
        if self.joker_active {
            self.get_card_values_with_joker()
        } else {
            self.get_card_values_without_joker()
        }
    }

    fn get_card_values_with_joker(&self) -> Vec<usize> {
        self.cards
            .iter()
            .map(|card| match card {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => 0,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                c => panic!("something strange happened: {c}")
            })
            .collect()
    }

    fn get_card_values_without_joker(&self) -> Vec<usize> {
        self.cards
            .iter()
            .map(|card| match card {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                c => panic!("something strange happened: {c}")
            })
            .collect()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_type = self.get_hand_type();
        let other_type = other.get_hand_type();

        if self_type > other_type {
            return Some(Ordering::Greater);
        } else if self_type < other_type {
            return Some(Ordering::Less);
        }

        let self_values = self.get_card_values();
        let other_values = other.get_card_values();

        for (i, val) in self_values.into_iter().enumerate() {
            if val > other_values[i] {
                return Some(Ordering::Greater);
            } else if val < other_values[i] {
                return Some(Ordering::Less);
            }
        }

        Some(Ordering::Equal)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum HandType {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPair = 2,
    OnePair = 1,
    High = 0,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}