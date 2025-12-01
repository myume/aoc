use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc25::read_input;

fn main() {
    let reader = read_input!("day1");

    let result = reader.lines().fold((50, 0, 0), |(acc, pt1, pt2), line| {
        let line = line.unwrap();
        let sign = if line.starts_with('L') { -1 } else { 1 };
        let rotation = line[1..].parse::<i32>().unwrap() * sign;

        let mut pt2 = pt2 + ((acc + rotation) / 100).abs();

        // there are two cases we need to handle in here:
        // 1. integer division rounds up so if we go from 50 -> -10, the division will
        //    round to 0. In that case we actually want to add one since we did pass 0.
        // 2. in the case where we go from 0 -> negative number, we actually don't want to
        //    count it since we don't actually land on zero for each rotation/click, we only land
        //    on the numbers 1 past the starting number.
        if acc + rotation <= 0 && acc > 0 {
            pt2 += 1
        };

        let acc = (acc + rotation).rem_euclid(100);
        let pt1 = pt1 + if acc == 0 { 1 } else { 0 };

        (acc, pt1, pt2)
    });

    println!("Part 1 Solution: {}", result.1);
    println!("Part 2 Solution: {}", result.2);
}
