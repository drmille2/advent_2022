use clap::Parser;
use std::{collections::VecDeque, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 10)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
struct Cpu {
    r1: i64,
    cur_op: Op,
    ops: VecDeque<Op>,
    cyc: usize,
    breaks: Vec<usize>,
}

impl Cpu {
    fn new(mut ops: VecDeque<Op>) -> Self {
        Cpu {
            r1: 1,
            cyc: 1,
            cur_op: ops.pop_front().unwrap(),
            ops: ops.clone(),
            breaks: Vec::new(),
        }
    }

    fn do_op(&mut self) -> Option<bool> {
        match self.cur_op {
            Op::Addx((cyc, val)) => {
                if cyc == 0 {
                    // println!("first add cycle, inc op & cont");
                    self.cur_op = Op::Addx((1, val));
                    self.cyc += 1;
                    return Some(true);
                } else {
                    // println!("second add cycle, finish up");
                    self.r1 += val;
                    if let Some(op) = self.ops.pop_front() {
                        self.cur_op = op;
                        self.cyc += 1;
                        return Some(true);
                    } else {
                        return None;
                    }
                }
            }
            Op::Noop => {
                // println!("noop, do nothing");
                if let Some(op) = self.ops.pop_front() {
                    self.cur_op = op;
                    self.cyc += 1;
                    return Some(true);
                } else {
                    return None;
                }
            }
        }
    }

    fn add_break(&mut self, b: usize) {
        self.breaks.push(b);
    }

    fn check_break(&self) -> Option<(usize, i64)> {
        // println!(
        //     "DEBUG: CYC={}, OP={:?}, R1={}",
        //     self.cyc, self.cur_op, self.r1
        // );
        if self.breaks.contains(&self.cyc) {
            return Some((self.cyc, self.r1));
        } else {
            return None;
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Addx((usize, i64)),
    Noop,
}

fn parse_input(s: &str) -> VecDeque<Op> {
    let mut out = VecDeque::new();
    for row in s.split_terminator("\n") {
        let elems: Vec<&str> = row.split(" ").collect();
        match elems[0] {
            "addx" => out.push_back(Op::Addx((0, elems[1].parse().unwrap_or_default()))),
            _ => out.push_back(Op::Noop),
        }
    }
    out
}

fn solve_part1(s: &str) -> i64 {
    let ops = parse_input(s);
    let mut cpu = Cpu::new(ops);
    // println!("CPU intialized as: {:?}", cpu);
    for b in [20, 60, 100, 140, 180, 220] {
        cpu.add_break(b);
    }
    let mut sum = 0;
    loop {
        if let Some(diag) = cpu.check_break() {
            let val = diag.0 as i64 * diag.1;
            println!("BREAK: cyc={}, r1={}, val={}", diag.0, diag.1, val);
            sum += val;
        }
        if let None = cpu.do_op() {
            break;
        }
    }
    sum
}

fn solve_part2(_s: &str) -> usize {
    0
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
