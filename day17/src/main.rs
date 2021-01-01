mod neighbors;
use common::load_single_object;
use neighbors::{NEIGHBOURS_3D, NEIGHBOURS_4D};
use num_cpus;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

fn main() {
    let thread_count = max(4, num_cpus::get() - 1);
    let threadpool = ThreadPool::new(thread_count);

    println!("Part 1: {}", run::<Coord3D>(&threadpool));
    println!("Part 2: {}", run::<Coord4D>(&threadpool));
}

fn run<T>(pool: &ThreadPool) -> usize
where
    T: Coord + Eq + std::hash::Hash + Send + Sync + 'static,
{
    let mut input: GameOfLife<T> = load_single_object("input/day17.txt");
    for _ in 0..6 {
        input = input.step(&pool);
    }

    input.active_cubes.len()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum CubeState {
    Active,
    Inactive,
}

impl Default for CubeState {
    fn default() -> Self {
        Self::Inactive
    }
}

impl CubeState {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Inactive,
            '#' => Self::Active,
            _ => unreachable!(),
        }
    }
}

trait Coord
where
    Self: Sized + Copy,
{
    fn shift_all_by(&self, val: isize) -> Self;

    fn get_neighbours<'a>(&'a self) -> &'a [Self];

    fn add(&self, other: &Self) -> Self;

    fn for_each_in_limits<F>(lower_limits: &Self, upper_limits: &Self, f: F)
    where
        F: FnMut(Self) -> ();

    fn take_each_max(&self, other: &Self) -> Self;

    fn take_each_min(&self, other: &Self) -> Self;

    fn get_limits(points: &HashSet<Self>) -> Option<(Self, Self)> {
        points.iter().fold(None, |acc, elem| match acc {
            None => Some((*elem, *elem)),
            Some((mins, maxs)) => Some((mins.take_each_min(elem), maxs.take_each_max(elem))),
        })
    }

    fn from_pair(x: isize, y: isize) -> Self;
}

type Coord3D = (isize, isize, isize);
type Coord4D = (isize, isize, isize, isize);

impl Coord for Coord3D {
    fn shift_all_by(&self, val: isize) -> Self {
        let (x, y, z) = self;
        (x + val, y + val, z + val)
    }

    fn get_neighbours(&self) -> &'static [Self] {
        &NEIGHBOURS_3D
    }

    fn add(&self, other: &Self) -> Self {
        let (x, y, z) = self;
        let (x1, y1, z1) = other;
        (x + x1, y + y1, z + z1)
    }

    fn for_each_in_limits<F>(lower_limits: &Self, upper_limits: &Self, mut f: F)
    where
        F: FnMut(Self) -> (),
    {
        let &(x_max, y_max, z_max) = upper_limits;
        let &(x_min, y_min, z_min) = lower_limits;
        for x in x_min..x_max {
            for y in y_min..y_max {
                for z in z_min..z_max {
                    f((x, y, z))
                }
            }
        }
    }

    fn take_each_min(&self, other: &Self) -> Self {
        let (x, y, z) = self;
        let (x1, y1, z1) = other;

        (min(*x, *x1), min(*y, *y1), min(*z, *z1))
    }

    fn take_each_max(&self, other: &Self) -> Self {
        let (x, y, z) = self;
        let (x1, y1, z1) = other;

        (max(*x, *x1), max(*y, *y1), max(*z, *z1))
    }

    fn from_pair(x: isize, y: isize) -> Self {
        (x, y, 0)
    }
}

impl Coord for Coord4D {
    fn shift_all_by(&self, val: isize) -> Self {
        let (x, y, z, w) = self;
        (x + val, y + val, z + val, w + val)
    }

    fn get_neighbours(&self) -> &'static [Self] {
        &NEIGHBOURS_4D
    }

    fn add(&self, other: &Self) -> Self {
        let (x, y, z, w) = self;
        let (x1, y1, z1, w1) = other;
        (x + x1, y + y1, z + z1, w + w1)
    }

    fn for_each_in_limits<F>(lower_limits: &Self, upper_limits: &Self, mut f: F)
    where
        F: FnMut(Self) -> (),
    {
        let &(x_max, y_max, z_max, w_max) = upper_limits;
        let &(x_min, y_min, z_min, w_min) = lower_limits;
        for x in x_min..x_max {
            for y in y_min..y_max {
                for z in z_min..z_max {
                    for w in w_min..w_max {
                        f((x, y, z, w))
                    }
                }
            }
        }
    }

    fn take_each_min(&self, other: &Self) -> Self {
        let (x, y, z, w) = self;
        let (x1, y1, z1, w1) = other;

        (min(*x, *x1), min(*y, *y1), min(*z, *z1), min(*w, *w1))
    }

    fn take_each_max(&self, other: &Self) -> Self {
        let (x, y, z, w) = self;
        let (x1, y1, z1, w1) = other;

        (max(*x, *x1), max(*y, *y1), max(*z, *z1), max(*w, *w1))
    }

    fn from_pair(x: isize, y: isize) -> Self {
        (x, y, 0, 0)
    }
}

#[derive(Debug)]
struct GameOfLife<T>
where
    T: Coord,
{
    active_cubes: HashSet<T>,
    upper_limits: T,
    lower_limits: T,
}

impl<T> GameOfLife<T>
where
    T: Coord + Eq + std::hash::Hash + 'static + Send + Sync,
{
    fn count_active_neighbours(active_cubes: &HashSet<T>, coord: T) -> usize {
        coord
            .get_neighbours()
            .iter()
            .filter(|neighbour| active_cubes.contains(&coord.add(neighbour)))
            .count()
    }

    fn should_update_state(active_cubes: Arc<HashSet<T>>, coord: T) -> bool {
        let active_neighbours = Self::count_active_neighbours(&active_cubes, coord);
        if active_cubes.contains(&coord) {
            active_neighbours == 2 || active_neighbours == 3
        } else {
            active_neighbours == 3
        }
    }

    fn step(mut self, pool: &ThreadPool) -> Self {
        let (lower, upper) = self.get_limits();

        // add to the limits to consider all values whose state may change
        let upper = upper.shift_all_by(2);
        let lower = lower.shift_all_by(-2);
        let active_cubes = Arc::new(self.active_cubes);
        let rx = {
            let (tx, rx) = channel();
            T::for_each_in_limits(&lower, &upper, |coord| {
                let active_cubes = Arc::clone(&active_cubes);
                let tx = tx.clone();
                pool.execute(move || {
                    if Self::should_update_state(active_cubes, coord) {
                        tx.send(coord).unwrap();
                    }
                })
            });
            rx
        };

        GameOfLife {
            active_cubes: rx.iter().collect(),
            lower_limits: self.lower_limits,
            upper_limits: self.upper_limits,
        }
    }

    fn get_limits(&mut self) -> (T, T) {
        T::get_limits(&self.active_cubes).unwrap()
    }
}

impl<T> FromStr for GameOfLife<T>
where
    T: Coord + Eq + std::hash::Hash,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let bad_ch = s
            .chars()
            .filter(|&c| c != '.' && c != '#' && c != '\n')
            .collect::<Vec<_>>();
        if !bad_ch.is_empty() {
            return Err(format!("Source string contained bad chars: {:?}", bad_ch));
        }
        let active_cubes: HashSet<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (
                            T::from_pair(x as isize, y as isize),
                            CubeState::from_char(c),
                        )
                    })
                    .collect::<Vec<(T, CubeState)>>()
            })
            .filter_map(|(c, state)| {
                if state == CubeState::Active {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
        let (upper_limits, lower_limits) = T::get_limits(&active_cubes).unwrap();

        Ok(GameOfLife {
            active_cubes,
            upper_limits,
            lower_limits,
        })
    }
}
