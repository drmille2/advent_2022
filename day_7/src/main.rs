use clap::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 7)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
struct State {
    pub sizes: HashMap<String, usize>,
    pub crumbs: Vec<String>,
}

impl State {
    fn new() -> Self {
        State {
            sizes: HashMap::new(),
            crumbs: Vec::new(),
        }
    }

    fn do_cd(&mut self, dir: String) {
        if dir == ".." {
            self.crumbs.pop();
        } else {
            self.crumbs.push(dir);
        }
    }

    fn add_file(&mut self, size: usize) {
        let mut crumbs = self.crumbs.clone();
        for _ in 0..crumbs.len() {
            let dir = render_wd(&crumbs);
            if let Some(cur_size) = self.sizes.get(&dir) {
                self.sizes.insert(dir, cur_size + size);
            } else {
                self.sizes.insert(dir, size);
            }
            crumbs.pop();
        }
    }

    fn run_cmd(&mut self, cmd: &str) {
        if cmd.starts_with("$ cd") {
            let command = &cmd.split(' ').map(String::from).collect::<Vec<String>>()[2];
            self.do_cd(command.clone());
        } else if cmd.starts_with("$ ls") || cmd.starts_with("dir") {
            return;
        } else {
            let fsize: usize = cmd.split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
            self.add_file(fsize);
        }
    }
}

fn render_wd(path: &Vec<String>) -> String {
    let mut output = String::new();
    for dir in path {
        output.push_str(dir);
        if dir != "/" {
            output.push('/');
        }
    }
    output
}

fn solve_part1(s: &str) -> usize {
    let mut state = State::new();
    let commands = s.split_terminator('\n');
    for cmd in commands {
        state.run_cmd(cmd);
    }

    let mut sum = 0;
    for (_, size) in state.sizes.drain() {
        if size <= 100000 {
            sum += size
        }
    }
    sum
}

fn solve_part2(s: &str) -> usize {
    let mut state = State::new();
    let commands = s.split_terminator('\n');
    for cmd in commands {
        state.run_cmd(cmd);
    }

    let mut sizes_vec: Vec<(&String, &usize)> = state.sizes.iter().collect();
    sizes_vec.sort_by(|a, b| (a.1).cmp(b.1));
    let space_needed = 30000000 - (70000000 - state.sizes.get("/").unwrap());
    for dirs in sizes_vec {
        if dirs.1 >= &space_needed {
            return *dirs.1;
        }
    }
    0
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
