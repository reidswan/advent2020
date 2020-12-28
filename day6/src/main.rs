use common::load_groups;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let groups: Vec<String> = load_groups("input/day6.txt");
    part1(&groups);
    part2(&groups);
}

fn part1(groups: &[String]) {
    let total: usize = groups
        .iter()
        .map(|group| {
            HashSet::<char>::from_iter(group.chars().filter(|&c| 'a' <= c && c <= 'z')).len()
        })
        .sum();
    println!("Day 1: {}", total)
}

fn part2(groups: &[String]) {
    let total: usize = groups
        .iter()
        .map(|group| {
            group
                .split('\n')
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        Some(HashSet::<char>::from_iter(trimmed.chars()))
                    } else {
                        None
                    }
                })
                .fold(None, |acc, set| {
                    acc.map(|existing: HashSet<char>| {
                        existing.intersection(&set).map(|&c| c).collect()
                    })
                    .or(Some(set))
                })
                .unwrap()
                .len()
        })
        .sum();

    println!("Part 2: {}", total)
}
