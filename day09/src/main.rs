use common::load_vec;
use std::collections::BTreeSet;
use std::iter::FromIterator;

const PREAMBLE_SIZE: usize = 25;

fn main() {
    let input: Vec<usize> = load_vec("input/day9.txt");
    let bad_elem = part1(&input);
    part2(&input, bad_elem)
}

fn part1(input: &[usize]) -> usize {
    let mut set = BTreeSet::from_iter(input[..PREAMBLE_SIZE].iter().map(|&i| i));
    for (idx, elem) in input[PREAMBLE_SIZE..].iter().enumerate() {
        if let None = find_pair_summing_to(*elem, &set) {
            println!("Part 1: {}", elem);
            return *elem;
        }
        set.remove(&input[idx]);
        set.insert(*elem);
    }
    panic!("Part 1: No solution found");
}

fn part2(input: &[usize], bad_elem: usize) {
    // elements can only add to bad_elem if they are less than bad_elem
    // so split the input into subranges containing only elements < bad_elem
    let max_ranges = input.split(|i| i >= &bad_elem).filter(|i| i.len() > 0);
    for range in max_ranges {
        if let Some((start, end)) = find_run_adding_to(range, bad_elem) {
            let run = &range[start..=end];
            println!(
                "Part 2: {}",
                run.iter().max().unwrap() + run.iter().min().unwrap()
            );
            return;
        }
    }

    panic!("Part 2: No solution found");
}

fn find_run_adding_to(src: &[usize], target: usize) -> Option<(usize, usize)> {
    if src.len() < 1 {
        return None;
    }
    let mut sum = src[0];
    let mut start_idx = 0;
    let mut end_idx = 0;
    while end_idx < src.len() - 1 {
        if sum == target {
            // we found a solution
            return Some((start_idx, end_idx));
        } else if sum > target {
            // we have exceeded the target, so shed some weight from the start
            sum -= src[start_idx];
            start_idx += 1;
        } else {
            // we have not yet hit the target, so add some extra weight on the end
            end_idx += 1;
            sum += src[end_idx];
        }
    }

    None
}

fn find_pair_summing_to(target: usize, src: &BTreeSet<usize>) -> Option<(usize, usize)> {
    for elem in src {
        if target >= *elem && src.contains(&(target - elem)) {
            return Some((*elem, target - elem));
        }
    }

    None
}
