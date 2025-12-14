use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    input
        .split(",")
        .flat_map(|part| {
            let split = part.split_once("-").unwrap();
            let range = (parse(split.0), parse(split.1));
            invalid_ids(range)
        })
        .sum()
}

fn invalid_ids(range: (usize, usize)) -> impl Iterator<Item = usize> {
    (range.0..=range.1).filter(|num| {
        let string = num.to_string();
        let split = string.split_at(string.len() / 2);
        split.0 == split.1
    })
}

pub fn solve_b(input: &str) -> usize {
    input
        .split(",")
        .flat_map(|part| {
            let split = part.split_once("-").unwrap();
            let range = (parse(split.0), parse(split.1));
            invalid_ids_b(range)
        })
        .sum()
}

fn invalid_ids_b(range: (usize, usize)) -> impl Iterator<Item = usize> {
    (range.0..=range.1).filter(|num| {
        let string = num.to_string();

        // At most the half of the string can be a repeating pattern
        for i in 1..=string.len() / 2 {
            if string.len() % i != 0 {
                // Can't be a repeating pattern if it doesn't fit the string
                continue;
            }

            let part = string.chars().take(i).collect::<String>();

            let new_string = (0..string.len() / i)
                .map(|_| part.as_str())
                .collect::<String>();

            if new_string == string {
                return true;
            }
        }

        false
    })
}
