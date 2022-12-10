use clap::Parser;
use std::{collections::HashSet, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 9)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
enum Dir {
    L,
    R,
    U,
    D,
    UR,
    UL,
    DR,
    DL,
    NS,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Coords {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct State {
    head: Coords,
    head_hist: Vec<Coords>,
    tail: Coords,
    tail_hist: Vec<Coords>,
}

impl State {
    fn new() -> Self {
        State {
            head: Coords { x: 0, y: 0 },
            tail: Coords { x: 0, y: 0 },
            head_hist: vec![Coords { x: 0, y: 0 }],
            tail_hist: vec![Coords { x: 0, y: 0 }],
        }
    }

    fn move_head(&mut self, dir: Dir) {
        match dir {
            Dir::U => self.head.y += 1,
            Dir::D => self.head.y -= 1,
            Dir::R => self.head.x += 1,
            Dir::L => self.head.x -= 1,
            Dir::UR => {
                self.head.y += 1;
                self.head.x += 1
            }
            Dir::UL => {
                self.head.y += 1;
                self.head.x -= 1
            }
            Dir::DR => {
                self.head.y -= 1;
                self.head.x += 1
            }
            Dir::DL => {
                self.head.y -= 1;
                self.head.x -= 1
            }
            Dir::NS => (),
        }
        println!("moving head {:?} to {:?}", dir, self.head);
        self.head_hist.push(self.head.clone());
    }

    fn move_tail(&mut self, dir: Dir) {
        match dir {
            Dir::U => self.tail.y += 1,
            Dir::D => self.tail.y -= 1,
            Dir::R => self.tail.x += 1,
            Dir::L => self.tail.x -= 1,
            Dir::UR => {
                self.tail.y += 1;
                self.tail.x += 1
            }
            Dir::UL => {
                self.tail.y += 1;
                self.tail.x -= 1
            }
            Dir::DR => {
                self.tail.y -= 1;
                self.tail.x += 1
            }
            Dir::DL => {
                self.tail.y -= 1;
                self.tail.x -= 1
            }
            Dir::NS => (),
        }
        println!("moving tail {:?} to {:?}", dir, self.tail);
        self.tail_hist.push(self.tail.clone());
    }

    fn calc_tail_move(&self) -> Dir {
        let res: Dir;
        if (self.head.x - self.tail.x).abs() <= 1 && (self.head.y - self.tail.y).abs() <= 1 {
            // touching, no move needed
            res = Dir::NS;
        } else {
            // not touching, check straight vs diagonal move
            // if (self.head.x - self.tail.x).abs() > 1 && (self.head.y - self.tail.y).abs() > 1 {
            if (self.head.x != self.tail.x) && (self.head.y != self.tail.y) {
                // if ((self.head.x - self.tail.x).abs() > 1 && (self.head.y != self.tail.y))
                //     || ((self.head.y - self.tail.y).abs() > 1 && (self.head.x != self.tail.x))
                // {
                // diagonal move needed
                let dx = self.head.x - self.tail.x;
                let dy = self.head.y - self.tail.y;
                println!("calculating diagonal, dx = {}, dy = {}", dx, dy);
                if dx > 0 && dy > 0 {
                    res = Dir::UR;
                } else if dx > 0 && dy < 0 {
                    res = Dir::DR;
                } else if dx < 0 && dy < 0 {
                    res = Dir::DL;
                } else {
                    res = Dir::UL;
                }
            } else {
                // straight move needed
                if self.head.x != self.tail.x {
                    if self.head.x - self.tail.x > 0 {
                        res = Dir::R;
                    } else {
                        res = Dir::L;
                    }
                } else {
                    if self.head.y - self.tail.y > 0 {
                        res = Dir::U;
                    } else {
                        res = Dir::D;
                    }
                }
            }
        }
        res
    }

    fn do_steps(&mut self, steps: Vec<Dir>) {
        steps
            .into_iter()
            .map(|x| {
                self.move_head(x);
                self.move_tail(self.calc_tail_move());
                println!(
                    "dx {}, dy {} \n",
                    self.head.x - self.tail.x,
                    self.head.y - self.tail.y
                );
            })
            .for_each(drop);
    }
}

fn parse_input(s: &str) -> Vec<Dir> {
    let mut out = Vec::new();
    for row in s.split_terminator("\n") {
        let elems: Vec<&str> = row.split(" ").collect();
        let steps = elems[1].parse().unwrap_or_default();

        // let chars: Vec<char> = row.chars().collect();
        // for _ in 0..chars[2].to_digit(10).unwrap_or_default() {
        for _ in 0..steps {
            match elems[0] {
                "L" => out.push(Dir::L),
                "R" => out.push(Dir::R),
                "U" => out.push(Dir::U),
                "D" => out.push(Dir::D),
                _ => (),
            }
        }
    }
    out
}

fn solve_part1(s: &str) -> usize {
    let steps = parse_input(s);
    let mut state = State::new();
    println!("{:?}", steps);
    state.do_steps(steps);
    // println!("final head location {:?}", state.head);
    let mut unique_locs: HashSet<Coords> = HashSet::new();
    // println!("all visited locations {:?}", state.tail_hist);
    for loc in state.tail_hist {
        unique_locs.insert(loc);
    }

    // println!("unique visited locations {:?}", unique_locs);
    unique_locs.len()
}

fn solve_part2(s: &str) -> usize {
    0
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
