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

#[derive(Debug, Clone)]
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
    segments: Vec<Coords>,
    // head: Coords,
    // tail: Coords,
    tail_hist: Vec<Coords>,
}

impl State {
    fn new(len: usize) -> Self {
        let mut segments = Vec::new();
        for _ in 0..len {
            segments.push(Coords { x: 0, y: 0 })
        }
        State {
            segments,
            // head: Coords { x: 0, y: 0 },
            // tail: Coords { x: 0, y: 0 },
            tail_hist: vec![Coords { x: 0, y: 0 }],
        }
    }

    fn move_segment(&mut self, pos: usize, dir: Dir) {
        match dir {
            Dir::U => self.segments[pos].y += 1,
            Dir::D => self.segments[pos].y -= 1,
            Dir::R => self.segments[pos].x += 1,
            Dir::L => self.segments[pos].x -= 1,
            Dir::UR => {
                self.segments[pos].y += 1;
                self.segments[pos].x += 1
            }
            Dir::UL => {
                self.segments[pos].y += 1;
                self.segments[pos].x -= 1
            }
            Dir::DR => {
                self.segments[pos].y -= 1;
                self.segments[pos].x += 1
            }
            Dir::DL => {
                self.segments[pos].y -= 1;
                self.segments[pos].x -= 1
            }
            Dir::NS => (),
        }
        if pos + 1 == self.segments.len() {
            self.tail_hist.push(self.segments[pos].clone())
        }
    }

    fn calc_tail_move(&self, pos: usize) -> Dir {
        let res: Dir;
        if (self.segments[pos - 1].x - self.segments[pos].x).abs() <= 1
            && (self.segments[pos - 1].y - self.segments[pos].y).abs() <= 1
        {
            // touching, no move needed
            res = Dir::NS;
        } else {
            // not touching, check straight vs diagonal move
            if (self.segments[pos - 1].x != self.segments[pos].x)
                && (self.segments[pos - 1].y != self.segments[pos].y)
            {
                // diagonal move needed
                let dx = self.segments[pos - 1].x - self.segments[pos].x;
                let dy = self.segments[pos - 1].y - self.segments[pos].y;
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
                if self.segments[pos - 1].x != self.segments[pos].x {
                    if self.segments[pos - 1].x - self.segments[pos].x > 0 {
                        res = Dir::R;
                    } else {
                        res = Dir::L;
                    }
                } else if self.segments[pos - 1].y - self.segments[pos].y > 0 {
                    res = Dir::U;
                } else {
                    res = Dir::D;
                }
            }
        }
        res
    }

    fn do_steps(&mut self, steps: Vec<Dir>) {
        steps
            .into_iter()
            .map(|x| {
                for pos in 0..(self.segments.len()) {
                    if pos == 0 {
                        self.move_segment(pos, x.clone())
                    } else {
                        self.move_segment(pos, self.calc_tail_move(pos))
                    }
                }
            })
            .for_each(drop);
    }
}

fn parse_input(s: &str) -> Vec<Dir> {
    let mut out = Vec::new();
    for row in s.split_terminator('\n') {
        let elems: Vec<&str> = row.split(' ').collect();
        let steps = elems[1].parse().unwrap_or_default();

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
    let mut state = State::new(2);
    state.do_steps(steps);
    let mut unique_locs: HashSet<Coords> = HashSet::new();
    for loc in state.tail_hist {
        unique_locs.insert(loc);
    }

    unique_locs.len()
}

fn solve_part2(s: &str) -> usize {
    let steps = parse_input(s);
    let mut state = State::new(10);
    state.do_steps(steps);
    let mut unique_locs: HashSet<Coords> = HashSet::new();
    for loc in state.tail_hist {
        unique_locs.insert(loc);
    }

    unique_locs.len()
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
