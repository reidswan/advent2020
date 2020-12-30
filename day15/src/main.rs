use common::load_raw_text;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input: Vec<usize> = get_input();
    let mut game = MemoryGame::new(&input).unwrap();
    game.run_until(2020);
    println!("Part 1: {}", game.last_number);
    game.run_until(30000000);
    println!("Part 2: {}", game.last_number);
}

struct MemoryGame {
    turn: usize,
    memory: HashMap<usize, (usize, Option<usize>)>,
    last_number: usize,
}

impl MemoryGame {
    fn new(src: &[usize]) -> Result<MemoryGame, String> {
        if src.is_empty() {
            Err("Need starting numbers".into())
        } else {
            let memory = src
                .iter()
                .enumerate()
                .map(|(i, j)| (*j, (i + 1, None)))
                .collect();
            Ok(MemoryGame {
                turn: src.len(),
                memory,
                last_number: *src.last().unwrap(),
            })
        }
    }

    fn step(&mut self) {
        self.turn += 1;
        let age = self
            .memory
            .get(&self.last_number)
            .and_then(|(last_turn, turn_before)| turn_before.map(|tb| last_turn - tb))
            .unwrap_or(0);
        let prev = match self.memory.get(&age) {
            Some(&(prev, _)) => Some(prev),
            _ => None,
        };
        self.memory.insert(age, (self.turn, prev));
        self.last_number = age;
    }

    fn run_until(&mut self, n_turns: usize) {
        if self.turn > n_turns {
            panic!("Already run past {}", n_turns);
        }
        while self.turn < n_turns {
            self.step()
        }
    }
}

fn get_input() -> Vec<usize> {
    load_raw_text("input/day15.txt")
        .trim()
        .split(",")
        .map(|s| usize::from_str(s).unwrap())
        .collect()
}
