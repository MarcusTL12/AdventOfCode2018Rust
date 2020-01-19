use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn sizefootprint(pos: &Vec<Complex<i64>>) -> usize {
    let minx = pos.iter().min_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let miny = pos.iter().min_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    let maxx = pos.iter().max_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let maxy = pos.iter().max_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    (Complex::new(maxx, maxy) - Complex::new(minx, miny)).norm_sqr() as usize
}

fn applyvel(pos: &mut Vec<Complex<i64>>, vel: &Vec<Complex<i64>>) {
    for (p, v) in pos.iter_mut().zip(vel.iter()) {
        *p += v;
    }
}

fn applyvelinverse(pos: &mut Vec<Complex<i64>>, vel: &Vec<Complex<i64>>) {
    for (p, v) in pos.iter_mut().zip(vel.iter()) {
        *p -= v;
    }
}

fn renderstars(pos: &Vec<Complex<i64>>) {
    let minx = pos.iter().min_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let miny = pos.iter().min_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    let maxx = pos.iter().max_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let maxy = pos.iter().max_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    for y in miny..maxy + 1 {
        for x in minx..maxx + 1 {
            print!(
                "{}",
                if pos.contains(&Complex::new(x, y)) {
                    '█'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
}

fn part1() {
    let reg = Regex::new(
        r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>",
    )
    .expect("Regex is broken!");
    //
    let (mut pos, vel): (Vec<_>, Vec<_>) = BufReader::new(
        File::open("inputfiles/day10/input.txt").expect("File is fucked"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| {
        let cap = reg.captures(&l).unwrap();
        let vals: Vec<i64> =
            (0..4).map(|i| cap[i + 1].parse().unwrap()).collect();
        (
            Complex::new(vals[0], vals[1]),
            Complex::new(vals[2], vals[3]),
        )
    })
    .unzip();
    //
    let mut lastfootprint = sizefootprint(&pos);
    applyvel(&mut pos, &vel);
    let mut curfootprint = sizefootprint(&pos);
    while curfootprint < lastfootprint {
        applyvel(&mut pos, &vel);
        lastfootprint = curfootprint;
        curfootprint = sizefootprint(&pos);
    }
    applyvelinverse(&mut pos, &vel);
    renderstars(&pos);
}

fn part2() {
    let reg = Regex::new(
        r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>",
    )
    .expect("Regex is broken!");
    //
    let (mut pos, vel): (Vec<_>, Vec<_>) = BufReader::new(
        File::open("inputfiles/day10/input.txt").expect("File is fucked"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| {
        let cap = reg.captures(&l).unwrap();
        let vals: Vec<i64> =
            (0..4).map(|i| cap[i + 1].parse().unwrap()).collect();
        (
            Complex::new(vals[0], vals[1]),
            Complex::new(vals[2], vals[3]),
        )
    })
    .unzip();
    //
    let mut lastfootprint = sizefootprint(&pos);
    applyvel(&mut pos, &vel);
    let mut curfootprint = sizefootprint(&pos);
    let mut i = 0;
    while curfootprint < lastfootprint {
        applyvel(&mut pos, &vel);
        lastfootprint = curfootprint;
        curfootprint = sizefootprint(&pos);
        i += 1;
    }
    println!("{}", i);
}
