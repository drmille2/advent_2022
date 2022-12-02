use clap::Parser;
use enum_primitive_derive::Primitive;
use num_traits::ToPrimitive;
use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 2)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}
fn main() {
    let cli_args = Cli::parse();

    let input = &fs::read_to_string(cli_args.input).unwrap();
    let rounds = input.split_terminator("\n");

    //
    // Part 1 solution
    //

    let mut sum = 0;
    for (idx, round) in rounds.enumerate() {
        println!("Scoring round {}", round);
        // parser for part 1
        // let throws = parse_round_as_throws(round);
        let throws = parse_round_as_result(round); // parser for part 2
        let score = score_round(throws[1], throws[0]);
        sum += score;
        println!("Round {} score: {}\n", idx, score);
    }
    println!("Final score: {}", sum);
}

#[derive(Eq, PartialEq, Primitive, Copy, Clone)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Error, Debug)]
enum Error {
    #[error("failed to parse throw, invalid input")]
    InvalidInput,
}

impl PartialOrd for Throw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Throw::Rock => match other {
                Throw::Rock => Some(Ordering::Equal),
                Throw::Paper => Some(Ordering::Less),
                Throw::Scissors => Some(Ordering::Greater),
            },
            Throw::Paper => match other {
                Throw::Rock => Some(Ordering::Greater),
                Throw::Paper => Some(Ordering::Equal),
                Throw::Scissors => Some(Ordering::Less),
            },
            Throw::Scissors => match other {
                Throw::Rock => Some(Ordering::Less),
                Throw::Paper => Some(Ordering::Greater),
                Throw::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

impl FromStr for Throw {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Throw::Rock),
            "B" | "Y" => Ok(Throw::Paper),
            "C" | "Z" => Ok(Throw::Scissors),
            _ => Err(Error::InvalidInput),
        }
    }
}

fn parse_round_as_throws(s: &str) -> Vec<Throw> {
    s.split_terminator(" ")
        .map(|t| Throw::from_str(t).unwrap())
        .collect()
}

fn parse_round_as_result(s: &str) -> Vec<Throw> {
    let entries: Vec<&str> = s.split_terminator(" ").collect();
    vec![
        Throw::from_str(entries[0]).unwrap(),
        get_desired_throw(Throw::from_str(entries[0]).unwrap(), entries[1]).unwrap(),
    ]
}

fn get_desired_throw(t: Throw, result: &str) -> Result<Throw, Error> {
    match result {
        "X" => match t {
            Throw::Rock => Ok(Throw::Scissors),
            Throw::Paper => Ok(Throw::Rock),
            Throw::Scissors => Ok(Throw::Paper),
        },
        "Y" => match t {
            Throw::Rock => Ok(Throw::Rock),
            Throw::Paper => Ok(Throw::Paper),
            Throw::Scissors => Ok(Throw::Scissors),
        },
        "Z" => match t {
            Throw::Rock => Ok(Throw::Paper),
            Throw::Paper => Ok(Throw::Scissors),
            Throw::Scissors => Ok(Throw::Rock),
        },
        _ => Err(Error::InvalidInput),
    }
}

fn score_round(you: Throw, opp: Throw) -> i32 {
    if you > opp {
        you.to_i32().unwrap() + 6
    } else if you < opp {
        you.to_i32().unwrap() + 0
    } else {
        you.to_i32().unwrap() + 3
    }
}

#[cfg(test)]
mod test {
    use crate::{score_round, Throw};

    #[test]
    fn test_score_round() {
        assert_eq!(score_round(Throw::Rock, Throw::Rock), 4);
        assert_eq!(score_round(Throw::Rock, Throw::Paper), 1);
        assert_eq!(score_round(Throw::Rock, Throw::Scissors), 7);
        assert_eq!(score_round(Throw::Paper, Throw::Rock), 8);
        assert_eq!(score_round(Throw::Paper, Throw::Paper), 5);
        assert_eq!(score_round(Throw::Paper, Throw::Scissors), 2);
        assert_eq!(score_round(Throw::Scissors, Throw::Rock), 3);
        assert_eq!(score_round(Throw::Scissors, Throw::Paper), 9);
        assert_eq!(score_round(Throw::Scissors, Throw::Scissors), 6);
    }
}
