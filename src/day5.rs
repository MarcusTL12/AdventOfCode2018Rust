use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

pub const PARTS: [fn(); 2] = [part1, part2];

fn reducetot(inp: String) -> usize {
    (0..)
        .scan((inp, false), |state, _| {
            if !state.1 {
                let mut done = true;
                let nstate: String = state
                    .0
                    .chars()
                    .zip(state.0.chars().skip(1).chain([' '].iter().cloned()))
                    .scan(false, |state, (a, b)| {
                        if *state {
                            *state = false;
                            Some(None)
                        } else {
                            *state = (a.to_ascii_uppercase()
                                == b.to_ascii_uppercase())
                                && (a.is_uppercase() ^ b.is_uppercase());
                            //
                            if *state {
                                done = false;
                                Some(None)
                            } else {
                                Some(Some(a))
                            }
                        }
                    })
                    .filter_map(|a| a)
                    .collect();
                //
                state.0 = nstate;
                state.1 = done;
                if state.1 {
                    Some(Some(state.0.clone()))
                } else {
                    Some(None)
                }
            } else {
                None
            }
        })
        .fold(None, |_, val| Some(val))
        .unwrap()
        .unwrap()
        .len()
}

fn part1() {
    let inp: String = BufReader::new(
        File::open("inputfiles/day5.txt").expect("File is fucked!"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| l.chars().collect::<Vec<_>>())
    .flatten()
    .collect();
    //
    let ans = reducetot(inp);
    //
    println!("{:?}", ans);
}

fn part2() {
    let inp: String = BufReader::new(
        File::open("inputfiles/day5.txt").expect("File is fucked!"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| l.chars().collect::<Vec<_>>())
    .flatten()
    .collect();
    //
    let alphabet: HashSet<_> =
        inp.chars().map(|c| c.to_ascii_uppercase()).collect();
    //
    let ans = alphabet
        .iter()
        .map(|&c| {
            reducetot(
                inp.chars()
                    .filter(|x| x.to_ascii_uppercase() != c)
                    .collect(),
            )
        })
        .min()
        .unwrap();
    //
    println!("{:?}", ans);
}
