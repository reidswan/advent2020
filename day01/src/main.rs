use common::load_vec;

fn main() {
    let (src, inversions) = {
        let mut src = load_vec("input/day1.txt");
        src.sort();
        let mut inversions = invert_arr(&src, 2020);
        inversions.reverse(); // sorts inversions
        (src, inversions)
    };

    part1(&src, &inversions);
    part2(&src);
}

fn part1(src: &[i64], inversions: &[i64]) {
    let (a, b) = find_pair(src, inversions);
    println!("Part 1: {}", a * b)
}

fn part2(src: &[i64]) {
    let (a, b, c) = find_trio(src, 2020);
    println!("Part 2: {}", a * b * c)
}

fn invert_arr(src: &[i64], from: i64) -> Vec<i64> {
    src.into_iter().map(|i| from - i).collect()
}

fn find_pair(src: &[i64], inversions: &[i64]) -> (i64, i64) {
    let mut inversions = inversions.iter();
    for s in src {
        while let Some(current_inversion) = inversions.next() {
            if s == current_inversion {
                return (*s, 2020 - s);
            } else if s < current_inversion {
                break; // s too small; go to the next s value
            }
        }
    }

    unreachable!();
}

fn find_trio(src: &[i64], target: i64) -> (i64, i64, i64) {
    let mut first = 2;
    while first < src.len() {
        let mut second = 1;
        while second < first && src[first] + src[second] < target {
            let mut third = 0;
            while third < second {
                let sum = src[first] + src[second] + src[third];
                if sum == target {
                    return (src[first], src[second], src[third]);
                } else if sum > target {
                    break;
                }
                third += 1;
            }
            second += 1;
        }
        first += 1;
    }
    unreachable!();
}
