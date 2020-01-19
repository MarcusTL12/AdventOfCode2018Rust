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

fn offsetnode<T>(
    list: &Vec<(usize, usize, T)>,
    mut nodeind: usize,
    mut i: isize,
) -> usize {
    if i < 0 {
        while i < 0 {
            nodeind = list[nodeind].0;
            i += 1;
        }
        0
    } else if i > 0 {
        while i > 0 {
            nodeind = list[nodeind].1;
            i -= 1;
        }
        nodeind
    } else {
        nodeind
    }
}

fn insertnode<T>(
    list: &mut Vec<(usize, usize, T)>,
    emptyslots: &mut VecDeque<usize>,
    node: usize,
    v: T,
) {
    let l = list.len();
    let leftnode = list[node].0;
    let rightnode = list[node].1;
    let nnode = (leftnode, rightnode, v);
    let nind = match emptyslots.pop_front() {
        Some(i) => i,
        None => l,
    };
    if nind == l {
        list.push(nnode);
    }
    list[leftnode].1 = nind;
    list[rightnode].0 = nind;
}

fn removenode<T>(
    list: &mut Vec<(usize, usize, T)>,
    emptyslots: &mut VecDeque<usize>,
    node: usize
) {
    emptyslots.push_back(node);
    list[node].0 = node.1;
    list[node.1].0 = node.0;
}

fn part2() {
    const PLAYERS: usize = 463;
    const LASTMARBLE: usize = 71787;
    //
    let mut marbles = vec![(0, 0, 0)];
    let mut emptyslots = VecDeque::<usize>::new();
    //
    let mut scores: Vec<_> = (0..PLAYERS).map(|_| 0).collect();
    //
    let mut curmarble = 0;
    //
    // for marble in 1..LASTMARBLE + 1 {
    //     if marble % 23 != 0 {
    //         curmarble = offsetnode(&marbles, curmarble, 1);
    //         insertnode(&mut marbles, &mut emptyslots, &mut curmarble, marble);
    //     } else {
    //         scores[marble % PLAYERS] += marble;
    //         scores[marble % PLAYERS] += curmarble.2;
    //         curmarble = offsetnode(&marbles, &curmarble, -7);
    //         removenode(&mut marbles, &mut emptyslots, &mut curmarble)
    //     }
    // }
    let winner = scores
        .into_iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .1;
    //
    println!("{}", winner);
}
