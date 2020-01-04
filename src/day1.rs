use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: i32 = BufReader::new(
        File::open("inputfiles/day1.txt").expect("File is fucked"),
    )
    .lines()
    .map(|x| x.expect("Line is fucked").parse::<i32>().unwrap())
    .sum();
    println!("Frequency: {}", ans);
}

fn part2() {
    let ans = BufReader::new(
        File::open("inputfiles/day1.txt").expect("File is fucked"),
    )
    .lines()
    .map(|x| x.expect("Line is fucked").parse().unwrap())
    .collect::<Vec<i32>>()
    .iter()
    .cycle()
    .scan((0, HashSet::new(), false), |state, val| {
        if state.2 {
            None
        } else {
            state.0 += val;
            if state.1.contains(&state.0) {
                state.2 = true;
            } else {
                state.1.insert(state.0);
            }
            Some(state.0)
        }
    })
    .fold(0, |_, v| v);
    println!("Frequency: {}", ans);
}
