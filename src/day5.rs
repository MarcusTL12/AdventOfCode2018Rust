use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let inp = String::from("dabAcCaCBAcCcaDA");
    // let inp: String = BufReader::new(
    //     File::open("inputfiles/day5.txt").expect("File is fucked!"),
    // )
    // .lines()
    // .map(|l| l.expect("Line is fucked!"))
    // .map(|l| l.chars().collect::<Vec<_>>())
    // .flatten()
    // .collect();
    //
    let temp =
        inp.chars()
            .zip(inp.chars().skip(1))
            .scan(false, |state, (a, b)| {
                let eq = (a.to_ascii_uppercase()
                    == b.to_ascii_uppercase())
                    && (a.is_uppercase() ^ b.is_uppercase());
                //
                
                
                Some(())
            });
    //
    // let temp: Vec<_> = (0..)
    //     .scan((inp, false), |state, _| {
    //         if !state.1 {
    //             let mut done = true;
    //             let nstate: String = state
    //                 .0
    //                 .chars()
    //                 .chunks(2)
    //                 .into_iter()
    //                 .filter_map(|mut it| {
    //                     let first = it.next().unwrap();
    //                     if let Some(second) = it.next() {
    //                         if (first.to_ascii_uppercase()
    //                             == second.to_ascii_uppercase())
    //                             && (first.is_uppercase()
    //                                 ^ second.is_uppercase())
    //                         {
    //                             done = false;
    //                             None
    //                         } else {
    //                             Some(vec![first, second])
    //                         }
    //                     } else {
    //                         Some(vec![first])
    //                     }
    //                 })
    //                 .flatten()
    //                 .collect();
    //             //
    //             println!("{}", nstate);
    //             state.0 = nstate;
    //             state.1 = done;
    //             if state.1 {
    //                 println!("Hei",);
    //                 Some(Some(state.0.clone()))
    //             } else {
    //                 println!("Hade",);
    //                 Some(None)
    //             }
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // .fold(None, |_, val| Some(val))
    // .unwrap()
    // .unwrap()
    // .len();
    //
    println!("{:?}", temp);
}

fn part2() {}
