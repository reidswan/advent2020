use std::iter::Peekable;
use std::ops::{Add, Div, Mul, Rem, Sub};
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

macro_rules! trait_alias {
    ($name:ident: $($trait_name:path)|*) => {
        pub trait $name: $( $trait_name + )* {}
        impl <T> $name for T where T: $( $trait_name +)* {}
    };
}

macro_rules! swap {
    (($a:ident, $b:ident) = ($c:expr, $d:expr)) => {{
        let temp1 = $c;
        let temp2 = $d;
        $a = temp1;
        $b = temp2;
    }};
}

trait_alias!(
    Number:
        Copy | Rem<Output = Self> | Ord | From<u8> | PartialEq |
        Add<Output = Self> | Div<Output = Self> | Mul<Output=Self> | Sub<Output=Self>
);

pub fn modulo<T>(a: T, b: T) -> T
where
    T: Number,
{
    ((a % b) + b) % b
}

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Number,
{
    let zero = T::from(0);

    while b != zero {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

pub fn extended_gcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Number,
{
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (T::from(1), T::from(0));
    let (mut old_t, mut t) = (T::from(0), T::from(1));

    let zero = T::from(0);
    while r != zero {
        let quotient = old_r / r;
        swap!((old_r, r) = (r, old_r - quotient * r));
        swap!((old_s, s) = (s, old_s - quotient * s));
        swap!((old_t, t) = (t, old_t - quotient * t));
    }

    (old_r, old_s, old_t)
}

pub fn is_coprime<T>(a: T, b: T) -> bool
where
    T: Number,
{
    gcd(a, b) == T::from(1)
}

pub fn is_coprimes<T>(numbers: &[T]) -> bool
where
    T: Number,
{
    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if !is_coprime(numbers[i], numbers[j]) {
                return false;
            }
        }
    }
    true
}

pub fn mod_inverse<T>(a: T, m: T) -> T
where
    T: Number,
{
    let (_, x, _) = extended_gcd(a, m);
    ((x % m) + m) % m
}

pub fn chinese_remainder<T>(numbers: &[(T, T)]) -> T
where
    T: Number,
{
    let zero = T::from(0);
    let one = T::from(1);
    let mod_product = numbers.iter().map(|(_, m)| *m).fold(one, |acc, i| acc * i);
    modulo(
        numbers
            .iter()
            .map(|&(a, m)| {
                let b = mod_product / m;
                let b_inverse = mod_inverse(b, m);
                modulo(a * b * b_inverse, mod_product)
            })
            .fold(zero, |acc, i| acc + i),
        mod_product,
    )
}
