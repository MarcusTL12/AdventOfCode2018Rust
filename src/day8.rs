use std::fs::File;
use std::io::{BufRead, BufReader};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let tree: Vec<usize> = BufReader::new(
        File::open("inputfiles/day8/example.txt").expect("File is fucked"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked"))
    .next()
    .unwrap()
    .split(' ')
    .map(|s| s.parse().unwrap())
    .collect();
    //
    fn rec(tree: &Vec<usize>, i: usize) -> (usize, usize) {
        let amt_children = tree[i];
        let amt_meta = tree[i + 1];
        //
        //
        (0, 0)
    }
}

fn part2() {}
