use std::fs::read_to_string;
use regex::Regex;

pub fn solve() {
    let input = read_to_string("input/3.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut ret: i64 = 0;
    for (_, [n, m]) in re.captures_iter(input).map(|x| x.extract()) {
        ret += n.parse::<i64>().unwrap() * m.parse::<i64>().unwrap();
    }
    return ret }

fn part2(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(d)(o)\(\)|(d)(o)n't\(\)").unwrap();

    let mut ret: i64 = 0;
    let mut on: bool = true;
    for c in re.captures_iter(input) {
        let m = c.get(0).unwrap().as_str();
        match &m[..3] {
            "do(" => on = true,
            "don" => on = false,
            _ => if on {
                let (_, [n, m]) = c.extract();
                 ret += n.parse::<i64>().unwrap() * m.parse::<i64>().unwrap();
            }
        }
        // ret += n.parse::<i64>().unwrap() * m.parse::<i64>().unwrap();
    }
    return ret 
}
