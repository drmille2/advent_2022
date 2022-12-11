use clap::Parser;
use std::{collections::VecDeque, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 11)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test_div: usize,
    if_true: usize,
    if_false: usize,
}

impl Monkey {}

#[derive(Debug)]
enum Op {
    Add(usize),
    Mult(usize),
    Square,
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

fn parse_starting_items(s: &str) -> VecDeque<usize> {
    let mut out = VecDeque::new();
    let items_str = s.split_once(":").unwrap().1;
    for item in items_str.split(",") {
        out.push_back(item.trim().parse().unwrap());
    }
    out
}

fn parse_op(s: &str) -> Op {
    let trimmed: Vec<&str> = s
        .split_once("=")
        .unwrap()
        .1
        .trim_start()
        .split(" ")
        .collect();
    match trimmed[1] {
        "+" => return Op::Add(trimmed[2].parse().unwrap()),
        "*" => {
            if let Ok(num) = trimmed[2].parse() {
                return Op::Mult(num);
            } else {
                return Op::Square;
            }
        }
        _ => panic!("invalid operation encountered"),
    }
}

fn parse_last_num(s: &str) -> usize {
    s.split(" ").reduce(|a, i| i).unwrap().parse().unwrap()
}

fn parse_test_div(s: &str) -> usize {
    parse_last_num(s)
}

fn parse_throw(s: (&str, &str)) -> (usize, usize) {
    (parse_last_num(s.0), parse_last_num(s.1))
}

fn parse_input(s: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    // let rows = s.split("\n").collect();
    let rows = s.split_terminator("\n").collect();
    for monkey_str in group_items(rows, 7) {
        let items = parse_starting_items(monkey_str[1]);
        let op = parse_op(monkey_str[2]);
        let test_div = parse_test_div(monkey_str[3]);
        let (if_true, if_false) = parse_throw((&monkey_str[4], &monkey_str[5]));
        monkeys.push(Monkey {
            items,
            op,
            test_div,
            if_true,
            if_false,
        })
    }
    monkeys
}
fn solve_part1(s: &str) {
    let monkeys = parse_input(s);
    dbg!(monkeys);
}

fn solve_part2(s: &str) {}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: \n");
    solve_part1(input);
    // println!("Part 2: {}", solve_part2(input));
}
