use std::{str::Lines, collections::HashSet};

use crate::solution::Solution;

fn build_charset(chars: Vec<char>) -> HashSet<char> {
    let mut charset = HashSet::<char>::new();
    chars.iter().for_each(|x| {charset.insert(*x);});

    charset
}

fn find_same_char(chars: &Vec<char>) -> char {
    let h = chars.len() / 2;

    let charset = build_charset(chars[0..h].to_vec());

    for c in chars.iter().skip(h) {
        if charset.contains(c) {
            return *c
        }
    }

    unreachable!();
}

fn priorities(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as u8 - 96) as i32;
    }

    (c as u8 - 64) as i32 + 26
}

pub struct Day3Part1 {}
pub struct Day3Part2 {}

trait Day3 {
    fn process(&self, lines: Lines) -> i32;
}

impl Day3 for Day3Part1 {
    fn process(&self, lines: Lines) -> i32 {
        let mut total = 0;
        for line in lines {
            let cs: Vec<char> = line.chars().collect();
            let same = find_same_char(&cs);
            total += priorities(same);
        }
        total
    }
}

impl Solution for Day3Part1 {
    fn run(&self, input: &str) -> String {
        self.process(input.lines()).to_string()
    }
}

fn update_charset(group: &[String], charset: &mut HashSet<char>) {
    for item in group {
        let temp_charset = build_charset(item.chars().collect());
        charset.retain(|x| temp_charset.contains(x));
    }
}

impl Day3 for Day3Part2 {
    fn process(&self, lines: Lines) -> i32 {
        let l: Vec<String> = lines.map(String::from).collect();
        let groups: Vec<&[String]> = l.chunks(3).collect();
        let mut points = 0;

        for group in groups {
            let mut charset = build_charset(group[0].chars().collect());
            update_charset(&group[1..], &mut charset);

            if charset.len() > 1 {
                unreachable!();
            }

            let item = charset.iter().next().unwrap();
            points += priorities(*item);
        }

        points
    }
}

impl Solution for Day3Part2 {
    fn run(&self, input: &str) -> String {
        self.process(input.lines()).to_string()
    }
}
