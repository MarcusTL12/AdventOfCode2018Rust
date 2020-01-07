use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDateTime;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let reg1 = Regex::new(r"\[(.+)\] (.+)").expect("Regex is broken!");
    let reg2 =
        Regex::new(r"Guard #(\d+) begins shift").expect("Regex is broken!");
    let mut inp: Vec<_> = BufReader::new(
        File::open("inputfiles/day4/input.txt").expect("File is fucked!"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| {
        let c = reg1.captures(&l).expect("Ill formated input!");
        (
            NaiveDateTime::parse_from_str(&c[1], "%Y-%m-%d %H:%M")
                .expect("Ill formated datetime!"),
            String::from(&c[2]),
        )
    })
    .collect();
    inp.sort_by(|(a, _), (b, _)| a.cmp(b));
    //
    let mut timetable: HashMap<_, (usize, Vec<_>)> = HashMap::new();
    let mut sleepcount = HashMap::new();
    //
    let mut fell_asleep = 0;
    let mut curday = NaiveDate::parse_from_str("0001-01-01", "%Y-%m-%d")
        .expect("Ill formated date!");
    //
    for (d, action) in inp {
        if action == "falls asleep" {
            fell_asleep = d.minute();
        } else if action == "wakes up" {
            let woke_up = d.minute();
            if let Some(x) = timetable.get_mut(&curday) {
                for min in fell_asleep..woke_up {
                    x.1[min as usize] = true;
                }
                if let Some(x) = sleepcount.get_mut(&x.0) {
                    *x += (fell_asleep..woke_up).len();
                } else {
                    sleepcount.insert(x.0, (fell_asleep..woke_up).len());
                }
            }
        } else if let Some(c) = reg2.captures(&action) {
            curday = (d + Duration::seconds(60 * 60)).date();
            let guard = c[1].parse().unwrap();
            let table = [false].repeat(60);
            timetable.insert(curday, (guard, table));
        } else {
            panic!("Input is fucked!");
        }
    }
    //
    let sleepiest_guard = sleepcount
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;
    //
    let sleepiest_minute = (0..60)
        .map(|i| {
            (
                timetable
                    .iter()
                    .filter(|(_, (guard, _))| guard == sleepiest_guard)
                    .fold(
                        0,
                        |state, (_, (_, table))| {
                            if table[i] {
                                state + 1
                            } else {
                                state
                            }
                        },
                    ),
                i,
            )
        })
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .unwrap()
        .1;
    //
    println!("{}", sleepiest_guard * sleepiest_minute);
}

fn part2() {
    let reg1 = Regex::new(r"\[(.+)\] (.+)").expect("Regex is broken!");
    let reg2 =
        Regex::new(r"Guard #(\d+) begins shift").expect("Regex is broken!");
    let mut inp: Vec<_> = BufReader::new(
        File::open("inputfiles/day4/input.txt").expect("File is fucked!"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| {
        let c = reg1.captures(&l).expect("Ill formated input!");
        (
            NaiveDateTime::parse_from_str(&c[1], "%Y-%m-%d %H:%M")
                .expect("Ill formated datetime!"),
            String::from(&c[2]),
        )
    })
    .collect();
    inp.sort_by(|(a, _), (b, _)| a.cmp(b));
    //
    let mut timetable: HashMap<_, (usize, Vec<_>)> = HashMap::new();
    //
    let mut fell_asleep = 0;
    let mut curday = NaiveDate::parse_from_str("0001-01-01", "%Y-%m-%d")
        .expect("Ill formated date!");
    //
    for (d, action) in inp {
        if action == "falls asleep" {
            fell_asleep = d.minute();
        } else if action == "wakes up" {
            let woke_up = d.minute();
            if let Some(x) = timetable.get_mut(&curday) {
                for min in fell_asleep..woke_up {
                    x.1[min as usize] = true;
                }
            }
        } else if let Some(c) = reg2.captures(&action) {
            curday = (d + Duration::seconds(60 * 60)).date();
            let guard = c[1].parse().unwrap();
            let table = [false].repeat(60);
            timetable.insert(curday, (guard, table));
        } else {
            panic!("Input is fucked!");
        }
    }
    //
    let sleepdata = timetable
        .iter()
        .scan(
            (HashMap::new(), timetable.len()),
            |state: &mut (HashMap<_, Vec<_>>, _), (_, (guard, table))| {
                if let Some(v) = state.0.get_mut(guard) {
                    for i in 0..60 {
                        if table[i] {
                            v[i] += 1;
                        }
                    }
                } else {
                    state.0.insert(
                        *guard,
                        table.iter().map(|&x| x as usize).collect(),
                    );
                }
                state.1 -= 1;
                Some(match state.1 {
                    0 => Some(state.0.clone()),
                    _ => None,
                })
            },
        )
        .fold(None, |_, x| x)
        .unwrap();
    //
    let ans = sleepdata
        .iter()
        .map(|(guard, minutes)| {
            (
                guard,
                minutes
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .unwrap(),
            )
        })
        .max_by(|(_, (_, a)), (_, (_, b))| a.cmp(b))
        .unwrap();
    //
    println!("{}", ans.0 * (ans.1).0);
}
