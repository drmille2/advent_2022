use clap::Parser;
use std::{collections::HashSet, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 8)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
struct Forest {
    pub rows: Vec<Vec<usize>>,
    pub columns: Vec<Vec<usize>>,
    pub visible: HashSet<(usize, usize)>,
}

/// panics if items is not evenly divisble by group_len
fn mod_group_items(items: Vec<usize>, modulo: usize) -> Vec<Vec<usize>> {
    let mut inter = Vec::new();

    let mut g = 0;
    while g < modulo {
        inter.push(vec![]);
        g += 1;
    }

    for item in items.into_iter().enumerate() {
        let group_num = item.0 % modulo;
        inter[group_num].push(item.1);
    }

    let mut out = Vec::new();
    for i in inter {
        out.push(i.into_iter().collect::<Vec<usize>>());
    }
    out
}

fn parse_forest(s: &str) -> Forest {
    let mut rows_iter = s.split_terminator("\n").peekable();
    let row_length = rows_iter.peek().unwrap().len();

    let mut rows = Vec::new();
    for row in rows_iter {
        let row: Vec<usize> = String::from(row)
            .chars()
            .flat_map(|x| x.to_digit(10))
            .map(|x| x as usize)
            .collect::<Vec<usize>>();
        rows.push(row);
    }

    let columns = mod_group_items(
        s.chars()
            .flat_map(|x| x.to_digit(10))
            .map(|x| x as usize)
            .collect::<Vec<usize>>(),
        row_length,
    );

    Forest {
        rows,
        columns,
        visible: HashSet::new(),
    }
}

fn get_visible(trees: Vec<Vec<usize>>, is_col: bool, visible: &mut HashSet<(usize, usize)>) {
    let num_lines = trees.len();
    for (line_num, line) in trees.iter().enumerate() {
        let line_len = line.len();
        line.iter().enumerate().fold((0, 0), |a, i| {
            let coords: (usize, usize);
            if is_col {
                coords = (i.0, line_num);
            } else {
                coords = (line_num, i.0);
            };
            if line_num == 0 || line_num == (num_lines - 1) {
                visible.insert(coords);
            } else if i.0 == 0 || i.0 == (line_len - 1) {
                visible.insert(coords);
            } else if i.1 > &a.1 {
                visible.insert(coords);
            };
            if i.1 > &a.1 {
                (i.0, *i.1)
            } else {
                a
            }
        });
    }
    for (line_num, line) in trees.iter().enumerate() {
        let line_len = line.len();
        line.iter().rev().enumerate().fold((0, 0), |a, i| {
            // count backwards this time
            let coords: (usize, usize);
            if is_col {
                coords = (num_lines - i.0 - 1, line_num);
            } else {
                coords = (line_num, num_lines - i.0 - 1);
            };
            if line_num == 0 || line_num == (num_lines - 1) {
                visible.insert(coords);
            } else if i.0 == 0 || i.0 == (line_len - 1) {
                visible.insert(coords);
            } else if i.1 > &a.1 {
                visible.insert(coords);
            };
            if i.1 > &a.1 {
                (i.0, *i.1)
            } else {
                a
            }
        });
    }
}

fn check_north(tree: (usize, usize), forest: &Forest) -> usize {
    let mut distance = 0;
    let mut i = 1;
    while i <= tree.1 {
        let us = forest.columns[tree.0][tree.1];
        let them = forest.columns[tree.0][tree.1 - i];
        if us <= them {
            distance += 1;
            break;
        }
        distance += 1;
        i += 1;
    }
    distance
}

fn check_south(tree: (usize, usize), forest: &Forest) -> usize {
    let mut distance = 0;
    let mut i = 1;
    while i < forest.rows.len() - tree.1 {
        let us = forest.columns[tree.0][tree.1];
        let them = forest.columns[tree.0][tree.1 + i];
        if us <= them {
            distance += 1;
            break;
        }
        distance += 1;
        i += 1;
    }
    distance
}

fn check_west(tree: (usize, usize), forest: &Forest) -> usize {
    let mut distance = 0;
    let mut i = 1;
    while i <= tree.0 {
        let us = forest.rows[tree.1][tree.0];
        let them = forest.rows[tree.1][tree.0 - i];
        if us <= them {
            distance += 1;
            break;
        }
        distance += 1;
        i += 1;
    }
    distance
}

fn check_east(tree: (usize, usize), forest: &Forest) -> usize {
    let mut distance = 0;
    let mut i = 1;
    while i < forest.columns.len() - tree.0 {
        let us = forest.rows[tree.1][tree.0];
        let them = forest.rows[tree.1][tree.0 + i];
        if us <= them {
            distance += 1;
            break;
        }
        distance += 1;
        i += 1;
    }
    distance
}

fn solve_part1(s: &str) -> usize {
    let mut forest = parse_forest(s);
    get_visible(forest.rows, false, &mut forest.visible);
    get_visible(forest.columns, true, &mut forest.visible);
    forest.visible.len()
}

fn solve_part2(s: &str) -> usize {
    let forest = parse_forest(s);
    let height = forest.columns[0].len();
    let width = forest.rows[0].len();
    let mut h = 0;
    let mut w = 0;
    let mut top_score = 0;
    while h < height {
        while w < width {
            let tree = (h, w);
            let s = check_north(tree, &forest)
                * check_south(tree, &forest)
                * check_west(tree, &forest)
                * check_east(tree, &forest);
            if s > top_score {
                top_score = s
            };
            w += 1;
        }
        h += 1;
        w = 0;
    }
    top_score
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
