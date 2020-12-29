use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

pub fn load_groups<T, E>(file_name: &str) -> Vec<T>
where
    T: FromStr<Err = E>,
    E: std::fmt::Debug,
{
    let raw_input = load_raw_text(file_name);
    raw_input
        .split("\n\n")
        .filter_map(|i| {
            let trimmed = i.trim();
            if !trimmed.is_empty() {
                Some(T::from_str(trimmed).unwrap())
            } else {
                None
            }
        })
        .collect()
}

pub fn load_vec<T, E>(file_name: &str) -> Vec<T>
where
    T: FromStr<Err = E>,
    E: std::fmt::Debug,
{
    let raw = load_raw_text(file_name);
    raw.split('\n')
        .filter_map(|i| {
            let trimmed = i.trim();
            if !trimmed.is_empty() {
                Some(trimmed)
            } else {
                None
            }
        })
        .map(|s| T::from_str(s).unwrap())
        .collect()
}

pub fn load_single_object<T, E>(file_name: &str) -> T
where
    T: FromStr<Err = E>,
    E: std::fmt::Debug,
{
    let raw = load_raw_text(file_name);

    T::from_str(raw.trim()).unwrap()
}

pub fn load_raw_text(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn take_first_number<T, E>(src: &mut Peekable<Chars<'_>>) -> Result<T, String>
where
    T: FromStr<Err = E> + Into<usize>,
    E: std::fmt::Debug,
{
    let mut min_s = String::new();
    while let Some(next) = src.peek() {
        if next.is_ascii_digit() {
            min_s.push(src.next().unwrap());
        } else {
            break;
        }
    }
    T::from_str(&min_s).map_err(|e| format!("{:?} ({})", e, min_s))
}

pub fn modulo<T>(a: T, b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy,
{
    ((a % b) + b) % b
}
