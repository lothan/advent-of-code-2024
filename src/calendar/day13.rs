use std::fs::read_to_string;
use regex::Regex;
use num::integer::{gcd, lcm};

#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut ret = vec![];
    let are = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let bre = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let pre = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut a = (0,0);
    let mut b = (0,0);
    let mut prize = (0,0);
    for line in input.lines() {
        if let Some(cap) = are.captures(line) {
            a = (cap.get(1).unwrap().as_str().parse().unwrap(),
                 cap.get(2).unwrap().as_str().parse().unwrap());
        }
        if let Some(cap) = bre.captures(line) {
            b = (cap.get(1).unwrap().as_str().parse().unwrap(),
                 cap.get(2).unwrap().as_str().parse().unwrap());
        }
        if let Some(cap) = pre.captures(line) {
            prize = (cap.get(1).unwrap().as_str().parse().unwrap(),
                     cap.get(2).unwrap().as_str().parse().unwrap());
        }
        if line == "" {
            ret.push(Game { a, b, prize });
        }
    }
    ret.push(Game { a, b, prize });
    ret
}

pub fn solve() {
    let input = read_to_string("input/13.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let games = parse_input(input);
    let mut count = 0;
    for game in &games {
        let alcm = lcm(game.a.0, game.a.1);

        let xmul = alcm / game.a.0;
        let ymul = alcm / game.a.1;

        let xcoef = xmul * game.b.0;
        let ycoef = ymul * game.b.1;

        let xconst = game.prize.0 * xmul;
        let yconst = game.prize.1 * ymul;

        let coef = ycoef - xcoef;
        let _const = yconst - xconst;

        if _const % coef != 0 { println!("No solution!"); continue; }
        let b = _const / coef;
        let rem = game.prize.0 - (game.b.0 * b);

        if rem % game.a.0 != 0 { println!("No solution!"); continue; }
        let a = rem / game.a.0;

        if a < 0 || b < 0 || a > 100 || b > 100 { println!("No solution!"); continue; }
        println!("Count: {count}");
        count += a * 3 + b;
    }
    for g in &games {
        println!("{g:?}");
    }
    count
}

fn part2(input: &str) -> i64 {
    let games = parse_input(input);
    let mut count = 0;
    for game in &games {
        let alcm = lcm(game.a.0, game.a.1);

        let xmul = alcm / game.a.0;
        let ymul = alcm / game.a.1;

        let xcoef = xmul * game.b.0;
        let ycoef = ymul * game.b.1;

        let xconst = (game.prize.0 + 10000000000000) * xmul;
        let yconst = (game.prize.1 + 10000000000000) * ymul;

        let coef = ycoef - xcoef;
        let _const = yconst - xconst;

        if _const % coef != 0 { println!("No solution!"); continue; }
        let b = _const / coef;
        let rem = (game.prize.0 + 10000000000000)- (game.b.0 * b);

        if rem % game.a.0 != 0 { println!("No solution!"); continue; }
        let a = rem / game.a.0;

        if a < 0 || b < 0 { println!("No solution!"); continue; }
        println!("Count: {count}");
        count += a * 3 + b;
    }
    for g in &games {
        println!("{g:?}");
    }
    count
}
