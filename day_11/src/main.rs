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

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u128>,
    op: Op,
    test_div: u128,
    if_true: usize,
    if_false: usize,
    ins_count: usize,
}

impl Monkey {
    fn inspect(&mut self) -> Option<u128> {
        if let Some(item) = self.items.pop_front() {
            let item = match self.op {
                Op::Add(val) => item + val,
                Op::Mult(val) => dbg!(item * val),
                Op::Square => item * item,
            };
            self.ins_count += 1;
            Some(item / 3)
        } else {
            None
        }
    }
    fn throw(&mut self) -> Option<(usize, u128)> {
        if let Some(item) = self.inspect() {
            if item % self.test_div == 0 {
                Some((self.if_true, item))
            } else {
                Some((self.if_false, item))
            }
        } else {
            None
        }
    }

    fn catch(&mut self, item: u128) {
        self.items.push_back(item);
    }
}

#[derive(Debug)]
struct Barrel {
    monkeys: Vec<Monkey>,
}

impl Barrel {
    fn from_str(s: &str) -> Self {
        let mut monkeys = Vec::new();
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
                ins_count: 0,
            })
        }
        Barrel { monkeys }
    }
    fn throw_and_catch(&mut self, pos: usize) -> Option<bool> {
        if let Some(throw) = self.monkeys[pos].throw() {
            self.monkeys[throw.0].catch(throw.1);
            Some(true)
        } else {
            None
        }
    }
    fn do_round(&mut self) {
        for m in 0..self.monkeys.len() {
            loop {
                if let Some(_) = self.throw_and_catch(m) {
                    continue;
                } else {
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Add(u128),
    Mult(u128),
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

fn parse_starting_items(s: &str) -> VecDeque<u128> {
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

fn parse_last_num(s: &str) -> u128 {
    s.split(" ").reduce(|_, i| i).unwrap().parse().unwrap()
}

fn parse_test_div(s: &str) -> u128 {
    parse_last_num(s)
}

fn parse_throw(s: (&str, &str)) -> (usize, usize) {
    (parse_last_num(s.0) as usize, parse_last_num(s.1) as usize)
}

fn solve_part1(s: &str) -> usize {
    let mut barrel = Barrel::from_str(s);
    for _ in 0..20 {
        barrel.do_round();
    }
    let mut sorted_monkeys = barrel.monkeys.clone();
    sorted_monkeys.sort_by(|a, b| b.ins_count.cmp(&a.ins_count));
    sorted_monkeys[0].ins_count * sorted_monkeys[1].ins_count
}

fn solve_part2(s: &str) -> usize {
    let mut barrel = Barrel::from_str(s);
    for r in 0..7000 {
        dbg!(r);
        barrel.do_round();
    }
    let mut sorted_monkeys = barrel.monkeys.clone();
    sorted_monkeys.sort_by(|a, b| b.ins_count.cmp(&a.ins_count));
    sorted_monkeys[0].ins_count * sorted_monkeys[1].ins_count
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
