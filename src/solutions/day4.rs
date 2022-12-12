use crate::solution::Solution;

fn to_interval(s: &str) -> (i32, i32) {
    let interval: Vec<&str> = s.split('-').collect();
    let min: i32 = interval[0].parse().unwrap();
    let max: i32 = interval[1].parse().unwrap();

    (min, max)
}

fn split_intervals(line: &str) -> (i32, i32, i32, i32) {
    let splits: Vec<&str> = line.split(",").collect();
    let (lmin, lmax) = to_interval(splits[0]);
    let (rmin, rmax) = to_interval(splits[1]);

    (lmin, lmax, rmin, rmax)
}

fn be_fully_contained(line: &str) -> bool {
    let (lmin, lmax, rmin, rmax) = split_intervals(line);

    if (lmin <= rmin && lmax >= rmax) || (lmin >= rmin && lmax <= rmax) {
        return true;
    }
    false
}

fn be_overlapped(line: &str) -> bool {
    let (lmin, lmax, rmin, rmax) = split_intervals(line);

    if (lmax < rmin) || (rmax < lmin) {
        return false;
    }

    true
}

trait Day4 {
    fn compare(&self) -> fn(&str) -> bool;
    fn process(&self, input: &str) -> String {
        input
            .lines()
            .map(|x| self.compare()(&String::from(x)))
            .filter(|x| *x)
            .count()
            .to_string()
    }
}

pub struct Day4Part1 {}
pub struct Day4Part2 {}

impl Day4 for Day4Part1 {
    fn compare(&self) -> fn(&str) -> bool {
        be_fully_contained
    }
}

impl Day4 for Day4Part2 {
    fn compare(&self) -> fn(&str) -> bool {
        be_overlapped
    }
}

impl Solution for Day4Part1 {
    fn run(&self, input: &str) -> String {
        self.process(input)
    }
}

impl Solution for Day4Part2 {
    fn run(&self, input: &str) -> String {
        self.process(input)
    }
}
