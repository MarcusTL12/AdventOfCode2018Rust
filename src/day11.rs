pub const PARTS: [fn(); 2] = [part1, part2];

const SERIAL: usize = 8979;

fn powercellval(x: usize, y: usize) -> i64 {
    let rackid = x + 10;
    let mut powlvl = rackid * y;
    powlvl += SERIAL;
    powlvl *= rackid;
    powlvl = (powlvl / 100) % 10;
    powlvl as i64 - 5
}

fn part1() {
    let ans = (0..300 - 3)
        .map(|y| (0..300 - 3).map(move |x| (x, y)))
        .flatten()
        .map(|(x, y)| {
            (
                x,
                y,
                (y..y + 3)
                    .map(move |y| (x..x + 3).map(move |x| (x, y)))
                    .flatten()
                    .map(|(x, y)| powercellval(x, y))
                    .sum::<i64>(),
            )
        })
        .max_by(|(_, _, a), (_, _, b)| a.cmp(b))
        .unwrap();
    //
    println!("{:?}", ans);
}

fn part2() {
    let mut data: Vec<Vec<Vec<_>>> = Vec::with_capacity(300);
    //
    data.push(
        (0..300)
            .map(|y| (0..300).map(move |x| powercellval(x, y)).collect())
            .collect(),
    );
    //
    for i in 1..300 {
        // println!("{}", i);
        let lastlayer = data.last().unwrap();
        let nlayer = (0..300 - i)
            .map(|y| {
                (0..300 - i)
                    .map(move |x| (x, y))
                    .map(|(x, y)| {
                        let prevsum = lastlayer[y][x];
                        let bottomsum: i64 =
                            data[0][y + i][x..x + i].iter().sum();
                        let rightsum: i64 = data[0][y..y + i]
                            .iter()
                            .map(|row| row[x + i])
                            .sum();
                        let corner = data[0][y + i][x + i];
                        prevsum + bottomsum + rightsum + corner
                    })
                    .collect()
            })
            .collect();
        data.push(nlayer);
    }
    //
    let ans = data
        .iter()
        .enumerate()
        .map(|(i, layer)| {
            layer
                .iter()
                .enumerate()
                .map(move |(j, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(k, &val)| (k, j, i + 1, val))
                })
                .flatten()
        })
        .flatten()
        .max_by(|(_, _, _, a), (_, _, _, b)| a.cmp(b))
        .unwrap();
    //
    println!("{:?}", ans);
}
