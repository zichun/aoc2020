use std::io::{self, Read};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    day07();
}

fn day07() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day07::day07_1(&input));
    println!("{}", day07::day07_2(&input));
}

fn day06() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day06::day06_1(&input));
    println!("{}", day06::day06_2(&input));
}

fn day05() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day05::day05_1(&input));
    println!("{}", day05::day05_2(&input));
}

fn day04() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day04::day04_1(&input));
    println!("{}", day04::day04_2(&input));
}

fn day03() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day03::day03_1(&input));
    println!("{}", day03::day03_2(&input));
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
