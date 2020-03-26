use digits_iterator::*;

use std::collections::VecDeque;

pub const PARTS: [fn(); 2] = [part1, part2];

fn recipies_iter() -> Box<dyn Iterator<Item = u8>> {
    Box::from(
        vec![3, 7].into_iter().chain(
            (0..)
                .scan((vec![3, 7], [0, 1]), |(recipies, elves), _| {
                    let new: Vec<_> = elves
                        .iter()
                        .map(|&elf| recipies[elf])
                        .sum::<u8>()
                        .digits()
                        .collect();
                    //
                    recipies.extend(new.iter());
                    //
                    for elf in elves.iter_mut() {
                        *elf = (*elf + (1 + recipies[*elf] as usize))
                            % recipies.len();
                    }
                    //
                    Some(new.into_iter())
                })
                .flatten(),
        ),
    )
}

fn part1() {
    const DIGITS: [char; 10] =
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    //
    let ans = recipies_iter()
        .skip(702831)
        .take(10)
        .map(|x| DIGITS[x as usize])
        .collect::<String>();
    //
    println!("{}", ans);
}

fn part2() {
    let sequence: Vec<_> =
        "702831".chars().map(|c| c as u8 - '0' as u8).collect();
    //
    let ans = recipies_iter()
        .scan(VecDeque::from(vec![0; sequence.len()]), |cur_seq, x| {
            cur_seq.pop_front();
            cur_seq.push_back(x);
            if cur_seq.iter().eq(sequence.iter()) {
                None
            } else {
                Some(())
            }
        })
        .count()
        - sequence.len()
        + 1;
    //
    println!("{}", ans);
}
