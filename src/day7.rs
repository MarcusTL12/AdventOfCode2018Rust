use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

use priority_queue::PriorityQueue;

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn makegraph(
    filename: &str,
) -> (HashMap<char, HashSet<char>>, HashMap<char, HashSet<char>>) {
    let reg =
        Regex::new(r"Step (\w) must be finished before step (\w) can begin\.")
            .expect("Regex is broken!");
    //
    let fileparsed =
        BufReader::new(File::open(filename).expect("File is fucked"))
            .lines()
            .map(|l| l.expect("Line is fucked"))
            .map(|l| {
                if let Some(c) = reg.captures(&l) {
                    (c[1].chars().next().unwrap(), c[2].chars().next().unwrap())
                } else {
                    panic!("Ill formated line")
                }
            });
    //
    let mut forward: HashMap<_, HashSet<_>> = HashMap::new();
    let mut reverse: HashMap<_, HashSet<_>> = HashMap::new();
    //
    for (a, b) in fileparsed {
        if let Some(x) = forward.get_mut(&a) {
            x.insert(b);
        } else {
            let mut nset = HashSet::new();
            nset.insert(b);
            forward.insert(a, nset);
        }
        if let Some(x) = reverse.get_mut(&b) {
            x.insert(a);
        } else {
            let mut nset = HashSet::new();
            nset.insert(a);
            reverse.insert(b, nset);
        }
    }
    //
    let f_keys: HashSet<_> = forward.keys().cloned().collect();
    let r_keys: HashSet<_> = reverse.keys().cloned().collect();
    for &k in r_keys.difference(&f_keys) {
        forward.insert(k, HashSet::new());
    }
    for &k in f_keys.difference(&r_keys) {
        reverse.insert(k, HashSet::new());
    }
    //
    (forward, reverse)
}

fn part1() {
    let (graph, dual) = makegraph("inputfiles/day7/input.txt");
    //
    let roots: HashSet<_> =
        graph
            .keys()
            .cloned()
            .collect::<HashSet<_>>()
            .difference(&graph.values().fold(HashSet::new(), |state, v| {
                state.union(v).cloned().collect()
            }))
            .cloned()
            .collect();
    //
    let mut queue: PriorityQueue<_, _> =
        roots.iter().map(|&c| (c, -(c as i32))).collect();
    //
    let mut order = Vec::new();
    let mut visited = HashSet::new();
    //
    while let Some((node, _)) = queue.pop() {
        order.push(node);
        visited.insert(node);
        //
        for &c in graph[&node].iter() {
            if dual[&c].difference(&visited).collect::<Vec<_>>().is_empty() {
                queue.push(c, -(c as i32));
            }
        }
    }
    //
    let ans: String = order.iter().collect();
    println!("{}", ans);
}

fn part2() {
    let (graph, dual) = makegraph("inputfiles/day7/input.txt");
    //
    let roots: HashSet<_> =
        graph
            .keys()
            .cloned()
            .collect::<HashSet<_>>()
            .difference(&graph.values().fold(HashSet::new(), |state, v| {
                state.union(v).cloned().collect()
            }))
            .cloned()
            .collect();
    let &product =
        dual.keys()
            .cloned()
            .collect::<HashSet<_>>()
            .difference(&dual.values().fold(HashSet::new(), |state, v| {
                state.union(v).cloned().collect()
            }))
            .next()
            .unwrap();
    //
    let mut queue: PriorityQueue<_, _> =
        roots.iter().map(|&c| (c, -(c as i32))).collect();
    let mut done: HashSet<char> = HashSet::new();
    //
    let mut workers = 5;
    const BASETIME: usize = 60;
    //
    let mut jobs = HashMap::new();
    //
    let mut time_elapsed = 0;
    //
    while !done.contains(&product) {
        while workers > 0 && !queue.is_empty() {
            if let Some((node, _)) = queue.pop() {
                workers -= 1;
                jobs.insert(node, BASETIME + node as usize - 64);
            } else {
                panic!("What the hell?")
            }
        }
        //
        let mut mtime = -1;
        //
        for (_, &t) in jobs.iter() {
            if mtime == -1 || (t as i64) < mtime {
                mtime = t as i64;
            }
        }
        //
        time_elapsed += mtime;
        //
        let mut just_done = Vec::new();
        //
        let ks: Vec<_> = jobs.keys().cloned().collect();
        for k in ks {
            if let Some(x) = jobs.get_mut(&k) {
                *x -= mtime as usize;
            }
            if jobs[&k] == 0 {
                just_done.push(k);
                jobs.remove(&k);
                workers += 1;
            }
        }
        //
        for node in just_done {
            done.insert(node);
            for &c in graph[&node].iter() {
                if dual[&c].difference(&done).collect::<Vec<_>>().is_empty() {
                    queue.push(c, -(c as i32));
                }
            }
        }
    }
    //
    println!("{}", time_elapsed);
}
