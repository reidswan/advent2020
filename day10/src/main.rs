use common::load_vec;
use std::collections::{BTreeSet, HashMap};
use std::iter::once;

fn main() {
    let mut input: Vec<isize> = load_vec("input/day10.txt");
    input.sort();
    let max_joltage = input.iter().max().unwrap();
    let device_joltage = max_joltage + 3;
    part1(&input, device_joltage);

    part2(&input, device_joltage);
}

fn part1(input: &[isize], device_joltage: isize) {
    let mut diff = JoltageDiffs::default();
    for (low, high) in once(&0)
        .chain(input.iter())
        .zip(input.iter().chain(once(&device_joltage)))
    {
        if (high - low).abs() > 3 {
            panic!("Invalid chain! {} and {} are too far apart", high, low);
        } else if high - low == 3 {
            diff.threes += 1
        } else if high - low == 1 {
            diff.ones += 1
        }
    }

    println!("Part 1: {}", diff.ones * diff.threes);
}

fn part2(input: &[isize], device_joltage: isize) {
    let possible_start_joltages = input
        .iter()
        .enumerate()
        .take_while(|(_, val)| **val <= 3)
        .map(|(i, _)| i);

    let possible_next_joltages: Vec<_> = (0..input.len())
        .map(|idx| get_possible_next_joltage_indices(idx, &input))
        .collect();

    let mut counts: Vec<HashMap<usize, usize>> = (0..input.len()).map(|_| HashMap::new()).collect();

    let total: usize = possible_start_joltages
        .map(|joltage_idx| {
            total_combos(
                &input,
                joltage_idx,
                device_joltage,
                &mut counts,
                &possible_next_joltages,
            )
        })
        .sum();

    println!("Part 2: {}", total)
}

fn total_combos(
    input: &[isize],
    idx: usize,
    device_joltage: isize,
    counts: &mut Vec<HashMap<usize, usize>>,
    possible_next_joltages: &Vec<BTreeSet<usize>>,
) -> usize {
    if idx == input.len() - 1 {
        return 1;
    }

    let mut total = if input[idx] + 3 >= device_joltage {
        1 // we can end the chain here
    } else {
        0 // we cannot end the chain here
    };

    for possible_next_joltage in possible_next_joltages[idx].iter() {
        total += match counts[idx].get(possible_next_joltage) {
            Some(count) => *count,
            _ => {
                let count = total_combos(
                    input,
                    *possible_next_joltage,
                    device_joltage,
                    counts,
                    possible_next_joltages,
                );

                counts[idx].insert(*possible_next_joltage, count);
                count
            }
        }
    }

    total
}

fn get_possible_next_joltage_indices(from_idx: usize, src: &[isize]) -> BTreeSet<usize> {
    let from = src[from_idx];
    src.iter()
        .enumerate()
        .skip(from_idx)
        .take_while(|(_, val)| **val <= from + 3)
        .filter(|(idx, _)| *idx != from_idx)
        .map(|(idx, _)| idx)
        .collect()
}

#[derive(Debug, Default)]
struct JoltageDiffs {
    ones: usize,
    threes: usize,
}
