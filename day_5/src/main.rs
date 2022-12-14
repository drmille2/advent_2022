use clap::Parser;
use std::{collections::VecDeque, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 5)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

fn split_input(s: &str) -> Vec<&str> {
    s.split("\n\n").collect()
}

fn parse_stacks(s: &str) -> Vec<VecDeque<char>> {
    let mut out: Vec<VecDeque<char>> = Vec::new();

    // pre-allocate our stacks
    let num_stacks = 9;
    for _ in 0..num_stacks {
        out.push(VecDeque::new());
    }

    // parse each row and send elements to their
    // corresponding stack
    for row in s.split('\n') {
        let mut row = String::from(row);
        row.push(' '); // our rows are 1 short for group_items to work, pad it out
        let columns = group_items(row.chars().collect::<Vec<char>>(), 4);
        for c in columns.into_iter().enumerate() {
            if let Some(elfbox) = to_elfbox(&c.1) {
                let stack_num = c.0 % num_stacks;
                out[stack_num].push_back(elfbox);
            }
        }
    }
    out
}

fn to_elfbox(s: &str) -> Option<char> {
    if s.trim_start().is_empty() {
        return None;
    }
    let res = s.trim_start_matches('[').chars().next().unwrap();
    if res.is_alphabetic() {
        Some(res)
    } else {
        None
    }
}

/// panics if items is not evenly divisble by group_len
fn group_items(items: Vec<char>, group_len: usize) -> Vec<String> {
    let mut inter = Vec::new();

    let mut g = 0;
    let groups = items.len() / group_len;
    while g < groups {
        inter.push(vec![]);
        g += 1;
    }

    for item in items.into_iter().enumerate() {
        let group_num = item.0 / group_len;
        inter[group_num].push(item.1);
    }

    let mut out = Vec::new();
    for i in inter {
        out.push(i.into_iter().collect::<String>());
    }
    out
}

fn parse_operations(s: &str) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for op_string in s.split('\n') {
        let mut op = Vec::new();
        for o in op_string
            .replace(|x: char| x.is_alphabetic(), "")
            .split(' ')
        {
            if !o.is_empty() {
                op.push(o.parse::<usize>().unwrap());
            }
        }
        out.push(op);
    }
    out
}

fn do_operation(stacks: &mut [VecDeque<char>], op: &[usize]) {
    for _ in 0..op[0] {
        let elfbox = stacks[op[1] - 1].pop_front().unwrap();
        stacks[op[2] - 1].push_front(elfbox);
    }
}

fn do_operation_p2(stacks: &mut [VecDeque<char>], op: &[usize]) {
    let mut elfboxes = VecDeque::new();
    for _ in 0..op[0] {
        elfboxes.push_back(stacks[op[1] - 1].pop_front().unwrap());
    }
    for _ in 0..op[0] {
        stacks[op[2] - 1].push_front(elfboxes.pop_back().unwrap());
    }
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(s: &str) -> String {
    let inputs = split_input(s);
    let mut output = String::new();
    let mut stacks = parse_stacks(inputs[0]);
    let operations = parse_operations(inputs[1]);
    for op in operations {
        do_operation(&mut stacks, &op);
    }
    for stack in stacks {
        output.push(stack[0]);
    }
    output
}

fn solve_part2(s: &str) -> String {
    let inputs = split_input(s);
    let mut output = String::new();
    let mut stacks = parse_stacks(inputs[0]);
    let operations = parse_operations(inputs[1]);
    for op in operations {
        do_operation_p2(&mut stacks, &op);
    }
    for stack in stacks {
        output.push(stack[0]);
    }
    output
}
