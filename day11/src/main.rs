use common::load_single_object;
use std::mem::swap;
use std::str::FromStr;

const SLOPES: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let input: SeatingMap = load_single_object("input/day11.txt");
    part1(&input);
    part2(&input);
}

fn part1(input: &SeatingMap) {
    let part1_config = SeatingRuleConfig {
        occupation_type_check: OccupationCheckType::Adjacent,
        max_occupied_seats: 4,
    };
    let (mut map1, mut map2) = (input.clone(), input.clone());
    while map1.step(&mut map2, part1_config) {
        swap(&mut map1, &mut map2)
    }
    println!("Part 1: {}", map1.count_occupied())
}

fn part2(input: &SeatingMap) {
    let part2_config = SeatingRuleConfig {
        occupation_type_check: OccupationCheckType::LineOfSight,
        max_occupied_seats: 5,
    };
    let (mut map1, mut map2) = (input.clone(), input.clone());
    while map1.step(&mut map2, part2_config) {
        swap(&mut map1, &mut map2);
    }
    println!("Part 2: {}", map1.count_occupied())
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum OccupationCheckType {
    LineOfSight,
    Adjacent,
}

#[derive(Debug, Clone, Copy)]
struct SeatingRuleConfig {
    occupation_type_check: OccupationCheckType,
    max_occupied_seats: usize,
}

#[derive(Debug, Clone)]
struct SeatingMap {
    grid: Vec<Vec<SeatState>>,
    nearest_seats: Vec<Vec<Vec<(usize, usize)>>>,
    rows: isize,
    cols: isize,
}

impl SeatingMap {
    fn new(grid: Vec<Vec<SeatState>>) -> Self {
        let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);

        let nearest_seats = Self::determine_nearest_seats(&grid, rows, cols);

        SeatingMap {
            grid,
            rows,
            cols,
            nearest_seats,
        }
    }

    fn determine_nearest_seats(
        grid: &Vec<Vec<SeatState>>,
        rows: isize,
        cols: isize,
    ) -> Vec<Vec<Vec<(usize, usize)>>> {
        let mut nearest = vec![];
        for src_row in 0..rows {
            let mut nearest_for_row = vec![];
            for src_col in 0..cols {
                let mut nearest_for_seat = vec![];
                for (drow, dcol) in SLOPES.iter() {
                    let (mut row, mut col) = (src_row as isize + drow, src_col as isize + dcol);
                    while Self::coordinate_within(rows, cols, row, col) {
                        let loc = grid[row as usize][col as usize];
                        if loc.is_seat() {
                            nearest_for_seat.push((row as usize, col as usize));
                            break;
                        }
                        row += drow;
                        col += dcol;
                    }
                }
                nearest_for_row.push(nearest_for_seat)
            }
            nearest.push(nearest_for_row);
        }
        nearest
    }

    fn count_occupied(&self) -> usize {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|seat| **seat == SeatState::Occupied)
                    .count()
            })
            .sum()
    }

    fn coordinate_within(n_rows: isize, n_cols: isize, row: isize, col: isize) -> bool {
        (0 <= row && row < n_rows) && (0 <= col && col < n_cols)
    }

    fn is_valid_coordinate(&self, row: isize, col: isize) -> bool {
        Self::coordinate_within(self.rows, self.cols, row, col)
    }

    fn count_occupied_line_of_sight_seats(&self, src_row: usize, src_col: usize) -> usize {
        self.nearest_seats[src_row][src_col]
            .iter()
            .map(|&(row, col)| {
                if self.grid[row][col] == SeatState::Occupied {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn count_occupied_adjacent_seats(&self, src_row: usize, src_col: usize) -> usize {
        SLOPES
            .iter()
            .map(|(drow, dcol)| (src_row as isize + drow, src_col as isize + dcol))
            .filter(|&(row, col)| self.is_valid_coordinate(row, col))
            .map(|(row, col)| self.grid[row as usize][col as usize])
            .filter(|state| *state == SeatState::Occupied)
            .count()
    }

    fn step(&self, target: &mut Self, config: SeatingRuleConfig) -> bool {
        let mut has_changed = false;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let current_state = self.grid[row][col];
                let next_state = self.next_state(row, col, config);
                has_changed = has_changed || current_state != next_state;
                target.grid[row][col] = next_state
            }
        }

        has_changed
    }

    fn next_state(&self, src_row: usize, src_col: usize, config: SeatingRuleConfig) -> SeatState {
        let current_state = self.grid[src_row][src_col];
        if !current_state.is_seat() {
            // floor does not change
            return SeatState::Floor;
        }

        let adjacent_seats_occupied = match config.occupation_type_check {
            OccupationCheckType::Adjacent => self.count_occupied_adjacent_seats(src_row, src_col),
            OccupationCheckType::LineOfSight => {
                self.count_occupied_line_of_sight_seats(src_row, src_col)
            }
        };

        match current_state {
            SeatState::Empty => {
                if adjacent_seats_occupied == 0 {
                    SeatState::Occupied
                } else {
                    SeatState::Empty
                }
            }

            SeatState::Occupied => {
                if adjacent_seats_occupied >= config.max_occupied_seats {
                    SeatState::Empty
                } else {
                    SeatState::Occupied
                }
            }
            _ => unreachable!(),
        }
    }
}

impl FromStr for SeatingMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<SeatState>> = s
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .filter_map(|c| SeatState::from_char(c))
                    .collect()
            })
            .collect();
        if grid.len() == 0 || grid[0].len() == 0 {
            return Err("Empty grid not supported".into());
        }
        let mut size = None;
        for elem in grid.iter() {
            if size == None {
                size = Some(elem.len())
            }
            if size.unwrap() != elem.len() {
                return Err("Not all lines in grid are the same size!".into());
            }
        }
        Ok(SeatingMap::new(grid))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

impl SeatState {
    fn from_char(c: char) -> Option<SeatState> {
        Some(match c {
            'L' => SeatState::Empty,
            '.' => SeatState::Floor,
            '#' => SeatState::Occupied,
            _ => return None,
        })
    }

    fn is_seat(&self) -> bool {
        matches!(self, SeatState::Occupied | SeatState::Empty)
    }
}
