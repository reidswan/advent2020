use std::collections::VecDeque;
use std::str::FromStr;

fn get_input() -> VecDeque<usize> {
    // VecDeque::from(vec![1, 3, 5, 4, 6, 8, 7, 2, 9])
    "135468729"
        .chars()
        .map(|c| usize::from_str(&format!("{}", c)).unwrap())
        .collect()
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut input = get_input();
    input.rotate_left(1);
    play_until(100, &mut input);
    let result = input.iter().fold(String::new(), |mut acc, i| {
        acc.push_str(&format!("{}", i)[..]);
        acc
    });

    println!("Part 1: {}", result);
}

fn play_until(end: usize, cups: &mut VecDeque<usize>) {
    for _ in 0..end {
        play(cups);
    }

    let loc = cups.iter().position(|&i| i == 1).unwrap();
    cups.rotate_left(loc);
    cups.pop_front();
}

fn part2() {
    let mut next_cup = transform_input(get_input());

    let mut current_cup = next_cup[0];
    for _ in 0..10_000_000 {
        let removed_cups = {
            let mut removed_cups = vec![];
            let mut prev_cup = current_cup;
            // follow the next cup 3 times to find the removed cups
            for _ in 0..3 {
                let removed_cup = next_cup[prev_cup];
                removed_cups.push(removed_cup);
                prev_cup = removed_cup;
            }
            // update the next cup for current cup to point to the cup after the removed cups
            next_cup[current_cup] = next_cup[prev_cup];
            removed_cups
        };

        // decrement preceding_cup until the target is not a removed cup
        let mut preceding_cup = if current_cup == 1 {
            1_000_000
        } else {
            current_cup - 1
        };
        while removed_cups.contains(&preceding_cup) {
            preceding_cup = if preceding_cup == 1 {
                1_000_000
            } else {
                preceding_cup - 1
            };
        }

        // insert the removed cups by changing
        // preceding_cup -> old_next :=>: preceding -> [removed_cups] -> old_next
        let old_next = next_cup[preceding_cup];
        let mut assign = preceding_cup;
        for removed_cup in removed_cups {
            next_cup[assign] = removed_cup;
            assign = removed_cup;
        }
        next_cup[assign] = old_next;
        current_cup = next_cup[current_cup]
    }

    let result = next_cup[1] * next_cup[next_cup[1]];
    println!("Part 2: {}", result);
}

fn play(cups: &mut VecDeque<usize>) {
    let src_cup = *cups.back().unwrap();
    let removed_cups = {
        let mut result = vec![];
        for _ in 0..3 {
            result.push(cups.pop_front().unwrap());
        }
        result.reverse();
        result
    };
    let mut dest_cup = if src_cup == 1 { 9 } else { src_cup - 1 };
    while removed_cups.contains(&dest_cup) {
        dest_cup = if dest_cup == 1 { 9 } else { dest_cup - 1 };
    }
    let loc = cups.iter().position(|&i| i == dest_cup).unwrap();
    cups.rotate_left(loc + 1);
    for elem in removed_cups {
        cups.push_front(elem);
    }
    cups.rotate_right(loc);
}

fn transform_input(input: VecDeque<usize>) -> [usize; 1_000_001] {
    let mut result = [0; 1_000_001];
    result[0] = input[0]; // the current cup
    let max = input.iter().max().unwrap();
    for idx in 0..input.len() - 1 {
        result[input[idx]] = input[idx + 1]
    }
    result[*input.back().unwrap()] = max + 1;

    for i in max + 1..1_000_000 {
        result[i] = i + 1;
    }

    result[1_000_000] = *input.front().unwrap();

    result
}
