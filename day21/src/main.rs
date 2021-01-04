use common::load_vec;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    let input: Vec<Food> = load_vec("input/day21.txt");
    let map = ingredient_allergen_map(&input);
    let assigned_ingredients = map.values().collect::<HashSet<_>>();
    let non_allergenic: Vec<_> = all_ingredients(&input)
        .into_iter()
        .filter(|i| !assigned_ingredients.contains(*i))
        .collect();

    let occurrences: usize = input
        .iter()
        .map(|f| {
            non_allergenic
                .iter()
                .filter(|n| f.ingredients.contains(&n[..]))
                .count()
        })
        .sum();

    println!("Part 1: {}", occurrences);

    println!(
        "Part 2: {}",
        create_canonical_dangerous_ingredient_list(&map)
    );
}

fn create_canonical_dangerous_ingredient_list(src: &HashMap<String, String>) -> String {
    let mut elements: Vec<_> = src.iter().collect();
    elements.sort_by(|(k1, _), (k2, _)| k1.cmp(&k2));
    elements
        .iter()
        .fold(None, |acc, (_, v)| match acc {
            None => Some((*v).clone()),
            Some(a) => Some(String::from(a + "," + v)),
        })
        .unwrap()
}

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn ingredient_allergen_map(food: &[Food]) -> HashMap<String, String> {
    let mut made_progress = true;
    let mut allergen_ingred_map = allergen_ingred_map(&food);
    let allergens_count = allergen_ingred_map.len();
    let mut assigned_allergens: HashMap<String, String> = HashMap::new();
    let mut assigned_ingredients: HashSet<String> = HashSet::new();
    while made_progress && assigned_allergens.len() < allergens_count {
        made_progress = false;
        allergen_ingred_map = allergen_ingred_map
            .iter()
            .map(|(allergen, ingredients)| {
                (
                    allergen.clone(),
                    ingredients
                        .iter()
                        .filter(|ingred| !assigned_ingredients.contains(*ingred))
                        .map(|ingred| String::from(ingred))
                        .collect(),
                )
            })
            .collect();
        assigned_ingredients = HashSet::new();
        for (allergen, ingredients) in allergen_ingred_map.iter() {
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap().clone();
                made_progress = true;
                assigned_allergens.insert(allergen.to_string(), ingredient.clone());
                assigned_ingredients.insert(ingredient);
            }
        }
    }

    if assigned_allergens.len() < allergens_count {
        panic!("FAILED to make progress");
    }

    assigned_allergens
}

fn allergen_ingred_map(food: &[Food]) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for food_item in food {
        for allergen in &food_item.allergens {
            if !map.contains_key(&allergen[..]) {
                map.insert(allergen.clone(), food_item.ingredients.clone());
            } else {
                let set = map.remove(&allergen[..]).unwrap();
                map.insert(
                    allergen.clone(),
                    set.intersection(&food_item.ingredients)
                        .map(|c| c.clone())
                        .collect(),
                );
            }
        }
    }
    map
}

fn all_ingredients<'a>(food: &'a [Food]) -> HashSet<&'a String> {
    food.iter().fold(HashSet::new(), |acc, i| {
        acc.union(&i.ingredients.iter().collect())
            .map(|i| *i)
            .collect()
    })
}

impl FromStr for Food {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ingredients_s, allergens_s) = {
            let mut parts = s.split("(contains");

            (
                parts.next().unwrap().trim(),
                parts
                    .next()
                    .ok_or(String::from("Expected input to contain '(contains ...)'"))?
                    .replace(")", "")
                    .replace(",", ""),
            )
        };
        Ok(Food {
            ingredients: ingredients_s
                .split(" ")
                .filter(|s| s.trim().len() != 0)
                .map(|s| String::from(s.trim()))
                .collect(),
            allergens: allergens_s
                .split(" ")
                .filter(|s| s.trim().len() != 0)
                .map(|s| String::from(s.trim()))
                .collect(),
        })
    }
}
