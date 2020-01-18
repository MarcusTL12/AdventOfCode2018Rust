use std::fs::File;
use std::io::{BufRead, BufReader};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let tree: Vec<usize> = BufReader::new(
        File::open("inputfiles/day8/input.txt").expect("File is fucked"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked"))
    .next()
    .unwrap()
    .split(' ')
    .map(|s| s.parse().unwrap())
    .collect();
    //
    fn rec(tree: &Vec<usize>, i: usize) -> (usize, usize) {
        let amt_children = tree[i];
        let amt_meta = tree[i + 1];
        //
        let mut offset = 2;
        let mut data = 0;
        //
        for _ in 0..amt_children {
            let (ndata, noffset) = rec(tree, i + offset);
            offset += noffset;
            data += ndata;
        }
        for _ in 0..amt_meta {
            data += tree[i + offset];
            offset += 1;
        }
        //
        (data, offset)
    }
    //
    let (ans, _) = rec(&tree, 0);
    println!("{}", ans);
}

fn part2() {
    let tree: Vec<usize> = BufReader::new(
        File::open("inputfiles/day8/input.txt").expect("File is fucked"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked"))
    .next()
    .unwrap()
    .split(' ')
    .map(|s| s.parse().unwrap())
    .collect();
    //
    fn rec(tree: &Vec<usize>, i: usize) -> (usize, usize) {
        let amt_children = tree[i];
        let amt_meta = tree[i + 1];
        //
        let mut offset = 2;
        //
        let children: Vec<_> = (0..amt_children)
            .map(|_| {
                let (data, noffset) = rec(tree, i + offset);
                offset += noffset;
                data
            })
            .collect();
        //
        let meta = &tree[i + offset..i + offset + amt_meta];
        offset += amt_meta;
        //
        let data = if amt_children == 0 {
            meta.iter().sum()
        } else {
            meta.iter().filter_map(|&j| children.get(j - 1)).sum()
        };
        //
        (data, offset)
    }
    //
    let (ans, _) = rec(&tree, 0);
    println!("{}", ans);
}
