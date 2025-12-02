use aoc25::read_input;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut reader = read_input!("day2");

    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");

    // remove newline
    let line = line.trim_end();
    let ranges: Vec<(String, String)> = line
        .split(",")
        .map(|id_range| {
            let range: Vec<&str> = id_range.split("-").collect();
            assert_eq!(range.len(), 2);
            (range[0].to_owned(), range[1].to_owned())
        })
        .collect();

    let result = ranges.into_iter().fold((0, 0), |acc, range| {
        let start = range.0.parse::<u64>().unwrap();
        let end = range.1.parse::<u64>().unwrap();

        // brute force
        // there's probably a math way to do it
        (start..=end).fold(acc, |(pt1, pt2), num| {
            let num_chars = num.to_string();

            let pt1 = if num_chars.len() % 2 == 0
                && num_chars[..num_chars.len() / 2] == num_chars[num_chars.len() / 2..]
            {
                pt1 + num
            } else {
                pt1
            };

            let pt2 = if is_invalid(&num.to_string()) {
                pt2 + num
            } else {
                pt2
            };

            (pt1, pt2)
        })
    });

    println!("Part 1 Solution: {}", result.0);
    println!("Part 2 Solution: {}", result.1);
}

fn is_invalid(s: &str) -> bool {
    (1..=s.len() / 2).any(|step| {
        (0..s.len())
            .step_by(step)
            .all(|i| s[i..(i + step).min(s.len())] == s[..step])
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_invalid_test() {
        assert!(is_invalid("123123"));
        assert!(is_invalid("11"));
        assert!(is_invalid("999"));
        assert!(is_invalid("1010"));
        assert!(is_invalid("11111"));
        assert!(!is_invalid("19190"));
    }
}
