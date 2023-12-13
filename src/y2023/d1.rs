pub fn solve_1a(input: &str) -> usize {
    input
        .lines()
        .map(|line| line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<_>>()
        )
        .map(|num_chars| format!("{}{}", num_chars.first().unwrap(), num_chars.last().unwrap()))
        .map(|s| s.parse::<usize>().unwrap())
        .sum()
}

pub fn solve_1b(input: &str) -> usize {
    input
        .lines()
        .map(|line| get_line_numbers(line))
        .map(|numbers| format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap()).parse::<usize>().unwrap())
        .sum()
}

fn get_line_numbers(line: &str) -> Vec<u32> {
    let mut numbers = vec![];
    let mut current_str = String::new();

    for c in line.chars() {
        if c.is_numeric() {
            numbers.push(c.to_digit(10).unwrap())
        } else {
            current_str.push(c);
        }

        if current_str.contains("one") {
            numbers.push(1);
            current_str.clear();
            current_str.push(c);
        } else if current_str.contains("two") {
            numbers.push(2);
            current_str.clear();
            current_str.push(c);
        } else if current_str.contains("three") {
            numbers.push(3);
            current_str.clear();
            current_str.push(c);
        } else if current_str.contains("four") {
            numbers.push(4);
            current_str.clear();
            current_str.push(c);
        } else if current_str.contains("five") {
            numbers.push(5);
            current_str.clear();
            current_str.push(c);
        } else if current_str.contains("six") {
            numbers.push(6);
            current_str.clear();
            current_str.push(c);
        } else if current_str.contains("seven") {
            numbers.push(7);
            current_str.clear();
            current_str.push(c);
        } else if current_str.contains("eight") {
            numbers.push(8);
            current_str.clear();
            current_str.push(c);
        } else if current_str.contains("nine") {
            numbers.push(9);
            current_str.clear();
            current_str.push(c);
        }
    }

    numbers
}