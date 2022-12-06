use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 6)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

fn count_duplicates(s: &str) -> usize {
    let mut dups = 0;
    for char in s.chars() {
        let matches = s.matches(char).collect::<Vec<&str>>();
        dups += matches.len() - 1;
    }
    dups
}

fn solve_part1(s: &str) -> usize {
    let mut last = String::from("");
    let mut pos = 0;
    for itr in s.chars().enumerate() {
        last.push(itr.1);
        if last.len() > 4 {
            last.remove(0);
        }
        if last.len() == 4 && count_duplicates(&last) == 0 {
            pos = itr.0;
            break;
        }
    }
    pos + 1
}

fn solve_part2(s: &str) -> usize {
    let mut last = String::from("");
    let mut pos = 0;
    for itr in s.chars().enumerate() {
        last.push(itr.1);
        if last.len() > 14 {
            last.remove(0);
        }
        if last.len() == 14 && count_duplicates(&last) == 0 {
            pos = itr.0;
            break;
        }
    }
    pos + 1
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
