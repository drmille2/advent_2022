use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 3)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

fn to_ranges(s: &str) -> Vec<Vec<usize>> {
    s.split(',')
        .map(|x| x.split('-').map(|y| y.parse::<usize>().unwrap()).collect())
        .collect()
}

fn check_range_inclusion(r: &[usize], s: &[usize]) -> bool {
    (r[0] >= s[0] && r[1] <= s[1]) || (s[0] >= r[0] && s[1] <= r[1])
}

fn check_range_overlap(r: &[usize], s: &[usize]) -> bool {
    if r[0] <= s[0] {
        r[1] >= s[0]
    } else {
        s[1] >= r[0]
    }
}

fn solve_part1(s: &str) -> usize {
    s.split_terminator('\n')
        .map(to_ranges)
        .map(|x| {
            if check_range_inclusion(&x[0], &x[1]) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn solve_part2(s: &str) -> usize {
    s.split_terminator('\n')
        .map(to_ranges)
        .map(|x| {
            if check_range_overlap(&x[0], &x[1]) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

#[cfg(test)]
mod test {
    use crate::{check_range_inclusion, check_range_overlap, to_ranges};

    #[test]
    fn test_to_ranges() {
        assert_eq!(to_ranges("2-4,6-8"), vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(to_ranges("2-3,4-5"), vec![vec![2, 3], vec![4, 5]]);
        assert_eq!(to_ranges("2-8,3-7"), vec![vec![2, 8], vec![3, 7]]);
    }

    #[test]
    fn test_check_range_inclusion() {
        assert!(check_range_inclusion(&[2, 3], &[2, 4]));
        assert!(check_range_inclusion(&[6, 6], &[4, 6]));
        assert!(!check_range_inclusion(&[2, 4], &[6, 8]));
    }

    #[test]
    fn test_check_range_overlap() {
        assert!(check_range_overlap(&[2, 3], &[2, 4]));
        assert!(check_range_overlap(&[6, 6], &[4, 6]));
        assert!(!check_range_overlap(&[2, 4], &[6, 8]));
        assert!(check_range_overlap(&[2, 5], &[4, 6]));
        assert!(!check_range_overlap(&[7, 8], &[5, 6]));
    }
}
