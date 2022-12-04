use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

fn main() {
    let cli_args = Cli::parse();

    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn to_priority(item: &char) -> Option<i32> {
    let offset: i32 = if item.is_lowercase() { -9 } else { 17 };
    item.to_digit(36).and_then(|x| Some(x as i32 + offset))
}

fn sep_compartments<'a>(sack: &'a str) -> (&'a str, &'a str) {
    sack.split_at(sack.len() / 2)
}

fn return_duplicates(compartments: (&str, &str)) -> Vec<char> {
    let mut out = Vec::new();
    let mut inclusion_map: HashMap<char, bool> = HashMap::new();
    for item in compartments.0.chars() {
        inclusion_map.insert(item, true);
    }
    for item in compartments.1.chars() {
        if *inclusion_map.get(&item).unwrap_or(&false) {
            out.push(item)
        }
    }
    out.sort();
    out.dedup();
    out
}

/// panics if items is not evenly divisble by group_len
fn group_items(items: Vec<&str>, group_len: usize) -> Vec<Vec<&str>> {
    let mut out = Vec::new();

    let mut g = 0;
    let groups = items.len() / group_len;
    while g < groups {
        out.push(vec![]);
        g += 1;
    }

    for item in items.into_iter().enumerate() {
        let group_num = item.0 / group_len;
        out[group_num].push(item.1);
    }

    out
}

fn find_common_elements(group: Vec<&str>) -> Vec<char> {
    let mut sets: Vec<HashSet<char>> = Vec::new();
    for g in group {
        let mut set = HashSet::new();
        g.chars().map(|x| set.insert(x)).for_each(drop);
        sets.push(set);
    }
    sets.into_iter()
        .reduce(|a, i| i.intersection(&a).map(|x| *x).collect::<HashSet<char>>())
        .unwrap()
        .into_iter()
        .collect::<Vec<char>>()
}

fn solve_part1(input: &str) -> i32 {
    input
        .split_terminator("\n")
        .map(sep_compartments)
        .map(return_duplicates)
        .map(|x| x.into_iter().map(|y| to_priority(&y).unwrap()).sum::<i32>())
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    group_items(input.split_terminator("\n").collect(), 3)
        .into_iter()
        .map(find_common_elements)
        .map(|x| x.into_iter().map(|y| to_priority(&y).unwrap()).sum::<i32>())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{
        find_common_elements, group_items, return_duplicates, sep_compartments, to_priority,
    };

    #[test]
    fn test_to_priority() {
        assert_eq!(to_priority(&'a').unwrap(), 1);
        assert_eq!(to_priority(&'z').unwrap(), 26);
        assert_eq!(to_priority(&'A').unwrap(), 27);
        assert_eq!(to_priority(&'Z').unwrap(), 52);
    }

    #[test]
    fn test_sep_compartments() {
        assert_eq!(
            sep_compartments("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        );
        assert_eq!(
            sep_compartments("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
        )
    }

    #[test]
    fn test_return_duplicates() {
        assert_eq!(
            return_duplicates(("vJrwpWtwJgWr", "hcsFMMfFFhFp")),
            vec!['p']
        );

        assert_eq!(
            return_duplicates(("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")),
            vec!['L']
        );
    }

    #[test]
    fn test_group_items() {
        assert_eq!(
            group_items(vec!["a", "b", "c", "d", "e", "f"], 3),
            vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]
        );
    }

    #[test]
    fn test_find_common_elements() {
        assert_eq!(
            find_common_elements(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ]),
            vec!['r']
        );
        assert_eq!(
            find_common_elements(vec![
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]),
            vec!['Z']
        );
    }
}
