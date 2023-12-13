use std::collections::HashMap;

pub fn solve_4a(input: &str) -> usize {
    input
        .lines()
        .map(Card::from_line)
        .map(|card| card.get_value())
        .sum()
}

pub fn solve_4b(input: &str) -> usize {
    let cards = input
        .lines()
        .map(Card::from_line)
        .collect::<Vec<_>>();

    let mut num_copy_count_map = HashMap::new();

    for card in &cards {
        num_copy_count_map.entry(card.number).and_modify(|val| *val += 1).or_insert(1);

        for copy in card.get_won_copies() { // 2,3,4,5
            for _ in 0..*num_copy_count_map.get(&card.number).unwrap() {
                num_copy_count_map.entry(copy).and_modify(|val| *val += 1).or_insert(1);
            }
        }
    }

    num_copy_count_map
        .values()
        .sum()
}

#[derive(Debug)]
struct Card {
    number: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

impl Card {
    fn from_line(line: &str) -> Self {
        let number = line.split(":").next().unwrap().replace("Card", "").trim().parse::<usize>().unwrap();
        let right_side = line.split(":").skip(1).next().unwrap();
        let numbers = right_side.split("|").collect::<Vec<_>>();
        let winning_numbers = numbers[0]
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let my_numbers = numbers[1]
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Card {
            number,
            winning_numbers,
            my_numbers,
        }
    }

    fn get_value(&self) -> usize {
        let amount_winning_numbers = self.my_numbers
            .iter()
            .filter(|my_num| self.winning_numbers.contains(my_num))
            .count();

        if amount_winning_numbers == 0 {
            0
        } else {
            let mut value = 1;

            for _ in 1..amount_winning_numbers {
                value *= 2;
            }
            value
        }
    }

    fn get_won_copies(&self) -> Vec<usize> {
        self.my_numbers
            .iter()
            .filter(|my_num| self.winning_numbers.contains(my_num))
            .enumerate()
            .map(|(i, _)| self.number + i + 1)
            .collect()
    }
}