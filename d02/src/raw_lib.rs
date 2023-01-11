//! raw_lib
//! main code house for first (raw) attempt, without examining other approaches

use std::fs;

///////////////////// Gen Funcs //////////////////////
pub fn get_input() -> String {
    let input = fs::read_to_string("input").expect("Error reading input file");
    input
}

///////////////////// Objects //////////////////////
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Throws {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Goals {
    Lose,
    Draw,
    Win,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Round {
    opponent: Throws,
    player: Throws,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StratRound {
    opponent: Throws,
    player: Goals,
}

///////////////////// Implems //////////////////////
impl Throws {
    fn throw_from_char(c: char) -> Throws {
        match c {
            'A' | 'X' => Throws::Rock,
            'B' | 'Y' => Throws::Paper,
            'C' | 'Z' => Throws::Scissors,
            _ => panic!("Invalid throw symbol"),
        }
    }

    fn goal_to_throw(&self, g: Goals) -> Throws {
        match (g, self) {
            (Goals::Lose, Throws::Rock) => Throws::Scissors,
            (Goals::Lose, Throws::Paper) => Throws::Rock,
            (Goals::Lose, Throws::Scissors) => Throws::Paper,
            (Goals::Draw, _) => self.clone(),
            (Goals::Win, Throws::Rock) => Throws::Paper,
            (Goals::Win, Throws::Paper) => Throws::Scissors,
            (Goals::Win, Throws::Scissors) => Throws::Rock,
        }
    }
}

impl Goals {
    fn goal_from_char(c: char) -> Goals {
        match c {
            'X' => Goals::Lose,
            'Y' => Goals::Draw,
            'Z' => Goals::Win,
            _ => panic!("Invalid throw symbol"),
        }
    }
}
/*
x: lose
y: draw
z: win
*/

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

impl StratRound {
    fn stratround_to_literal(&self) -> Round {
        let new_throw = goal_to_throw(self.player, self.opponent);

        Round::new(self.opponent, new_throw)
    }

    fn new(left: Throws, right: Goals) -> StratRound {
        StratRound {
            opponent: left,
            player: right,
        }
    }
}

///////////////////// Non-Impls //////////////////////
fn goal_to_throw(g: Goals, t: Throws) -> Throws {
    match (g, t) {
        (Goals::Lose, Throws::Rock) => Throws::Scissors,
        (Goals::Lose, Throws::Paper) => Throws::Rock,
        (Goals::Lose, Throws::Scissors) => Throws::Paper,
        (Goals::Draw, t) => t,
        (Goals::Win, Throws::Rock) => Throws::Paper,
        (Goals::Win, Throws::Paper) => Throws::Scissors,
        (Goals::Win, Throws::Scissors) => Throws::Rock,
    }
}

pub fn to_throw_pairs(input: &str) -> Vec<Round> {
    let mut throw_pairs = Vec::new();
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let left = Throws::throw_from_char(chars.next().unwrap());
        // let mid = chars.next().unwrap();
        chars.next();
        let right = Throws::throw_from_char(chars.next().unwrap());
        // println!("left: {:?}", left);
        // println!("mid: {:?}", mid);
        // println!("right: {:?}", right);
        throw_pairs.push(Round::new(left, right));
    });

    throw_pairs
}

pub fn to_gt_pair(input: &str) -> Vec<StratRound> {
    let mut throw_pairs = Vec::new();
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let left = Throws::throw_from_char(chars.next().unwrap());
        // let mid = chars.next().unwrap();
        chars.next();
        let right = Goals::goal_from_char(chars.next().unwrap());
        // println!("left: {:?}", left);
        // println!("mid: {:?}", mid);
        // println!("right: {:?}", right);
        throw_pairs.push(StratRound::new(left, right));
    });

    throw_pairs
}

pub fn strat_to_rounds(strat: &Vec<StratRound>) -> Vec<Round> {
    let mut rounds = Vec::new();
    strat.iter().for_each(|sr| {
        rounds.push(sr.stratround_to_literal());
    });

    rounds
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

//////////////////////////////////////////////////
///////////////////// Tests //////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_from_char() {
        assert_eq!(Throws::throw_from_char('A'), Throws::Rock);
        assert_eq!(Throws::throw_from_char('B'), Throws::Paper);
        assert_eq!(Throws::throw_from_char('C'), Throws::Scissors);
    }
    #[test]
    #[should_panic]
    fn test_from_char_panic() {
        Throws::throw_from_char('q');
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

    #[test]
    fn test_goal_to_throw() {
        assert_eq!(goal_to_throw(Goals::Win, Throws::Rock), Throws::Paper);
        assert_eq!(goal_to_throw(Goals::Win, Throws::Paper), Throws::Scissors);
        assert_eq!(goal_to_throw(Goals::Win, Throws::Scissors), Throws::Rock);

        assert_eq!(goal_to_throw(Goals::Lose, Throws::Rock), Throws::Scissors);
        assert_eq!(goal_to_throw(Goals::Lose, Throws::Paper), Throws::Rock);
        assert_eq!(goal_to_throw(Goals::Lose, Throws::Scissors), Throws::Paper);

        assert_eq!(goal_to_throw(Goals::Draw, Throws::Rock), Throws::Rock);
        assert_eq!(goal_to_throw(Goals::Draw, Throws::Paper), Throws::Paper);
        assert_eq!(
            goal_to_throw(Goals::Draw, Throws::Scissors),
            Throws::Scissors
        );
    }

    #[test]
    fn test_to_gt_pair() {
        let pairs = to_gt_pair(TEST_INPUT);
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0], StratRound::new(Throws::Rock, Goals::Draw));
        assert_eq!(pairs[1], StratRound::new(Throws::Paper, Goals::Lose));
        assert_eq!(pairs[2], StratRound::new(Throws::Scissors, Goals::Win));
    }

    #[test]
    fn test_strat_to_rounds() {
        let pairs = to_gt_pair(TEST_INPUT);
        let rounds = strat_to_rounds(&pairs);
        assert_eq!(rounds.len(), 3);
        assert_eq!(rounds[0], Round::new(Throws::Rock, Throws::Rock));
        assert_eq!(rounds[1], Round::new(Throws::Paper, Throws::Rock));
        assert_eq!(rounds[2], Round::new(Throws::Scissors, Throws::Rock));
    }

    #[test]
    fn test_strat_score_rounds() {
        let pairs = to_gt_pair(TEST_INPUT);
        let rounds = strat_to_rounds(&pairs);
        assert_eq!(score_rounds(rounds), 12);
    }
}
