use common::load_vec;
use lazy_static::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Colour = String;

fn main() {
    let input: Vec<Rule> = load_vec("input/day7.txt");

    part1(&input);
    part2(&input);
}

fn part1(rules: &[Rule]) {
    let containing_map = create_containing_graph(rules);
    let mut seen: HashSet<&str> = HashSet::new();
    let shiny_gold = String::from("shiny gold");
    let mut unprocessed_colours = vec![&shiny_gold];
    while let Some(colour) = unprocessed_colours.pop() {
        seen.insert(colour);
        unprocessed_colours.append(
            &mut containing_map
                .get(colour)
                .unwrap_or(&vec![])
                .iter()
                .filter(|col| !seen.contains(&col[..]))
                .map(|&c| c)
                .collect(),
        );
    }
    seen.remove(&shiny_gold[..]);
    println!("Part 1: {}", seen.len());
}

fn part2(rules: &[Rule]) {
    let container_map = create_container_graph(rules);
    let mut total_contained_store: HashMap<&String, usize> = HashMap::new();
    println!(
        "Part 2: {}",
        get_total_contained(
            &"shiny gold".into(),
            &container_map,
            &mut total_contained_store
        ) - 1 // exclude the shiny gold bag
    )
}

fn get_total_contained<'a>(
    color: &'a String,
    container_map: &HashMap<&'a String, &'a Vec<(usize, String)>>,
    total_contained_store: &mut HashMap<&'a String, usize>,
) -> usize {
    if total_contained_store.contains_key(&color) {
        return *total_contained_store.get(&color).unwrap();
    }
    let total = match container_map.get(&color) {
        Some(contained) if !contained.is_empty() => contained
            .iter()
            .map(|(count, col)| {
                println!("{} * {}", count, col);
                count * get_total_contained(col, container_map, total_contained_store)
            })
            .sum(),
        _ => 0, // this bag contains nothing
    } + 1; // + 1 for the current bag

    total_contained_store.insert(color, total);

    println!("Got {} {}", total, color);
    total
}

#[derive(Debug)]
struct Rule {
    outer: Colour,
    inner: Vec<(usize, Colour)>,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(src: &str) -> Result<Rule, Self::Err> {
        lazy_static! {
            static ref CONTAINED_BAG_RE: Regex = Regex::new(r"(\d+) ([a-zA-Z\s]+) bags?").unwrap();
        };

        let (colour, rest) = {
            let mut iter = src.split(" bags contain ");
            (
                iter.next().unwrap(),
                iter.next().unwrap().trim_end_matches("."),
            )
        };
        let mut inner_bags = vec![];
        for inner_bag in rest.split(", ") {
            if inner_bag == "no other bags" {
                continue;
            }
            let caps = CONTAINED_BAG_RE
                .captures(inner_bag)
                .ok_or(format!("{} did not match expected format", inner_bag))?;
            inner_bags.push((usize::from_str(&caps[1]).unwrap(), caps[2].into()))
        }

        Ok(Rule {
            outer: colour.into(),
            inner: inner_bags,
        })
    }
}

fn create_containing_graph<'a>(rules: &'a [Rule]) -> HashMap<&'a String, Vec<&'a String>> {
    let mut map = HashMap::new();

    for rule in rules {
        let container_colour = &rule.outer;
        for (_, contained_colour) in rule.inner.iter() {
            if !map.contains_key(contained_colour) {
                map.insert(contained_colour, vec![container_colour]);
            } else {
                map.get_mut(contained_colour)
                    .unwrap()
                    .push(container_colour)
            }
        }
    }

    map
}

fn create_container_graph<'a>(rules: &'a [Rule]) -> HashMap<&'a String, &'a Vec<(usize, String)>> {
    let mut map = HashMap::new();

    for rule in rules {
        let container_colour = &rule.outer;
        map.insert(container_colour, &rule.inner);
    }

    map
}
