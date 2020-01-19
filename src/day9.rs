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
        nodeind
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
    let rightnode = list[node].1;
    let nnode = (node, rightnode, v);
    let nind = match emptyslots.pop_front() {
        Some(i) => i,
        None => l,
    };
    if nind == l {
        list.push(nnode);
    } else {
        list[nind] = nnode;
    }
    list[node].1 = nind;
    list[rightnode].0 = nind;
}

fn removenode<T>(
    list: &mut Vec<(usize, usize, T)>,
    emptyslots: &mut VecDeque<usize>,
    node: usize
) -> usize {
    emptyslots.push_back(node);
    let leftnode = list[node].0;
    let rightnode = list[node].1;
    list[leftnode].1 = list[node].1;
    list[rightnode].0 = list[node].0;
    rightnode
}

fn _showlist<T: std::fmt::Debug>(list: &Vec<(usize, usize, T)>, node: usize) {
    print!("List: {:?}", list[node].2);
    let mut i = offsetnode(list, node, 1);
    while i != node {
        print!(" -> {:?}", list[i].2);
        i = offsetnode(list, i, 1);
    }
    println!();
}

fn part2() {
    const PLAYERS: usize = 463;
    const LASTMARBLE: usize = 7178700;
    //
    let mut marbles = vec![(0, 0, 0)];
    let mut emptyslots = VecDeque::<usize>::new();
    //
    let mut scores: Vec<_> = (0..PLAYERS).map(|_| 0).collect();
    //
    let mut curmarble = 0;
    //
    for marble in 1..LASTMARBLE + 1 {
        if marble % 23 != 0 {
            curmarble = offsetnode(&marbles, curmarble, 1);
            insertnode(&mut marbles, &mut emptyslots, curmarble, marble);
            curmarble = offsetnode(&marbles, curmarble, 1);
        } else {
            scores[marble % PLAYERS] += marble;
            curmarble = offsetnode(&marbles, curmarble, -7);
            scores[marble % PLAYERS] += marbles[curmarble].2;
            curmarble = removenode(&mut marbles, &mut emptyslots, curmarble)
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

