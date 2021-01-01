use common::load_single_object;
use std::str::FromStr;

fn main() {
    let input: Input = load_single_object("input/day19.txt");
    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    let n_valid_msgs = input
        .strings
        .iter()
        .filter(|s| input.rules[0].rule.matches(s, &input.rules))
        .count();

    println!("Part 1: {}", n_valid_msgs);
}

fn part2(input: &Input) {
    let mut rules = input.rules.clone();
    rules[8].rule = Rule::Any(vec![
        Rule::Seq(vec![Rule::Ref(42)]),
        Rule::Seq(vec![Rule::Ref(42), Rule::Ref(8)]),
    ]);
    rules[11].rule = Rule::Any(vec![
        Rule::Seq(vec![Rule::Ref(42), Rule::Ref(31)]),
        Rule::Seq(vec![Rule::Ref(42), Rule::Ref(11), Rule::Ref(31)]),
    ]);
    let n_valid_msgs = input
        .strings
        .iter()
        .filter(|s| Rule::Part2Rule0.matches(s, &rules))
        .count();

    println!("Part 2: {}", n_valid_msgs)
}

#[derive(Debug, Clone)]
enum Rule {
    Single(char),
    Seq(Vec<Rule>),
    Any(Vec<Rule>),
    Ref(usize),
    Part2Rule0,
}

#[derive(Debug, Clone)]
struct Input {
    rules: Vec<RuleEntry>,
    strings: Vec<String>,
}

#[derive(Debug, Clone)]
struct RuleEntry {
    rule: Rule,
    id: usize,
}

#[derive(Debug)]
struct Look<'a> {
    inner: &'a Vec<char>,
    position: usize,
}

impl<'a> Look<'a> {
    fn new(src: &'a Vec<char>) -> Self {
        Look {
            inner: src,
            position: 0,
        }
    }

    fn weak_clone(&self) -> Self {
        Self {
            inner: self.inner,
            position: self.position,
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.at_end() {
            None
        } else {
            Some(self.inner[self.position])
        }
    }

    fn next(&mut self) -> Option<char> {
        let ans = self.peek()?;
        self.position += 1;
        Some(ans)
    }

    fn at_end(&self) -> bool {
        self.position >= self.inner.len()
    }

    fn set_from(&mut self, other: &Self) {
        self.position = other.position;
    }
}

impl Rule {
    fn matches(&self, input: &str, rules: &[RuleEntry]) -> bool {
        let chars = input.chars().collect();
        let mut look = Look::new(&chars);

        self.match_inner(&mut look, rules) && look.at_end()
    }

    fn match_inner(&self, input: &mut Look, rule_entries: &[RuleEntry]) -> bool {
        use Rule::*;
        match self {
            Single(c) => input.next() == Some(*c),
            Seq(rules) => {
                let mut matched = true;
                for rule in rules {
                    if !rule.match_inner(input, rule_entries) {
                        matched = false;
                        break;
                    }
                }
                matched
            }
            Any(rules) => {
                let mut matched = false;
                for rule in rules {
                    let mut cloned = input.weak_clone();
                    if rule.match_inner(&mut cloned, rule_entries) {
                        input.set_from(&cloned);
                        matched = true;
                        break;
                    }
                }
                matched
            }
            Ref(id) => rule_entries[*id].rule.match_inner(input, rule_entries),
            Part2Rule0 => {
                /* === An awful hack ===
                 * I observed in my input that the only places 8 and 11 are in use are in rule 0 itself
                 * and that rule 8 and 11 together accept input that matches a chain of m [42]s followed
                 * by n [31]s for which m > n, m >= 2.
                 * So I harcoded that instead of trying to have an arbitrary rewind ¯\_(ツ)_/¯
                 */
                if !rule_entries[42].rule.match_inner(input, rule_entries) {
                    false
                } else {
                    let mut count_42_match = 1;
                    while !input.at_end() {
                        if rule_entries[31]
                            .rule
                            .match_inner(&mut input.weak_clone(), rule_entries)
                        {
                            if let Some(count_31_match) = rule_entries[31]
                                .rule
                                .repeat_to_end(&mut input.weak_clone(), rule_entries)
                            {
                                if count_31_match < count_42_match {
                                    input.position = input.inner.len();
                                    return true;
                                }
                            }
                        }
                        if !rule_entries[42].rule.match_inner(input, rule_entries) {
                            return false;
                        } else {
                            count_42_match += 1
                        }
                    }
                    false
                }
            }
        }
    }

    fn repeat_to_end(&self, input: &mut Look, rule_entries: &[RuleEntry]) -> Option<usize> {
        let mut count = 0;
        while !input.at_end() {
            if !self.match_inner(input, rule_entries) {
                return None;
            }
            count += 1;
        }
        Some(count)
    }
}

impl FromStr for RuleEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = {
            let mut parts = s.split(":");
            let part1 = parts.next().unwrap();
            let part2 = parts
                .next()
                .ok_or(String::from("Expected ':' in rule"))?
                .trim();

            (usize::from_str(part1).map_err(|e| format!("{}", e))?, part2)
        };

        if rest.starts_with('"') {
            if rest.ends_with('"') && rest.len() == 3 {
                Ok(RuleEntry {
                    id,
                    rule: Rule::Single(*&rest[1..].chars().next().unwrap()),
                })
            } else {
                Err(format!("Malformed rule: {}", rest))
            }
        } else {
            let mut options = vec![];
            for option_part in rest.split("|") {
                let mut seq = vec![];
                for id in option_part.trim().split(" ") {
                    let id = usize::from_str(id).map_err(|e| format!("{}", e))?;
                    seq.push(Rule::Ref(id))
                }
                options.push(Rule::Seq(seq))
            }

            if options.len() > 1 {
                Ok(RuleEntry {
                    id,
                    rule: Rule::Any(options),
                })
            } else if options.len() == 1 {
                Ok(RuleEntry {
                    id,
                    rule: options[0].to_owned(),
                })
            } else {
                Err("Failed to parse any rules!".into())
            }
        }
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, strings) = {
            let mut parts = s.split("\n\n");
            let part1 = parts.next().unwrap().trim();
            let part2 = parts
                .next()
                .ok_or(String::from("Expected single blank line in input"))?
                .trim();

            (part1, part2)
        };

        let mut rules = rules
            .lines()
            .map(|line| RuleEntry::from_str(line))
            .collect::<Result<Vec<_>, _>>()?;
        rules.sort_by(|a, b| a.id.cmp(&b.id));

        if rules
            .iter()
            .enumerate()
            .any(|(idx, rule_entry)| rule_entry.id != idx)
        {
            return Err(
                "Not all rules are present! Try replacing the vec with a hashmap<usize, ruleentry>"
                    .into(),
            );
        }

        let strings = strings
            .trim()
            .lines()
            .map(|line| line.trim().into())
            .collect();

        Ok(Input { rules, strings })
    }
}
