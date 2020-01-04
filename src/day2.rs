use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let counts: Vec<_> = BufReader::new(
        File::open("inputfiles/day2.txt").expect("File is fucked"),
    )
    .lines()
    .map(|x| x.expect("Line is fucked"))
    .map(|l| {
        let mut m = HashMap::new();
        l.chars().fold((), |_, val| {
            if let Some(x) = m.get_mut(&val) {
                *x += 1;
            } else {
                m.insert(val, 1);
            }
        });
        (
            m.values().find(|&&v| v == 2).is_some(),
            m.values().find(|&&v| v == 3).is_some(),
        )
    })
    .collect();
    //
    let ans = counts.iter().filter(|&&(v, _)| v).count()
        * counts.iter().filter(|&&(_, v)| v).count();
    println!("Checksum: {}", ans);
}

fn part2() {
    let lines: Vec<_> = BufReader::new(
        File::open("inputfiles/day2.txt").expect("File is fucked"),
    )
    .lines()
    .map(|x| x.expect("Line is fucked"))
    .collect();
    //
    let ((i, j), _) = (0..lines.len())
        .map(|i| (0..lines.len()).map(move |j| (i, j)))
        .flatten()
        .map(|(i, j)| {
            (
                (i, j),
                lines[i]
                    .chars()
                    .zip(lines[j].chars())
                    .filter(|(a, b)| a != b)
                    .count(),
            )
        })
        .find(|&(_, i)| i == 1)
        .unwrap();
    //
    let ans: String = lines[i]
        .chars()
        .zip(lines[j].chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect();
    //
    println!("{}", ans);
}
