use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let reg = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)")
        .expect("Regex is fucked");
    //
    let mut claims = HashMap::new();
    //
    BufReader::new(File::open("inputfiles/day3.txt").expect("File is fucked"))
        .lines()
        .map(|x| x.expect("Line is fucked"))
        .for_each(|l| {
            if let Some(c) = reg.captures(&l) {
                let x: usize = c[1].parse().unwrap();
                let y: usize = c[2].parse().unwrap();
                let w: usize = c[3].parse().unwrap();
                let h: usize = c[4].parse().unwrap();
                for i in x..x + w {
                    for j in y..y + h {
                        if let Some(x) = claims.get_mut(&(i, j)) {
                            *x += 1;
                        } else {
                            claims.insert((i, j), 1 as usize);
                        }
                    }
                }
            }
        });
    //
    let ans = claims.values().filter(|&&v| v >= 2).count();
    println!("Area: {}", ans);
}

fn part2() {
    let reg = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
        .expect("Regex is fucked");
    //
    let mut claims = HashMap::new();
    let mut tiles = HashMap::<_, HashSet<_>>::new();
    //
    BufReader::new(File::open("inputfiles/day3.txt").expect("File is fucked"))
        .lines()
        .map(|x| x.expect("Line is fucked"))
        .for_each(|l| {
            if let Some(c) = reg.captures(&l) {
                let id: usize = c[1].parse().unwrap();
                let x: usize = c[2].parse().unwrap();
                let y: usize = c[3].parse().unwrap();
                let w: usize = c[4].parse().unwrap();
                let h: usize = c[5].parse().unwrap();
                for i in x..x + w {
                    for j in y..y + h {
                        if let Some(x) = tiles.get_mut(&(i, j)) {
                            x.insert(id);
                        } else {
                            let mut temp = HashSet::new();
                            temp.insert(id);
                            tiles.insert((i, j), temp);
                        }
                    }
                }
                claims.insert(
                    id,
                    (x..x + w)
                        .map(|i| (y..y + h).map(move |j| (i, j)))
                        .flatten()
                        .collect::<HashSet<_>>(),
                );
            }
        });
    //
    if let Some((ans, _)) = claims
    .iter()
    .find(|(_, claim)| claim.iter().all(|x| tiles[x].len() == 1)) {
        println!("Claim nr: {}", ans);
    } else {
        println!("Found no suitable claims");
    }
}
