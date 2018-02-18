use std::env::args;
use std::string::String;

fn main() {
    for i in 1..max() {
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

fn max() -> u32 {
    let args: Vec<_> = args().collect();

    if args.len() > 1 {
        match args[1].parse::<u32>() {
            Ok(n) => return n + 1,
            Err(_) => return 101,
        };
    } else {
        return 101;
    }
}
