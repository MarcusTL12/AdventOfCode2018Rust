use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Track {
    Vert,
    Hori,
    Rcurve,
    Lcurve,
    Cross,
    Empty,
}

fn _showmap(
    map: &Vec<Vec<Track>>,
    carts: &Vec<(Complex<i32>, Complex<i32>, usize)>,
    trackdict: &HashMap<Track, char>,
    dirdict: &HashMap<Complex<i32>, char>,
) {
    for (i, row) in map.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            print!(
                "{}",
                if let Some((dir, _, _)) =
                    carts
                        .iter()
                        .find(|(_, pos, _)| *pos
                            == Complex::new(j as i32, i as i32))
                {
                    dirdict[dir]
                } else {
                    trackdict[cell]
                }
            );
        }
        println!();
    }
}

fn movecart(
    map: &Vec<Vec<Track>>,
    transmap: &HashMap<Complex<i32>, HashMap<Track, Track>>,
    carts: &mut Vec<(Complex<i32>, Complex<i32>, usize)>,
    cart: usize,
) -> Option<Complex<i32>> {
    let dir = carts[cart].0;
    carts[cart].1 += dir;
    //
    if carts
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if i != cart { Some(v.1) } else { None })
        .any(|v| v == carts[cart].1)
    {
        Some(carts[cart].1)
    } else {
        carts[cart].0 = match transmap[&dir]
            [&map[carts[cart].1.im as usize][carts[cart].1.re as usize]]
        {
            Track::Vert => dir,
            Track::Rcurve => dir * Complex::new(0, 1),
            Track::Lcurve => dir * Complex::new(0, -1),
            Track::Cross => {
                carts[cart].2 = (carts[cart].2 + 1) % 3;
                dir * match carts[cart].2 {
                    0 => Complex::new(0, 1),
                    1 => Complex::new(0, -1),
                    2 => Complex::new(1, 0),
                    _ => panic!("Math stopped working!"),
                }
            }
            _ => {
                println!("Reached dead end at {:?}", carts[cart].1);
                dir
            }
        };
        None
    }
}

fn loadmap(
    filename: &str,
    trackmap: &HashMap<char, Track>,
    dirmap: &HashMap<char, Complex<i32>>,
) -> (Vec<Vec<Track>>, Vec<(Complex<i32>, Complex<i32>, usize)>) {
    let rawmap: Vec<Vec<_>> =
        BufReader::new(File::open(filename).expect("File is broken"))
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.chars().collect())
            .collect();
    //
    let mut carts = Vec::new();
    //
    let map: Vec<Vec<_>> = rawmap
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, cell)| (x, y, cell))
                .map(|(x, y, cell)| {
                    if let Some(&trackpart) = trackmap.get(cell) {
                        trackpart
                    } else {
                        carts.push((
                            dirmap[cell],
                            Complex::new(x as i32, y as i32),
                            0,
                        ));
                        match (x, y) {
                            (0, 0) => Track::Rcurve,
                            (0, y) if y == rawmap.len() - 1 => Track::Lcurve,
                            (x, 0) if x == rawmap[0].len() - 1 => Track::Lcurve,
                            (x, y)
                                if x == rawmap[0].len() - 1
                                    && y == rawmap.len() - 1 =>
                            {
                                Track::Lcurve
                            }
                            (0, y) => {
                                let a = ['/', '|'].contains(&rawmap[y - 1][x]);
                                let b = ['\\', '|'].contains(&rawmap[y + 1][x]);
                                let c = ['/', '\\', '-']
                                    .contains(&rawmap[y][x + 1]);
                                //
                                match (a, b, c) {
                                    (true, true, false) => Track::Vert,
                                    (true, false, true) => Track::Lcurve,
                                    (false, true, true) => Track::Rcurve,
                                    _ => Track::Empty,
                                }
                            }
                            (x, 0) => {
                                let a = ['/', '-'].contains(&rawmap[y][x - 1]);
                                let b = ['\\', '-'].contains(&rawmap[y][x + 1]);
                                let c = ['/', '\\', '|']
                                    .contains(&rawmap[y + 1][x]);
                                //
                                match (a, b, c) {
                                    (true, true, false) => Track::Hori,
                                    (true, false, true) => Track::Lcurve,
                                    (false, true, true) => Track::Rcurve,
                                    _ => Track::Empty,
                                }
                            }
                            (x, y) if x == rawmap[0].len() - 1 => {
                                let a = ['\\', '|'].contains(&rawmap[y - 1][x]);
                                let b = ['/', '|'].contains(&rawmap[y + 1][x]);
                                let c = ['/', '\\', '-']
                                    .contains(&rawmap[y][x - 1]);
                                //
                                match (a, b, c) {
                                    (true, true, false) => Track::Vert,
                                    (true, false, true) => Track::Rcurve,
                                    (false, true, true) => Track::Lcurve,
                                    _ => Track::Empty,
                                }
                            }
                            (x, y) if y == rawmap.len() - 1 => {
                                let a = ['\\', '-'].contains(&rawmap[y][x - 1]);
                                let b = ['/', '-'].contains(&rawmap[y][x + 1]);
                                let c = ['/', '\\', '|']
                                    .contains(&rawmap[y - 1][x]);
                                //
                                match (a, b, c) {
                                    (true, true, false) => Track::Hori,
                                    (true, false, true) => Track::Rcurve,
                                    (false, true, true) => Track::Lcurve,
                                    _ => Track::Empty,
                                }
                            }
                            (x, y) => {
                                let a = ['/', '\\', '-', '+']
                                    .contains(&rawmap[y][x - 1]);
                                let b = ['/', '\\', '-', '+']
                                    .contains(&rawmap[y][x + 1]);
                                let c = ['/', '\\', '|', '+']
                                    .contains(&rawmap[y - 1][x]);
                                let d = ['/', '\\', '|', '+']
                                    .contains(&rawmap[y + 1][x]);
                                //
                                match (a, b, c, d) {
                                    (true, true, false, false) => Track::Hori,
                                    (true, false, true, false) => Track::Rcurve,
                                    (true, false, false, true) => Track::Lcurve,
                                    (false, true, true, false) => Track::Lcurve,
                                    (false, true, false, true) => Track::Rcurve,
                                    (false, false, true, true) => Track::Vert,
                                    (true, true, true, true) => Track::Cross,
                                    _ => Track::Empty,
                                }
                            }
                        }
                    }
                })
                .collect()
        })
        .collect();
    (map, carts)
}

fn part1() {
    let trackmap: HashMap<_, _> = vec![
        ('|', Track::Vert),
        ('-', Track::Hori),
        ('/', Track::Rcurve),
        ('\\', Track::Lcurve),
        ('+', Track::Cross),
        (' ', Track::Empty),
    ]
    .into_iter()
    .collect();
    //
    // let _reversemap: HashMap<_, _> =
    //     trackmap.iter().map(|(&a, &b)| (b, a)).collect();
    //
    let dirmap: HashMap<_, _> = vec![
        ('>', Complex::new(1, 0)),
        ('v', Complex::new(0, 1)),
        ('<', Complex::new(-1, 0)),
        ('^', Complex::new(0, -1)),
    ]
    .into_iter()
    .collect();
    //
    // let _reversedirmap: HashMap<_, _> =
    //     dirmap.iter().map(|(&a, &b)| (b, a)).collect();
    //
    let transmap: HashMap<_, HashMap<_, _>> = vec![
        (
            dirmap[&'^'],
            vec![
                (Track::Vert, Track::Vert),
                (Track::Hori, Track::Hori),
                (Track::Rcurve, Track::Rcurve),
                (Track::Lcurve, Track::Lcurve),
                (Track::Cross, Track::Cross),
                (Track::Empty, Track::Empty),
            ],
        ),
        (
            dirmap[&'v'],
            vec![
                (Track::Vert, Track::Vert),
                (Track::Hori, Track::Hori),
                (Track::Rcurve, Track::Rcurve),
                (Track::Lcurve, Track::Lcurve),
                (Track::Cross, Track::Cross),
                (Track::Empty, Track::Empty),
            ],
        ),
        (
            dirmap[&'>'],
            vec![
                (Track::Vert, Track::Hori),
                (Track::Hori, Track::Vert),
                (Track::Rcurve, Track::Lcurve),
                (Track::Lcurve, Track::Rcurve),
                (Track::Cross, Track::Cross),
                (Track::Empty, Track::Empty),
            ],
        ),
        (
            dirmap[&'<'],
            vec![
                (Track::Vert, Track::Hori),
                (Track::Hori, Track::Vert),
                (Track::Rcurve, Track::Lcurve),
                (Track::Lcurve, Track::Rcurve),
                (Track::Cross, Track::Cross),
                (Track::Empty, Track::Empty),
            ],
        ),
    ]
    .into_iter()
    .map(|(a, b)| (a, b.into_iter().collect()))
    .collect();
    //
    let (map, mut carts) =
        loadmap("inputfiles/day13/input.txt", &trackmap, &dirmap);
    //
    let w = map[0].len();
    let mut order: Vec<_> = (0..carts.len()).collect();
    //
    let mut ans;
    'outer: loop {
        order.sort_by_cached_key(|&i| carts[i].1.re + carts[i].1.im * w as i32);
        for &i in order.iter() {
            ans = movecart(&map, &transmap, &mut carts, i);
            if ans.is_some() {
                break 'outer;
            }
        }
    }
    println!("{}", ans.unwrap());
}

fn part2() {
    let trackmap: HashMap<_, _> = vec![
        ('|', Track::Vert),
        ('-', Track::Hori),
        ('/', Track::Rcurve),
        ('\\', Track::Lcurve),
        ('+', Track::Cross),
        (' ', Track::Empty),
    ]
    .into_iter()
    .collect();
    //
    let _reversemap: HashMap<_, _> =
        trackmap.iter().map(|(&a, &b)| (b, a)).collect();
    //
    let dirmap: HashMap<_, _> = vec![
        ('>', Complex::new(1, 0)),
        ('v', Complex::new(0, 1)),
        ('<', Complex::new(-1, 0)),
        ('^', Complex::new(0, -1)),
    ]
    .into_iter()
    .collect();
    //
    let _reversedirmap: HashMap<_, _> =
        dirmap.iter().map(|(&a, &b)| (b, a)).collect();
    //
    let transmap: HashMap<_, HashMap<_, _>> = vec![
        (
            dirmap[&'^'],
            vec![
                (Track::Vert, Track::Vert),
                (Track::Hori, Track::Hori),
                (Track::Rcurve, Track::Rcurve),
                (Track::Lcurve, Track::Lcurve),
                (Track::Cross, Track::Cross),
                (Track::Empty, Track::Empty),
            ],
        ),
        (
            dirmap[&'v'],
            vec![
                (Track::Vert, Track::Vert),
                (Track::Hori, Track::Hori),
                (Track::Rcurve, Track::Rcurve),
                (Track::Lcurve, Track::Lcurve),
                (Track::Cross, Track::Cross),
                (Track::Empty, Track::Empty),
            ],
        ),
        (
            dirmap[&'>'],
            vec![
                (Track::Vert, Track::Hori),
                (Track::Hori, Track::Vert),
                (Track::Rcurve, Track::Lcurve),
                (Track::Lcurve, Track::Rcurve),
                (Track::Cross, Track::Cross),
                (Track::Empty, Track::Empty),
            ],
        ),
        (
            dirmap[&'<'],
            vec![
                (Track::Vert, Track::Hori),
                (Track::Hori, Track::Vert),
                (Track::Rcurve, Track::Lcurve),
                (Track::Lcurve, Track::Rcurve),
                (Track::Cross, Track::Cross),
                (Track::Empty, Track::Empty),
            ],
        ),
    ]
    .into_iter()
    .map(|(a, b)| (a, b.into_iter().collect()))
    .collect();
    //
    let (map, mut carts) =
        loadmap("inputfiles/day13/input.txt", &trackmap, &dirmap);
    //
    let w = map[0].len();
    let mut order: Vec<_> = (0..carts.len()).collect();
    // _showmap(&map, &carts, &_reversemap, &_reversedirmap);
    loop {
        // std::io::stdin()
        //     .read_line(&mut String::new())
        //     .expect("Failed to read line");
        //
        let mut needscleenup = HashSet::new();
        order.sort_by_cached_key(|&i| carts[i].1.re + carts[i].1.im * w as i32);
        for &i in order.iter() {
            if let Some(crash) = movecart(&map, &transmap, &mut carts, i) {
                needscleenup.insert(crash);
            }
        }
        if needscleenup.len() > 0 {
            carts = carts
                .into_iter()
                .filter(|(_, pos, _)| !needscleenup.contains(pos))
                .collect();
            order = (0..carts.len()).collect();
            if carts.len() <= 1 {
                break;
            }
        }
        // _showmap(&map, &carts, &_reversemap, &_reversedirmap);
    }
    println!("{}", carts[0].1);
}
