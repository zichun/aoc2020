use std::io::{self, Read};

mod day01;
mod day02;

fn main() {
    day02();
}

fn day02() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day02::day02_1(&input));
    println!("{}", day02::day02_2(&input));
}

fn day01() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let input: Vec<u32> = input
        .split("\n")
        .filter_map(|x|
                    x.trim().parse().ok())
        .collect();

    println!("{}", day01::day01_1(&input));
    println!("{}", day01::day01_2(&input));
    println!("{}", day01::day01_2_linear(&input));
}
