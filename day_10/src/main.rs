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
}

impl Cpu {
    fn new(mut ops: VecDeque<Op>) -> Self {
        Cpu {
            r1: 1,
            cyc: 1,
            cur_op: ops.pop_front().unwrap(),
            ops: ops.clone(),
        }
    }

    fn do_op(&mut self) -> Option<bool> {
        match self.cur_op {
            Op::Addx((cyc, val)) => {
                if cyc == 0 {
                    self.cur_op = Op::Addx((1, val));
                    self.cyc += 1;
                    return Some(true);
                } else {
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

    fn get_state(&self) -> (usize, i64, Op) {
        // println!(
        //     "DEBUG: CYC={}, OP={:?}, R1={}",
        //     self.cyc, self.cur_op, self.r1
        // );
        return (self.cyc, self.r1, self.cur_op.clone());
    }
}

#[derive(Debug, Clone)]
enum Op {
    Addx((usize, i64)),
    Noop,
}

#[derive(Debug)]
struct Crt {
    row_num: usize,
    pos: usize,
    rows: Vec<Vec<bool>>,
}

impl Crt {
    fn new(height: usize, width: usize) -> Self {
        let row = vec![false; width];
        Crt {
            row_num: 0,
            pos: 0,
            rows: vec![row; height],
        }
    }

    fn draw_pixel(&mut self, r1: i64) {
        println!("drawing pixel at row {} pos {}", self.row_num, self.pos);
        if self.pos as i64 >= (r1 - 1) && self.pos as i64 <= (r1 + 1) {
            self.rows[self.row_num][self.pos] = true
        } else {
            self.rows[self.row_num][self.pos] = false
        }
        self.inc_pos()
    }

    fn inc_pos(&mut self) {
        if self.pos < self.rows[0].len() - 1 {
            self.pos += 1
        } else {
            self.row_num += 1;
            self.pos = 0
        }
    }

    fn render(&self) {
        println!("Rendering display\n");
        for row in self.rows.clone() {
            let mut row_string = String::new();
            row.iter().for_each(|x| {
                if *x {
                    row_string.push_str("█")
                } else {
                    row_string.push_str(" ")
                }
            });
            println!("{}", row_string);
        }
    }
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
    let mut sum = 0;
    loop {
        let diag = cpu.get_state();
        if [20, 60, 100, 140, 180, 220].contains(&diag.0) {
            let val = diag.0 as i64 * diag.1;
            // println!(
            //     "op={:?}, cyc={}, r1={}, val={}",
            //     diag.2, diag.0, diag.1, val
            // );
            sum += val;
        }
        if let None = cpu.do_op() {
            break;
        }
    }
    sum
}

fn solve_part2(s: &str) {
    let ops = parse_input(s);
    let mut cpu = Cpu::new(ops);
    let mut crt = Crt::new(6, 40);
    loop {
        let state = cpu.get_state();
        crt.draw_pixel(state.1);
        if let None = cpu.do_op() {
            break;
        }
    }
    crt.render();
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2\n");
    solve_part2(input);
}
