use self::Shape::*;
use self::Strat::*;

pub fn solve_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| *c != ' ');
            let s1 = Shape::from(chars.next().unwrap());
            let s2 = Shape::from(chars.next().unwrap());
            (s1, s2)
        })
        .map(|(opponent, you)| you.score() + you.score_against(opponent))
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| *c != ' ');
            let opp = Shape::from(chars.next().unwrap());
            let strat = Strat::from(chars.next().unwrap());
            (opp, strat)
        })
        .map(|(opp, strat)| (opp, strat.answer(opp)))
        .map(|(opponent, you)| you.score() + you.score_against(opponent))
        .sum()
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }

    fn score_against(&self, other: Shape) -> usize {
        match (*self, other) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Paper) => 3,
            (Paper, Rock) => 6,
            (Paper, Scissors) => 0,
            (Scissors, Scissors) => 3,
            (Scissors, Rock) => 0,
            (Scissors, Paper) => 6
        }
    }
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!()
        }
    }
}

#[derive(Clone, Copy)]
enum Strat {
    Win,
    Draw,
    Lose
}

impl Strat {
    fn answer(&self, opponent: Shape) -> Shape {
        match (opponent, *self) {
            (Rock, Lose) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
            (Paper, Lose) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Lose) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock
        }
    }
}

impl From<char> for Strat {
    fn from(value: char) -> Self {
        match value {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!()
        }
    }
}