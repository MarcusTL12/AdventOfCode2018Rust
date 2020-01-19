use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet, VecDeque};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn _drawmap(map: &Vec<Vec<Option<(usize, usize)>>>) {
    let alphabet: Vec<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for line in map.iter() {
        for cell in line.iter() {
            if let Some((i, l)) = cell {
                if *l == 0 {
                    print!("{}", alphabet[*i].to_ascii_uppercase());
                } else {
                    print!("{}", alphabet[*i]);
                }
            } else {
                print!("{}", '.');
            }
        }
        println!();
    }
}

fn _drawmaplen(map: &Vec<Vec<Option<(usize, usize)>>>) {
    for line in map.iter() {
        for cell in line.iter() {
            if let Some((_, l)) = cell {
                if *l == 0 {
                    print!("{}", l);
                } else {
                    print!("{}", l);
                }
            } else {
                print!("{}", '.');
            }
        }
        println!();
    }
}

fn ucplx_to_icplx(x: Complex<usize>) -> Complex<isize> {
    Complex::new(x.re as isize, x.im as isize)
}

fn icplx_to_ucplx(x: Complex<isize>) -> Complex<usize> {
    Complex::new(x.re as usize, x.im as usize)
}

fn bfs(map: &mut Vec<Vec<Option<(usize, usize)>>>, from: Complex<usize>) {
    let dirs = [
        Complex::new(1, 0),
        Complex::new(0, 1),
        Complex::new(-1, 0),
        Complex::new(0, -1),
    ];
    //
    let mut queue = VecDeque::new();
    queue.push_back((from, 0));
    //
    let mut visited = HashMap::new();
    visited.insert(from, None);
    //
    let mut bestyet: Option<(usize, usize)> = None;
    let mut endpos = from;
    let mut endlen = 0;
    let mut closest = std::usize::MAX;
    //
    while let Some((npos, nlen)) = queue.pop_front() {
        if nlen > closest {
            continue;
        }
        if let Some((source, restlen)) = map[npos.im][npos.re] {
            if let Some((bsource, blen)) = bestyet {
                if restlen + nlen < blen {
                    endpos = npos;
                    endlen = nlen;
                    if closest > restlen + nlen {
                        closest = restlen + nlen;
                        bestyet = Some((source, restlen + nlen));
                    }
                } else if restlen + nlen == blen && source != bsource {
                    bestyet = None;
                }
            } else {
                endpos = npos;
                endlen = nlen;
                if closest > restlen + nlen {
                    closest = restlen + nlen;
                    bestyet = Some((source, restlen + nlen));
                }
            }
        } else {
            for d in dirs.iter() {
                let nnpos = ucplx_to_icplx(npos) + d;
                //
                if nnpos.re >= 0
                    && nnpos.re < map[0].len() as isize
                    && nnpos.im >= 0
                    && nnpos.im < map.len() as isize
                {
                    let nnpos = icplx_to_ucplx(nnpos);
                    let isthereapoint = if let Some((_, blen)) = bestyet {
                        blen >= nlen
                    } else {
                        true
                    };
                    if !visited.contains_key(&nnpos) && isthereapoint {
                        queue.push_back((nnpos, nlen + 1));
                        visited.insert(nnpos, Some(npos));
                    }
                }
            }
        }
    }
    //
    if let Some((source, len)) = bestyet {
        let mut curpos = endpos;
        let mut curlen = len - endlen;
        map[curpos.im][curpos.re] = Some((source, curlen));
        while let Some(npos) = visited[&curpos] {
            curpos = npos;
            curlen += 1;
            map[curpos.im][curpos.re] = Some((source, curlen));
        }
    }
}

fn part1() {
    let mut inp: Vec<Complex<usize>> = BufReader::new(
        File::open("inputfiles/day6/input.txt").expect("File is fucked!"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| {
        let mut sp = l.split(", ").map(|s| s.parse().unwrap());
        Complex::new(sp.next().unwrap(), sp.next().unwrap())
    })
    .collect();
    //
    let (w, h) = {
        let topleft = Complex::new(
            inp.iter().min_by(|a, b| a.re.cmp(&b.re)).unwrap().re,
            inp.iter().min_by(|a, b| a.im.cmp(&b.im)).unwrap().im,
        );
        inp.iter_mut().for_each(|x| *x -= topleft);
        (
            inp.iter().max_by(|a, b| a.re.cmp(&b.re)).unwrap().re + 1,
            inp.iter().max_by(|a, b| a.im.cmp(&b.im)).unwrap().im + 1,
        )
    };
    //
    let mut map: Vec<Vec<Option<(usize, usize)>>> =
        (0..h).map(|_| (0..w).map(|_| None).collect()).collect();
    //
    for (i, pos) in inp.iter().enumerate() {
        map[pos.im][pos.re] = Some((i, 0));
    }
    //
    for y in 0..h {
        for x in 0..w {
            bfs(&mut map, Complex::new(x, y));
        }
    }
    //
    let blacklist: HashSet<_> = (0..w)
        .map(|i| Complex::new(i, 0))
        .chain((0..w).map(|i| Complex::new(i, h - 1)))
        .chain((0..h).map(|i| Complex::new(0, i)))
        .chain((0..h).map(|i| Complex::new(w - 1, i)))
        .filter_map(|c| match map[c.im][c.re] {
            Some((source, _)) => Some(source),
            _ => None,
        })
        .collect();
    //
    let mut areas = HashMap::new();
    //
    for y in 0..h {
        for x in 0..w {
            if let Some((source, _)) = map[y][x] {
                if !blacklist.contains(&source) {
                    if let Some(area) = areas.get_mut(&source) {
                        *area += 1;
                    } else {
                        areas.insert(source, 1);
                    }
                }
            }
        }
    }
    //
    let ans = areas.values().max().unwrap();
    println!("{}", ans);
}

fn sumofdist(sources: &Vec<Complex<usize>>, pos: Complex<usize>) -> usize {
    sources
        .iter()
        .map(|&s| (ucplx_to_icplx(s) - ucplx_to_icplx(pos)).l1_norm())
        .sum::<isize>() as usize
}

fn part2() {
    let mut inp: Vec<Complex<usize>> = BufReader::new(
        File::open("inputfiles/day6/input.txt").expect("File is fucked!"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| {
        let mut sp = l.split(", ").map(|s| s.parse().unwrap());
        Complex::new(sp.next().unwrap(), sp.next().unwrap())
    })
    .collect();
    //
    let (w, h) = {
        let topleft = Complex::new(
            inp.iter().min_by(|a, b| a.re.cmp(&b.re)).unwrap().re,
            inp.iter().min_by(|a, b| a.im.cmp(&b.im)).unwrap().im,
        );
        inp.iter_mut().for_each(|x| *x -= topleft);
        (
            inp.iter().max_by(|a, b| a.re.cmp(&b.re)).unwrap().re + 1,
            inp.iter().max_by(|a, b| a.im.cmp(&b.im)).unwrap().im + 1,
        )
    };
    //
    let ans = (0..h)
        .map(|y| (0..w).map(move |x| Complex::new(x, y)))
        .flatten()
        .map(|pos| sumofdist(&inp, pos))
        .filter(|&l| l < 10000)
        .count();
    //
    println!("{}", ans);
}
