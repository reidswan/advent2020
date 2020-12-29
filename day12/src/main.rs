use common::{load_vec, modulo};
use std::str::FromStr;

fn main() {
    let input: Vec<Movement> = load_vec("input/day12.txt");
    part1(&input);
    part2(&input);
}

fn part1(input: &[Movement]) {
    println!(
        "Part 1: {}",
        PositionWithDirection::default().navigate_and_get_position(input)
    )
}

fn part2(input: &[Movement]) {
    println!(
        "Part 2: {}",
        PositionWithWaypoint::default().navigate_and_get_position(input)
    )
}

#[derive(Debug, Copy, Clone, Default)]
struct PositionWithDirection {
    x: isize,
    y: isize,
    direction: CardinalDirection,
}

#[derive(Debug, Copy, Clone)]
struct PositionWithWaypoint {
    x: isize,
    y: isize,
    waypoint: Waypoint,
}

impl Default for PositionWithWaypoint {
    fn default() -> Self {
        PositionWithWaypoint {
            x: 0,
            y: 0,
            waypoint: Waypoint { x: 10, y: -1 },
        }
    }
}

trait Positional: Default {
    fn x(&self) -> isize;

    fn y(&self) -> isize;

    fn navigate(&mut self, movement: &Movement);

    fn navigate_and_get_position(&mut self, movements: &[Movement]) -> usize {
        for movement in movements {
            self.navigate(movement);
        }
        self.manhattan_distance_from_origin()
    }

    fn manhattan_distance_from_origin(&self) -> usize {
        self.manhattan_distance_from(&Self::default())
    }
    fn manhattan_distance_from(&self, other: &Self) -> usize {
        (self.x() - other.x()).abs() as usize + (self.y() - other.y()).abs() as usize
    }
}

impl Positional for PositionWithWaypoint {
    fn x(&self) -> isize {
        self.x
    }
    fn y(&self) -> isize {
        self.y
    }

    fn navigate(&mut self, movement: &Movement) {
        use MovementType::*;
        match movement.movement_type {
            North => self.waypoint.y -= movement.amount as isize,
            South => self.waypoint.y += movement.amount as isize,
            East => self.waypoint.x += movement.amount as isize,
            West => self.waypoint.x -= movement.amount as isize,
            Left => {
                self.waypoint = self
                    .waypoint
                    .rotate(TurnDirection::CounterClockwise, movement.amount)
            }
            Right => {
                self.waypoint = self
                    .waypoint
                    .rotate(TurnDirection::Clockwise, movement.amount)
            }
            Forward => {
                let Waypoint { x, y } = &self.waypoint;
                self.x += *x * (movement.amount as isize);
                self.y += *y * (movement.amount as isize);
            }
        }
    }
}

impl Positional for PositionWithDirection {
    fn x(&self) -> isize {
        self.x
    }
    fn y(&self) -> isize {
        self.y
    }
    fn navigate(&mut self, movement: &Movement) {
        use MovementType::*;
        match movement.movement_type {
            North => self.y -= movement.amount as isize,
            South => self.y += movement.amount as isize,
            East => self.x += movement.amount as isize,
            West => self.x -= movement.amount as isize,
            Left => {
                self.direction = self
                    .direction
                    .turn(TurnDirection::CounterClockwise, movement.amount)
            }
            Right => {
                self.direction = self
                    .direction
                    .turn(TurnDirection::Clockwise, movement.amount)
            }
            Forward => {
                let (dx, dy) = self.direction.move_in_direction(movement.amount);
                self.x += dx;
                self.y += dy;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

const DIRECTION_ORDER: [CardinalDirection; 4] = [
    CardinalDirection::North,
    CardinalDirection::East,
    CardinalDirection::South,
    CardinalDirection::West,
];

enum TurnDirection {
    Clockwise,
    CounterClockwise,
}

impl CardinalDirection {
    fn turn(self, direction: TurnDirection, amount: usize) -> Self {
        if amount % 90 != 0 {
            panic!("Expected only right turns!");
        }

        let amount = ((amount % 360) / 90) as isize;
        let idx = (DIRECTION_ORDER.iter().position(|&i| i == self).unwrap()) as isize;
        DIRECTION_ORDER[match direction {
            TurnDirection::CounterClockwise => {
                modulo(idx - amount, DIRECTION_ORDER.len() as isize) as usize
            }
            TurnDirection::Clockwise => {
                modulo(idx + amount, DIRECTION_ORDER.len() as isize) as usize
            }
        }]
    }

    fn move_in_direction(self, amount: usize) -> (isize, isize) {
        use CardinalDirection::*;
        let amount = amount as isize;
        match self {
            East => (amount, 0),
            West => (-amount, 0),
            North => (0, -amount),
            South => (0, amount),
        }
    }
}

impl Default for CardinalDirection {
    fn default() -> Self {
        CardinalDirection::East
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Movement {
    movement_type: MovementType,
    amount: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MovementType {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl MovementType {
    fn from_char(c: char) -> Self {
        use MovementType::*;
        match c {
            'N' => North,
            'S' => South,
            'E' => East,
            'W' => West,
            'L' => Left,
            'R' => Right,
            'F' => Forward,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let movement_type = MovementType::from_char(s.chars().next().unwrap());
        let amount = usize::from_str(&s[1..]).unwrap();

        Ok(Movement {
            movement_type,
            amount,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    x: isize,
    y: isize,
}

impl Waypoint {
    fn rotate(self, direction: TurnDirection, amount: usize) -> Self {
        if amount % 90 != 0 {
            panic!("Expected only right turns!");
        }

        let mut num_turns = ((amount % 360) / 90) as isize;
        if num_turns == 0 {
            return self;
        }

        // normalize to CounterClockwise
        if let TurnDirection::Clockwise = direction {
            num_turns = 4 - num_turns;
        }

        let Waypoint { x, y } = self;

        match num_turns {
            1 => Waypoint { x: y, y: -x },
            2 => Waypoint { x: -x, y: -y },
            3 => Waypoint { x: -y, y: x },
            _ => unreachable!(),
        }
    }
}
