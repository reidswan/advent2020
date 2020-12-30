use common::{chinese_remainder, load_single_object};
use std::str::FromStr;

fn main() {
    let input: Input = load_single_object("input/day13.txt");

    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    println!(
        "Part 1: {}",
        input
            .bus_ids
            .iter()
            .filter_map(|&i| i.map(|id| (id, id - (input.arrival_ts % id))))
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(i, j)| i * j)
            .unwrap()
    )
}

fn part2(input: &Input) {
    //(i, j) => (x + i) % j = 0
    // === x + i == 0 (mod j)
    // === x == (-i) (mod j)
    // === x == (j-i) (mod j)
    let busses: Vec<_> = input
        .bus_ids
        .iter()
        .enumerate()
        .filter_map(|(idx, bus_id)| bus_id.map(|id| (id as isize - idx as isize, id as isize)))
        .collect();
    let rem = chinese_remainder(&busses);
    println!("Part 2: {}", rem);
}

#[derive(Debug, Clone)]
struct Input {
    arrival_ts: usize,
    bus_ids: Vec<Option<usize>>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (timestamp_s, bus_ids_s) = {
            let mut lines = s.lines();

            (lines.next().unwrap(), lines.next().unwrap())
        };

        let arrival_ts = usize::from_str(timestamp_s).unwrap();
        let bus_ids = bus_ids_s
            .split(",")
            .map(|bus_id| {
                if bus_id == "x" {
                    None
                } else {
                    Some(usize::from_str(bus_id).unwrap()) // intentionally using Some(...unwrap()) instead of .ok() because we want to panic on failure
                }
            })
            .collect();

        Ok(Input {
            arrival_ts,
            bus_ids,
        })
    }
}
