use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let reg1 = Regex::new(r"initial state: ([.#]+)").expect("Regex is broken");
    let reg2 = Regex::new(r"([.#]+) => ([.#])").expect("Regex is broken");
    //
    let mut inpf = String::new();
    File::open("inputfiles/day12.txt")
        .expect("File is broken")
        .read_to_string(&mut inpf)
        .expect("Could not read to string");
    //
    let state: Vec<_> = reg1.captures(&inpf).unwrap()[1]
        .chars()
        .map(|c| c == '#')
        .collect();
    //
    let growmap: HashMap<_, _> = reg2
        .captures_iter(&inpf)
        .map(|cap| {
            let a: Vec<_> = cap[1].chars().map(|c| c == '#').collect();
            let b = cap[2].chars().next().unwrap() == '#';
            (a, b)
        })
        .collect();
    //
    const LEFTPADDING: isize = 20;
    const RIGHTPADDING: isize = 40;
    //
    let mut buffer_a: Vec<_> = (-LEFTPADDING
        ..state.len() as isize + RIGHTPADDING)
        .map(|_| false)
        .collect();
    let mut buffer_b = buffer_a.clone();
    //
    for (target, &from) in buffer_a
        [LEFTPADDING as usize..LEFTPADDING as usize + state.len()]
        .iter_mut()
        .zip(state.iter())
    {
        *target = from;
    }
    //
    for _ in 0..20 {
        for i in 2..buffer_a.len() - 2 {
            buffer_b[i] = growmap[&buffer_a[i - 2..i + 3]];
        }
        let temp = buffer_a;
        buffer_a = buffer_b;
        buffer_b = temp;
    }
    //
    let ans: isize = buffer_a
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v {
                Some(i as isize - LEFTPADDING)
            } else {
                None
            }
        })
        .sum();
    //
    println!("{}", ans);
}

fn _showstate(buffer: &Vec<bool>) {
    for &v in buffer {
        print!("{}", if v { '#' } else { '.' });
    }
    println!();
}

fn part2() {
    let reg1 = Regex::new(r"initial state: ([.#]+)").expect("Regex is broken");
    let reg2 = Regex::new(r"([.#]+) => ([.#])").expect("Regex is broken");
    //
    let mut inpf = String::new();
    File::open("inputfiles/day12.txt")
        .expect("File is broken")
        .read_to_string(&mut inpf)
        .expect("Could not read to string");
    //
    let state: Vec<_> = reg1.captures(&inpf).unwrap()[1]
        .chars()
        .map(|c| c == '#')
        .collect();
    //
    let growmap: HashMap<_, _> = reg2
        .captures_iter(&inpf)
        .map(|cap| {
            let a: Vec<_> = cap[1].chars().map(|c| c == '#').collect();
            let b = cap[2].chars().next().unwrap() == '#';
            (a, b)
        })
        .collect();
    //
    const LEFTPADDING: isize = 5;
    const RIGHTPADDING: isize = 200;
    //
    let mut buffer_a: Vec<_> = (-LEFTPADDING
        ..state.len() as isize + RIGHTPADDING)
        .map(|_| false)
        .collect();
    let mut buffer_b = buffer_a.clone();
    //
    for (target, &from) in buffer_a
        [LEFTPADDING as usize..LEFTPADDING as usize + state.len()]
        .iter_mut()
        .zip(state.iter())
    {
        *target = from;
    }
    //
    fn metric(buffer: &Vec<bool>) -> isize {
        buffer
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| {
                if v {
                    Some(i as isize - LEFTPADDING)
                } else {
                    None
                }
            })
            .sum()
    }
    //
    fn _freespace(buffer: &Vec<bool>) -> (usize, usize) {
        let leftmost = buffer
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .min()
            .unwrap();
        let rightmost = buffer
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .max()
            .unwrap();
        (leftmost, buffer.len() - rightmost)
    }
    //
    let mut prev_metric = metric(&buffer_a);
    //
    for _ in 0..200 {
        for i in 2..buffer_a.len() - 2 {
            buffer_b[i] = growmap[&buffer_a[i - 2..i + 3]];
        }
        let temp = buffer_a;
        buffer_a = buffer_b;
        buffer_b = temp;
        //
        let nmetric = metric(&buffer_a);
        println!("+ {} \t\t->\t {}", nmetric - prev_metric, nmetric);
        prev_metric = nmetric;
    }
    //
    println!("{}", metric(&buffer_a));
    // println!("{:?}", freespace(&buffer_a));
    // showstate(&buffer_a);
}
