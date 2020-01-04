use std::env;
use std::time;

mod day1;
mod day2;

fn main() {
    let funcs = [day1::PARTS, day2::PARTS];
    let mut args = env::args();
    args.next();
    if let Some(x) = args.next() {
        if let Ok(x) = x.parse::<usize>() {
            if let Some(y) = args.next() {
                if let Ok(y) = y.parse::<usize>() {
                    let timer = time::Instant::now();
                    funcs[x - 1][y - 1]();
                    println!("Took {} seconds", timer.elapsed().as_secs_f32());
                } else {
                    println!("Must enter numbers!");
                }
            } else {
                println!("Pass day and part as commandline parameters");
            }
        } else {
            println!("Must enter numbers!");
        }
    } else {
        println!("Pass day and part as commandline parameters");
    }
}
