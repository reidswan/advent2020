use common::{load_vec, take_first_number};
use std::str::FromStr;

fn main() {
    let passwords: Vec<PasswordLine> = load_vec("input/day2.txt");
    part1(&passwords);
    part2(&passwords);
}

fn part1(passwords: &[PasswordLine]) {
    println!(
        "Part 1: {}",
        passwords.iter().filter(|p| p.check_part1()).count()
    )
}

fn part2(passwords: &[PasswordLine]) {
    println!(
        "Part 2: {}",
        passwords.iter().filter(|p| p.check_part2()).count()
    )
}

#[derive(Debug)]
struct PasswordLine {
    password: String,
    requirement: Requirement,
}

impl PasswordLine {
    fn check_part1(&self) -> bool {
        let total = self
            .password
            .chars()
            .filter(|&c| c == self.requirement.required)
            .count();
        total >= self.requirement.min && total <= self.requirement.max
    }

    fn check_part2(&self) -> bool {
        let expected = Some(self.requirement.required);
        (self.password.chars().nth(self.requirement.min - 1) == expected)
            ^ (self.password.chars().nth(self.requirement.max - 1) == expected)
    }
}

impl FromStr for PasswordLine {
    type Err = String;

    //<min>-<max> <required>
    fn from_str(s: &str) -> Result<PasswordLine, Self::Err> {
        let (requirements, rest) =
            s.split_at(s.find(':').ok_or(format!("No ':' found in source {}", s))?);
        let requirement = Requirement::from_str(requirements)?;
        let password = rest.trim_start_matches(": ").into();

        Ok(PasswordLine {
            password,
            requirement,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Requirement {
    required: char,
    min: usize,
    max: usize,
}

impl FromStr for Requirement {
    type Err = String;

    //<min>-<max> <required>
    fn from_str(s: &str) -> Result<Requirement, Self::Err> {
        let mut src = s.chars().peekable();
        let min = take_first_number(&mut src)?;

        if src.next() != Some('-') {
            return Err("Expected but did not find '-' after <min>".into());
        }
        let max = take_first_number(&mut src)?;
        if src.next() != Some(' ') {
            return Err("Expected but did not find ' ' after <max>".into());
        }
        let required = src.next().ok_or("Expected a final char after <max>")?;

        Ok(Requirement { min, max, required })
    }
}
