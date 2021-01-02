use common::load_groups;
use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input: Vec<Image> = load_groups("input/day20.txt");
    part1(&input);
}

fn part1(input: &[Image]) {
    let mut all_possible_sides = HashMap::new();
    for ib in input.iter() {
        add_all(&mut all_possible_sides, &ib.borders.possible_sides(), ib.id)
    }

    let corners_id_product: usize = input
        .iter()
        .filter(|ib| {
            let possible_sides = ib.borders.possible_sides();
            possible_sides
                .iter()
                .filter(|c| all_possible_sides.get(c).unwrap().len() > 1)
                .count()
                <= 2
        })
        .map(|corn| corn.id)
        .product();

    println!("Part 1: {}", corners_id_product);
}

fn part2(input: &[Image]) {}

fn add_all<T, U>(target: &mut HashMap<T, Vec<U>>, sides: &[T], id: U)
where
    T: Eq + std::hash::Hash + Copy,
    U: Copy,
{
    for side in sides {
        if target.contains_key(side) {
            target.get_mut(side).unwrap().push(id)
        } else {
            target.insert(*side, vec![id]);
        }
    }
}

// top and right are read backwards btw
#[derive(Copy, Clone, Debug, PartialEq)]
struct ImageBorders {
    top: u16,
    left: u16,
    right: u16,
    bottom: u16,
}

#[derive(Debug, Clone)]
struct Image {
    id: usize,
    borders: ImageBorders,
    contents: Vec<String>,
}

fn reverse(mut s: u16) -> u16 {
    let mut reversed = 0;
    // only consider lower 10 digits of the 'border'
    for _ in 0..10 {
        reversed <<= 1;
        if s & 1 == 1 {
            reversed ^= 1
        }
        s >>= 1
    }
    reversed
}

impl ImageBorders {
    fn possible_sides(self) -> Vec<u16> {
        // we normalize each side to the min of (side, reverse(side))
        vec![self.top, self.bottom, self.left, self.right]
            .into_iter()
            .map(|c| min(c, reverse(c)))
            .collect()
    }

    fn flip_vertical(self) -> Self {
        let ImageBorders {
            top,
            left,
            right,
            bottom,
        } = self;
        // reverse left and right, swap top and bottom
        ImageBorders {
            left: reverse(left),
            right: reverse(right),
            top: bottom,
            bottom: top,
        }
    }

    fn flip_horizontal(self) -> Self {
        let ImageBorders {
            top,
            left,
            right,
            bottom,
        } = self;
        // reverse left and right, swap top and bottom
        ImageBorders {
            top: reverse(top),
            bottom: reverse(bottom),
            left: right,
            right: left,
        }
    }

    fn rotate_anticlock(self) -> Self {
        // top -(r)-> left -> bottom -(r)-> right -> top
        let ImageBorders {
            top,
            left,
            right,
            bottom,
        } = self;
        // reverse left and right, swap top and bottom
        ImageBorders {
            left: reverse(top),
            bottom: left,
            right: reverse(bottom),
            top: right,
        }
    }

    fn rotate_clock(self) -> Self {
        // top -> right -(r)> bottom -> left -(r)> top
        let ImageBorders {
            top,
            left,
            right,
            bottom,
        } = self;
        // reverse left and right, swap top and bottom
        ImageBorders {
            top: reverse(left),
            left: bottom,
            bottom: reverse(right),
            right: top,
        }
    }
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (top_line, image_lines) = {
            let mut lines = s.lines();

            (lines.next().unwrap(), lines.collect::<Vec<_>>())
        };

        let id_s = top_line
            .trim()
            .trim_start_matches("Tile ")
            .trim_end_matches(":");
        let id = usize::from_str(id_s).map_err(|e| format!("{}", e))?;

        if image_lines.len() != 10 {
            return Err("Expected a 10x10 image tile".into());
        }

        if !image_lines.iter().all(|line| line.len() == 10) {
            return Err("Expected a 10x10 image tile".into());
        }

        let top = border_to_u16(image_lines[0].chars())?;
        let bottom = border_to_u16(image_lines[9].chars())?;
        let left = border_to_u16(image_lines.iter().map(|line| line.chars().next().unwrap()))?;
        let right = border_to_u16(image_lines.iter().map(|line| line.chars().last().unwrap()))?;
        let borders = ImageBorders {
            top,
            bottom,
            left,
            right,
        };

        let contents = image_lines
            .iter()
            .map(|line| String::from(&line[1..9]))
            .collect();

        Ok(Image {
            id,
            borders,
            contents,
        })
    }
}

fn border_to_u16<T>(border: T) -> Result<u16, String>
where
    T: Iterator<Item = char>,
{
    let bin_str = border
        .map(|c| {
            if c == '#' {
                Some('1')
            } else if c == '.' {
                Some('0')
            } else {
                None
            }
        })
        .collect::<Option<String>>()
        .ok_or(String::from("Expected only '#' and '.' in border chars"))?;

    Ok(u16::from_str_radix(&bin_str, 2).unwrap())
}
