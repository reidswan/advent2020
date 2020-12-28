use common::load_vec;
use std::collections::HashSet;

fn main() {
    let input: Vec<usize> = load_vec("input/day5.txt")
        .iter()
        .map(|s: &String| {
            s.replace('F', "0")
                .replace('B', "1")
                .replace('L', "0")
                .replace('R', "1")
        })
        .map(|s| usize::from_str_radix(&s, 2).unwrap())
        .collect();
    part1(&input);
    part2(&input);
}

fn part1(src: &[usize]) {
    println!("Part 1: {}", src.iter().max().unwrap())
}

fn part2(src: &[usize]) {
    let seats: HashSet<_> = src.into_iter().map(|&i| i).collect();

    let start = *seats.iter().min().unwrap();
    let end = *seats.iter().max().unwrap();
    for i in start + 1..end {
        if !seats.contains(&i) {
            println!("Part 2: {}", i);
            break;
        }
    }
}
