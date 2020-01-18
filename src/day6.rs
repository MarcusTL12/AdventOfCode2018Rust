use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, VecDeque};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn drawmap(map: &Vec<Vec<Option<(usize, usize)>>>) {
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

fn bfs(
    map: &mut Vec<Vec<Option<(usize, usize)>>>,
    from: Complex<usize>,
) {
    fn ucplx_to_icplx(x: Complex<usize>) -> Complex<isize> {
        Complex::new(x.re as isize, x.im as isize)
    }
    fn icplx_to_ucplx(x: Complex<isize>) -> Complex<usize> {
        Complex::new(x.re as usize, x.im as usize)
    }
    //
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
    //
    // println!("{:?}", queue);
    while let Some((npos, nlen)) = queue.pop_front() {
        if let Some((source, restlen)) = map[npos.im][npos.re] {
            println!("{:?}", npos);
            if let Some((_, blen)) = bestyet {
                println!("{:?}", (restlen, nlen, blen));
                if restlen + nlen < blen {
                    endpos = npos;
                    endlen = nlen;
                    bestyet = Some((source, restlen + nlen));
                } else if restlen + nlen == blen {
                    bestyet = None;
                    break;
                }
            } else {
                endpos = npos;
                endlen = nlen;
                bestyet = Some((source, restlen + nlen));
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
        // println!("{:?}", queue);
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
        File::open("inputfiles/day6/example.txt").expect("File is fucked!"),
    )
    .lines()
    .map(|l| l.expect("Line is fucked!"))
    .map(|l| {
        let mut sp = l.split(", ").map(|s| s.parse());
        Complex::new(sp.next().unwrap().unwrap(), sp.next().unwrap().unwrap())
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
    let mut map: Vec<Vec<Option<(usize, usize)>>> = (0..h)
        .map(|_| (0..w).map(|_| None).collect())
        .collect();
    //
    for (i, pos) in inp.iter().enumerate() {
        map[pos.im][pos.re] = Some((i, 0));
    }
    //
    drawmap(&map);
    println!("--------");
    //
    for y in 0..h {
        for x in 0..w {
            println!("x: {}, y: {}", x, y);
            bfs(&mut map, Complex::new(x, y));
        }
    }
    // bfs(&mut map, Complex::new(4, 0));
    //
    drawmap(&map);
}

fn part2() {}
