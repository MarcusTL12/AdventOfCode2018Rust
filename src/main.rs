use std::env;
use std::time;

mod day1;
mod day2;
mod day3;

fn main() {
    let funcs = [day1::PARTS, day2::PARTS, day3::PARTS];
    let mut args = env::args();
    args.next();
    if let Some(x) = args.next() {
        if let Ok(x) = x.parse::<usize>() {
            if let Some(y) = args.next() {
                if let Ok(y) = y.parse::<usize>() {
                    if let Some(x) = funcs.get(x - 1) {
                        if let Some(x) = x.get(y - 1) {
                            let timer = time::Instant::now();
                            x();
                            println!(
                                "Took {} seconds",
                                timer.elapsed().as_secs_f32()
                            );
                        } else {
                            println!("Not implemented");
                        }
                    } else {
                        println!("Not implemented");
                    }
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
