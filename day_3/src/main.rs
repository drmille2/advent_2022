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

fn to_priority(c: &char) -> Option<i32> {
    let offset: i32 = if c.is_lowercase() { -9 } else { 17 };
    c.to_digit(36).and_then(|x| Some(x as i32 + offset))
}

#[cfg(test)]
mod test {
    use crate::to_priority;

    #[test]
    fn test_to_priority() {
        assert_eq!(to_priority(&'a').unwrap(), 1);
        assert_eq!(to_priority(&'z').unwrap(), 26);
        assert_eq!(to_priority(&'A').unwrap(), 27);
        assert_eq!(to_priority(&'Z').unwrap(), 52);
    }
}
