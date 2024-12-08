use itertools::Itertools;
use std::collections::HashSet;

fn part1() {
    let input = std::fs::read_to_string("inputs/01.input").unwrap();

    let numbers = input
        .lines()
        .map(str::split_whitespace)
        .flatten()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut left = numbers.iter().copied().step_by(2).collect::<Vec<_>>();
    let mut right = numbers
        .iter()
        .copied()
        .skip(1)
        .step_by(2)
        .collect::<Vec<_>>();

    left.sort();
    right.sort();

    let distance_sum = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("Part 1: {}", distance_sum);
}

fn part2() {
    let input = std::fs::read_to_string("inputs/01.input").unwrap();

    let numbers = input
        .lines()
        .map(str::split_whitespace)
        .flatten()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let left = numbers.iter().copied().step_by(2).collect::<Vec<_>>();
    let mut right = numbers
        .iter()
        .skip(1)
        .step_by(2)
        .copied()
        .collect::<Vec<_>>();

    right.sort();

    let product = right
        .iter()
        .chunk_by(|x| *x)
        .into_iter()
        .map(|(key, chunk)| (key, chunk.collect::<Vec<_>>()))
        // .inspect(|x| println!("{:?}", x))
        .map(|(key, chunk)| {
            left.iter().map(|val| val == key).filter(|x| *x).count() as i32
                * chunk.len() as i32
                * key
        })
        // .inspect(|x| println!("{}", x))
        .sum::<i32>();

    println!("Part 2: {}", product);
}

fn main() {
    part1();
    part2();
}
