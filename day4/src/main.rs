#[macro_use]
extern crate lazy_static;

use common::load_groups;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

lazy_static! {
    static ref REQUIRED_FIELDS: HashMap<String, Regex> =
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .map(|s| (
                String::from(s),
                Regex::new(&format!("{}:([^\\s]+)", s)).unwrap()
            ))
            .collect();
    static ref HAIR_RE: Regex = Regex::new("#[a-f0-9]{6}").unwrap();
    static ref VALID_EYE_COLORS: HashSet<&'static str> =
        HashSet::from_iter(vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter());
}

fn main() {
    // let raw_input = load_raw_text("input/day4.txt");
    let passports: Vec<_> = load_groups("input/day4.txt");

    part1(&passports);
    part2(&passports);
}

fn part1(passports: &[String]) {
    let part1 = passports
        .iter()
        .filter(|passport| check_passport(passport))
        .count();

    println!("Part 1: {}", part1);
}

fn part2(passports: &[String]) {
    let part2 = passports
        .iter()
        .map(|p| Passport::from_string(p))
        .filter(|p| p.validate())
        .count();

    println!("Part 2: {}", part2);
}

fn check_passport(passport: &str) -> bool {
    for re in REQUIRED_FIELDS.values() {
        if !re.is_match(passport) {
            return false;
        }
    }
    true
}

#[derive(Default, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn from_string(src: &str) -> Self {
        let mut passport = Passport::default();
        for (key, value) in REQUIRED_FIELDS.iter() {
            if let Some(cap) = value.captures_iter(src).next() {
                passport.set(key, cap[1].into());
            }
        }

        passport
    }

    fn set(&mut self, key: &str, value: String) {
        match key {
            "byr" => self.byr = Some(value),
            "iyr" => self.iyr = Some(value),
            "eyr" => self.eyr = Some(value),
            "hgt" => self.hgt = Some(value),
            "hcl" => self.hcl = Some(value),
            "ecl" => self.ecl = Some(value),
            "pid" => self.pid = Some(value),
            _ => unreachable!(),
        }
    }

    fn validate(&self) -> bool {
        self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_color()
            && self.valid_eye_color()
            && self.valid_passport_id()
    }

    fn valid_birth_year(&self) -> bool {
        is_number_between(self.byr.as_ref(), 1920, 2002)
    }

    fn valid_issue_year(&self) -> bool {
        is_number_between(self.iyr.as_ref(), 2010, 2020)
    }

    fn valid_expiration_year(&self) -> bool {
        is_number_between(self.eyr.as_ref(), 2020, 2030)
    }

    fn valid_height(&self) -> bool {
        if let Some(hgt) = self.hgt.as_ref() {
            if hgt.ends_with("cm") {
                return is_number_between(
                    Some(hgt.trim_end_matches("cm").into()).as_ref(),
                    150,
                    193,
                );
            } else if hgt.ends_with("in") {
                return is_number_between(Some(hgt.trim_end_matches("in").into()).as_ref(), 59, 76);
            }
        }

        false
    }

    fn valid_hair_color(&self) -> bool {
        if let Some(hcl) = self.hcl.as_ref() {
            HAIR_RE.is_match(hcl)
        } else {
            false
        }
    }

    fn valid_eye_color(&self) -> bool {
        if let Some(ecl) = self.ecl.as_ref() {
            VALID_EYE_COLORS.contains(&ecl[..])
        } else {
            false
        }
    }

    fn valid_passport_id(&self) -> bool {
        if let Some(pid) = self.pid.as_ref() {
            pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
        } else {
            false
        }
    }
}

fn is_number_between(src: Option<&String>, min: usize, max: usize) -> bool {
    src.and_then(|num| usize::from_str(&num).ok())
        .map(|num| min <= num && num <= max)
        .unwrap_or(false)
}
