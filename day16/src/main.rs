use common::load_single_object;
use lazy_static::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    let input: Input = load_single_object("input/day16.txt");
    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    println!(
        "{}",
        input
            .nearby_tickets
            .iter()
            .flat_map(|t| t.fields_not_satisfying_a_validation(&input.validations))
            .sum::<usize>()
    );
}

fn part2(input: &Input) {
    let valid_tickets: Vec<_> = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .fields_not_satisfying_a_validation(&input.validations)
                .len()
                == 0
        })
        .collect();
    let field_sets = make_field_sets(&valid_tickets);
    let mut possible_fields_for_sets: Vec<_> = field_sets
        .iter()
        .map(|set| determine_possible_fields_for_set(set, &input.validations))
        .collect();
    let mut has_changed = true;
    let mut resolved_names: HashMap<String, usize> = get_resolved_names(&possible_fields_for_sets);
    while has_changed && any_has_multiple_possibilities(&possible_fields_for_sets) {
        has_changed = false;
        for (idx, poss) in possible_fields_for_sets.iter_mut().enumerate() {
            for (name, matched_idx) in resolved_names.iter() {
                if poss.contains_key(&name[..]) && *matched_idx != idx {
                    has_changed = true;
                    poss.remove(&name[..]);
                }
            }
        }
        resolved_names = get_resolved_names(&possible_fields_for_sets);
    }
    if any_has_multiple_possibilities(&possible_fields_for_sets) {
        panic!("Failed to narrow possibilities down sufficiently");
    }

    let departure_prod: usize = resolved_names
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, idx)| input.your_ticket.fields[*idx])
        .product();
    println!("Part 2: {}", departure_prod);
}

fn get_resolved_names(
    possible_fields_for_sets: &Vec<HashMap<&str, &ValidationField>>,
) -> HashMap<String, usize> {
    possible_fields_for_sets
        .iter()
        .enumerate()
        .filter(|(_, poss)| poss.len() == 1)
        .map(|(idx, poss)| ((**poss.iter().next().unwrap().0).into(), idx))
        .collect()
}

fn any_has_multiple_possibilities(possibilities: &Vec<HashMap<&str, &ValidationField>>) -> bool {
    possibilities.iter().map(|poss| poss.len()).any(|i| i > 1)
}

// collect all the values from the same field on a ticket into a set
fn make_field_sets(tickets: &[&Ticket]) -> Vec<HashSet<usize>> {
    let mut sets: Vec<_> = (0..tickets[0].fields.len())
        .map(|_| HashSet::new())
        .collect();
    for ticket in tickets {
        for (idx, field) in ticket.fields.iter().enumerate() {
            sets[idx].insert(*field);
        }
    }

    sets
}

fn determine_possible_fields_for_set<'a>(
    field_set: &HashSet<usize>,
    validations: &'a [ValidationField],
) -> HashMap<&'a str, &'a ValidationField> {
    validations
        .iter()
        .filter(|&validation| field_set.iter().all(|field| validation.satisfied(*field)))
        .map(|validation| (&validation.name[..], validation))
        .collect()
}

trait Range {
    fn in_range(&self, value: usize) -> bool;
}

#[derive(Copy, Clone, Debug)]
struct InclusiveRange {
    min: usize,
    max: usize,
}

impl Range for InclusiveRange {
    fn in_range(&self, value: usize) -> bool {
        self.min <= value && value <= self.max
    }
}

impl FromStr for InclusiveRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RANGE_RE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
        }
        if !RANGE_RE.is_match(s) {
            return Err("Expected format <min>-<max>".into());
        }

        let caps = RANGE_RE.captures(s).unwrap();
        let min = usize::from_str(caps.get(1).unwrap().as_str()).unwrap();
        let max = usize::from_str(caps.get(2).unwrap().as_str()).unwrap();
        if min > max {
            return Err(format!(
                "Malformed range: min={} greater than max={}",
                min, max
            ));
        }

        Ok(InclusiveRange { min, max })
    }
}

#[derive(Clone, Debug)]
struct MultiRange {
    ranges: Vec<InclusiveRange>,
}

impl Range for MultiRange {
    fn in_range(&self, value: usize) -> bool {
        self.ranges.iter().any(|range| range.in_range(value))
    }
}

impl FromStr for MultiRange {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        lazy_static! {
            static ref MULTIRANGE_RE: Regex =
                Regex::new(r"(\d+-\d+)((\s+or\s+\d+-\d+)+)?").unwrap();
        }
        if !MULTIRANGE_RE.is_match(s) {
            return Err("Expected format <mina>-<maxa> (or <minb>-<maxb> or ...)".into());
        }

        let caps = MULTIRANGE_RE.captures(s).unwrap();
        let mut ranges = vec![InclusiveRange::from_str(caps.get(1).unwrap().as_str())?];
        if let Some(cap) = caps.get(2) {
            let remaining = cap
                .as_str()
                .trim_start()
                .trim_start_matches("or")
                .trim_start();
            ranges.append(&mut MultiRange::from_str(remaining)?.ranges)
        }

        Ok(MultiRange { ranges })
    }
}

#[derive(Debug, Clone)]
struct ValidationField {
    name: String,
    range: MultiRange,
}

impl ValidationField {
    fn satisfied(&self, value: usize) -> bool {
        self.range.in_range(value)
    }
}

impl FromStr for ValidationField {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref VALIDATIONFIELD_RE: Regex =
                Regex::new(r"([a-zA-Z\s]+):\s+(\d+-\d+\s+(or\s+\d+-\d+))*").unwrap();
        }
        if !VALIDATIONFIELD_RE.is_match(s) {
            return Err(format!("Parsing '{}' into ValidationField failed", s));
        }
        let caps = VALIDATIONFIELD_RE.captures(s).unwrap();
        let name: String = caps.get(1).unwrap().as_str().into();
        let range = MultiRange::from_str(caps.get(2).unwrap().as_str())?;

        Ok(ValidationField { name, range })
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    fn fields_not_satisfying_a_validation(&self, validations: &[ValidationField]) -> Vec<usize> {
        self.fields
            .iter()
            .map(|f| *f)
            .filter(|field| {
                validations
                    .iter()
                    .all(|validation| !validation.satisfied(*field))
            })
            .collect()
    }
}

impl FromStr for Ticket {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.split(",")
            .any(|part| !part.chars().all(|ch| ch.is_ascii_digit()))
        {
            return Err("Expected a CSV of numbers when parsing ticket".into());
        }
        Ok(Ticket {
            fields: s
                .split(",")
                .map(|part| usize::from_str(part).unwrap())
                .collect(),
        })
    }
}

#[derive(Debug, Clone)]
struct Input {
    validations: Vec<ValidationField>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (validations, rest) = {
            let mut parts = s.split("your ticket:");

            (
                parts.next().unwrap().trim(),
                parts
                    .next()
                    .ok_or(String::from("Expected 'your ticket:' in input"))?,
            )
        };

        let validations = validations
            .lines()
            .map(|line| ValidationField::from_str(line))
            .collect::<Result<_, _>>()?;
        let (your_ticket_s, nearby_tickets_s) = {
            let mut parts = rest.split("nearby tickets:");

            (
                parts.next().unwrap().trim(),
                parts
                    .next()
                    .map(|s| s.trim())
                    .ok_or(String::from("Expected 'nearby tickets:' in input"))?,
            )
        };
        let your_ticket = Ticket::from_str(your_ticket_s)?;
        let nearby_tickets = nearby_tickets_s
            .lines()
            .map(|line| Ticket::from_str(line))
            .collect::<Result<_, _>>()?;

        Ok(Input {
            validations,
            your_ticket,
            nearby_tickets,
        })
    }
}
