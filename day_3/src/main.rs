use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 3)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

fn main() {
    println!("Hello, world!");
}
