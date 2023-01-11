//! raw_lib
//! main code house for first (raw) attempt, without examining other approaches

use std::fs;

pub fn get_input() -> String {
    let input = fs::read_to_string("input").expect("Error reading input file");
    input
}

#[derive(Debug, PartialEq, Eq)]
pub enum Throws {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Round {
    opponent: Throws,
    player: Throws,
}

impl Throws {
    fn from_char(c: char) -> Throws {
        match c {
            'A' | 'X' => Throws::Rock,
            'B' | 'Y' => Throws::Paper,
            'C' | 'Z' => Throws::Scissors,
            _ => panic!("Invalid throw symbol"),
        }
    }
}

impl Round {
    fn new(left: Throws, right: Throws) -> Round {
        Round {
            opponent: left,
            player: right,
        }
    }

    fn measure_round(&self) -> u32 {
        let shape_value: u32 = match &self.player {
            Throws::Rock => 1,
            Throws::Paper => 2,
            Throws::Scissors => 3,
        };

        let outcome_value: u32 = match (&self.opponent, &self.player) {
            (Throws::Rock, Throws::Rock) => 3,
            (Throws::Rock, Throws::Paper) => 6,
            (Throws::Rock, Throws::Scissors) => 0,

            (Throws::Paper, Throws::Rock) => 0,
            (Throws::Paper, Throws::Paper) => 3,
            (Throws::Paper, Throws::Scissors) => 6,

            (Throws::Scissors, Throws::Rock) => 6,
            (Throws::Scissors, Throws::Paper) => 0,
            (Throws::Scissors, Throws::Scissors) => 3,
        };

        shape_value + outcome_value
    }
}

pub fn to_throw_pairs(input: &str) -> Vec<Round> {
    let mut throw_pairs = Vec::new();
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let left = Throws::from_char(chars.next().unwrap());
        // let mid = chars.next().unwrap();
        chars.next();
        let right = Throws::from_char(chars.next().unwrap());
        // println!("left: {:?}", left);
        // println!("mid: {:?}", mid);
        // println!("right: {:?}", right);
        throw_pairs.push(Round::new(left, right));
    });

    throw_pairs
}

pub fn score_rounds(vec: Vec<Round>) -> u32 {
    vec.iter().map(|round| round.measure_round()).sum()
}

/*
x: +1
y: +2
z: +3

l: +0
d: +3
w: +6

a > z
b > x
c > y
*/

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_from_char() {
        assert_eq!(Throws::from_char('A'), Throws::Rock);
        assert_eq!(Throws::from_char('B'), Throws::Paper);
        assert_eq!(Throws::from_char('C'), Throws::Scissors);
    }
    #[test]
    #[should_panic]
    fn test_from_char_panic() {
        Throws::from_char('q');
    }

    #[test]
    fn test_new() {
        let round = Round::new(Throws::Rock, Throws::Paper);
        assert_eq!(round.opponent, Throws::Rock);
        assert_eq!(round.player, Throws::Paper);
    }

    #[test]
    fn test_measure_round() {
        let round = Round::new(Throws::Rock, Throws::Paper);
        assert_eq!(round.measure_round(), 8);
    }

    #[test]
    fn test_to_throw_pairs() {
        let pairs = to_throw_pairs(TEST_INPUT);
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0], Round::new(Throws::Rock, Throws::Paper));
        assert_eq!(pairs[1], Round::new(Throws::Paper, Throws::Rock));
        assert_eq!(pairs[2], Round::new(Throws::Scissors, Throws::Scissors));
    }

    #[test]
    fn test_score_rounds() {
        let rounds = to_throw_pairs(TEST_INPUT);
        assert_eq!(score_rounds(rounds), 15);
    }
}
