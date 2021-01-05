use common::load_vec;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;
use Direction::*;

fn main() {
    let input: Vec<TileDescription> = load_vec("input/day24.txt");
    let mut layout = initial_layout(&input);
    let black_tiles_count = count_with_color(&layout, TileColor::Black);
    println!("Part 1: {}", black_tiles_count);

    for _ in 0..100 {
        update_layout(&mut layout);
    }

    let updated_black_tiles_count = count_with_color(&layout, TileColor::Black);
    println!("Part 2: {}", updated_black_tiles_count);
}

fn count_with_color(layout: &HashMap<Coord, TileColor>, color: TileColor) -> usize {
    layout.values().filter(|&&i| i == color).count()
}

const ALL_DIRECTIONS: [Direction; 6] = [East, West, SouthEast, SouthWest, NorthEast, NorthWest];

fn initial_layout(input: &[TileDescription]) -> HashMap<Coord, TileColor> {
    let mut tile_colors: HashMap<Coord, TileColor> = HashMap::new();

    for description in input {
        let coord = get_coord(&description.directions);
        tile_colors.insert(coord, tile_colors.get(&coord).unwrap_or_default().flip());
    }

    tile_colors
}

fn update_layout(src: &mut HashMap<Coord, TileColor>) {
    let mut changes = vec![];

    let (min_edge, max_edge) = {
        let (min, max) = bounding_rectangle(src.keys());

        (min + Coord(-2, -2), max + Coord(2, 2))
    };

    for x in min_edge.0..=max_edge.0 {
        for y in min_edge.1..=max_edge.1 {
            if !Coord::is_valid_pair(x, y) {
                continue;
            };
            let coord = Coord(x, y);
            let color = *src.get(&coord).unwrap_or_default();
            let matching_neighbours = count_neighbours_with_color(src, coord, TileColor::Black);
            let should_flip = if color == TileColor::Black {
                matching_neighbours == 0 || matching_neighbours > 2
            } else {
                matching_neighbours == 2
            };
            if should_flip {
                changes.push((coord, color.flip()));
            }
        }
    }

    for (coord, color) in changes {
        src.insert(coord, color);
    }
}

fn bounding_rectangle<'a, T: Iterator<Item = &'a Coord>>(src: T) -> (Coord, Coord) {
    src.fold(None, |opt, c| match opt {
        None => Some((*c, *c)),
        Some((min, max)) => Some((min.each_min(c), max.each_max(c))),
    })
    .unwrap()
}

fn count_neighbours_with_color(
    src: &HashMap<Coord, TileColor>,
    at: Coord,
    color: TileColor,
) -> usize {
    ALL_DIRECTIONS
        .iter()
        .filter(|dir| src.get(&(at + dir.as_coord())).unwrap_or_default() == &color)
        .count()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileColor {
    Black,
    White,
}

impl TileColor {
    fn flip(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl Default for TileColor {
    fn default() -> Self {
        Self::White
    }
}

impl Default for &TileColor {
    fn default() -> Self {
        &TileColor::White
    }
}

#[derive(Debug, Clone)]
struct TileDescription {
    directions: Vec<Direction>,
}

#[derive(Debug, PartialEq, Eq, std::hash::Hash, Copy, Clone)]
struct Coord(isize, isize);

impl Coord {
    fn each_max(&self, other: &Self) -> Self {
        Coord(max(self.0, other.0), max(self.1, other.1))
    }

    fn each_min(&self, other: &Self) -> Self {
        Coord(min(self.0, other.0), min(self.1, other.1))
    }

    fn is_valid_pair(x: isize, y: isize) -> bool {
        (x + y) % 2 == 0
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Self) -> Self {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}

impl Sum for Coord {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Coord(0, 0), |a, b| a + b)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    East,
    West,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

impl Direction {
    fn as_coord(&self) -> Coord {
        match self {
            East => Coord(2, 0),
            West => Coord(-2, 0),
            SouthEast => Coord(1, -1),
            SouthWest => Coord(-1, -1),
            NorthEast => Coord(1, 1),
            NorthWest => Coord(-1, 1),
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "e" => East,
            "w" => West,
            "se" => SouthEast,
            "sw" => SouthWest,
            "ne" => NorthEast,
            "nw" => NorthWest,
            _ => unreachable!(),
        }
    }
}

fn get_coord(directions: &[Direction]) -> Coord {
    directions.iter().map(|d| d.as_coord()).sum()
}

impl FromStr for TileDescription {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut directions = vec![];
        loop {
            directions.push(match chars.next() {
                None => break,
                Some(c) if c == 'e' || c == 'w' => Direction::from_str(&c.to_string()),
                Some(c) if c == 's' || c == 'n' => {
                    let c2 = chars.next().ok_or(format!("Unexpected end of input"))?;
                    if c2 != 'e' && c2 != 'w' {
                        return Err(format!("Unexpected direction: {}{}", c, c2));
                    }
                    Direction::from_str(&format!("{}{}", c, c2))
                }
                Some(c) => return Err(format!("Unexpected character in directions: {}", c)),
            })
        }

        Ok(TileDescription { directions })
    }
}
