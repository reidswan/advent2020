use common::load_vec;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let lines: Vec<TreeLine> = load_vec("input/day3.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[TreeLine]) {
    let trees_encountered = lines
        .iter()
        .enumerate()
        .filter(|(i, line)| line.is_tree_at(i * 3))
        .count();
    println!("Part 1: {}", trees_encountered)
}

fn part2(lines: &[TreeLine]) {
    let result: usize = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| count_collisions_on_slope(&lines, dx, dy))
        .product();

    println!("Part 2: {}", result)
}

fn count_collisions_on_slope(lines: &[TreeLine], dx: usize, dy: usize) -> usize {
    let (mut x, mut y, mut count) = (0, 0, 0);

    while y < lines.len() {
        if lines[y].is_tree_at(x) {
            count += 1;
        }
        x += dx;
        y += dy;
    }

    count
}

#[derive(Debug)]
struct TreeLine {
    tree_locations: HashSet<usize>,
    length: usize,
}

impl TreeLine {
    fn is_tree_at(&self, location: usize) -> bool {
        self.tree_locations.contains(&(location % self.length))
    }
}

impl FromStr for TreeLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.chars().any(|ch| ch != '#' && ch != '.') {
            return Err("Line contains characters other than '#' and '.'".into());
        }

        let length = s.len();
        let tree_locations = HashSet::from_iter(s.char_indices().filter_map(|(index, ch)| {
            if ch == '#' {
                Some(index)
            } else {
                None
            }
        }));
        Ok(TreeLine {
            tree_locations,
            length,
        })
    }
}
