use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 1)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}
fn main() {
    let cli_args = Cli::parse();

    let input = &fs::read_to_string(cli_args.input).unwrap();
    print_answer(school_girl(input));
    print_answer(chest_hair(input));
}

/// here's a solution for schoolgirls
fn school_girl(input: &str) -> Vec<(usize, i32)> {
    let num_groups = input.split("\n\n");
    let mut pos = 0;
    let mut ans = Vec::new();
    for group in num_groups {
        let sum = sum_str_list(group);
        ans.push((pos, sum));
        pos += 1;
    }
    // okay I just re-used the sort solution from the
    // better answer here to solve part 2
    ans.sort_by(|a, b| (b.1).cmp(&a.1));
    ans
}

fn sum_str_list(s: &str) -> i32 {
    let nums = s.split("\n");
    let mut sum: i32 = 0;
    for n in nums {
        // since we're summing, or_default works here (giving 0 on empty)
        // if that causes issues in the future then will need better handling
        let n: i32 = n.parse().unwrap_or_default();
        sum += n;
    }
    sum
}

/// and here's a solution with some chest hair
fn chest_hair(input: &str) -> Vec<(usize, i32)> {
    let num_groups = input.split("\n\n");
    let mut ans = num_groups
        // split each elf into indiv entries
        .map(|g| g.split("\n"))
        .map(|nums| {
            // convert str -> int
            nums.map(|n| n.parse::<i32>().unwrap_or_default())
                // sum each elf's totals
                .sum::<i32>()
        })
        // need an enumerator to track the index
        .enumerate()
        // if we only wanted to find the elf with the heaviest load we can
        // use the following reduce approach, but that won't work for part two
        // .reduce(|a, i| if a.1 > i.1 { a } else { i }).unwrap()
        .collect::<Vec<(usize, i32)>>();

    // instead we'll collect all of our answers and then sort them by their values
    ans.sort_by(|a, b| (b.1).cmp(&a.1));
    ans
}

fn print_answer(ans: Vec<(usize, i32)>) {
    println!(
        "Top 3 Elves are in positions {}, {} and {} with {}, {} and {} calories for {} total",
        ans[0].0,
        ans[1].0,
        ans[2].0,
        ans[0].1,
        ans[1].1,
        ans[2].1,
        ans[0].1 + ans[1].1 + ans[2].1
    );
}
