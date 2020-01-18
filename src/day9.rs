use std::collections::VecDeque;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    const PLAYERS: usize = 463;
    const LASTMARBLE: usize = 71787;
    //
    let mut marbles = vec![0];
    let mut scores: Vec<_> = (0..PLAYERS).map(|_| 0).collect();
    //
    let mut curmarble = 0;
    //
    for marble in 1..LASTMARBLE + 1 {
        if marble % 23 != 0 {
            curmarble = (curmarble + 2) % marbles.len();
            marbles.insert(curmarble, marble);
        } else {
            scores[marble % PLAYERS] += marble;
            if curmarble < 7 {
                curmarble += marbles.len() - 7;
            } else {
                curmarble -= 7;
            }
            scores[marble % PLAYERS] += marbles.remove(curmarble);
        }
    }
    let winner = scores
        .into_iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .1;
    //
    println!("{}", winner);
}

fn _offsetnode<'a, T>(
    list: &'a (Vec<(usize, usize, T)>, VecDeque<usize>),
    mut node: &'a (usize, usize, T),
    mut i: isize,
) -> &'a (usize, usize, T) {
    if i < 0 {
        while i < 0 {
            node = &list.0[node.0];
            i += 1;
        }
        node
    } else if i > 0 {
        while i > 0 {
            node = &list.0[node.1];
            i -= 1;
        }
        node
    } else {
        node
    }
}

fn part2() {
    const PLAYERS: usize = 463;
    const LASTMARBLE: usize = 71787;
    //
    let mut marbles = (vec![(0, 0, 0)], VecDeque::<usize>::new());
    //
    let mut scores: Vec<_> = (0..PLAYERS).map(|_| 0).collect();
    //
    let mut curmarble = 0;
    //
    // for marble in 1..LASTMARBLE + 1 {
    //     if marble % 23 != 0 {
    //         curmarble = (curmarble + 2) % marbles.len();
    //         marbles.insert(curmarble, marble);
    //     } else {
    //         scores[marble % PLAYERS] += marble;
    //         if curmarble < 7 {
    //             curmarble += marbles.len() - 7;
    //         } else {
    //             curmarble -= 7;
    //         }
    //         scores[marble % PLAYERS] += marbles.remove(curmarble);
    //     }
    // }
    // let winner = scores
    //     .into_iter()
    //     .enumerate()
    //     .max_by(|(_, a), (_, b)| a.cmp(b))
    //     .unwrap()
    //     .1;
    // //
    // println!("{}", winner);
}
