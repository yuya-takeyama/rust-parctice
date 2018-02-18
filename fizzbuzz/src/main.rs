use std::env::Args;
use std::env::args;
use std::string::String;

fn main() {
    for i in 1..max(args()) {
        let mut s = String::from("");

        if i % 3 == 0 {
            s.push_str("Fizz");
        }
        if i % 5 == 0 {
            s.push_str("Buzz");
        }
        if s == "" {
            s.push_str(&i.to_string());
        }

        println!("{}", s);
    }
}

const DEFAULT_MAX: u32 = 101;

fn max(args: Args) -> u32 {
    let mut args = args;
    return args.nth(1)
        .map(|s| s.parse().unwrap_or(DEFAULT_MAX))
        .unwrap_or(DEFAULT_MAX);
}
