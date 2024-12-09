use std::fs::read_to_string;
use Operation::*;
use Operation2::*;

pub fn solve() {
    let input = read_to_string("input/7.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Equation {
    test: i64,
    nums: Vec<i64>,
}

#[derive(PartialEq, Clone, Debug)]
enum Operation {
    Add,
    Mult,
}

#[derive(PartialEq, Clone, Debug)]
enum Operation2 {
    Add,
    Mult,
    Concat,
}

// next time might be interested to find out to make this into
// a custom iterator
fn next_ops(ops: Vec<Operation>) -> Vec<Operation> {
    let mut switched = false;
    let mut ret = vec![];
    for op in ops {
        if switched { ret.push(op); continue; }
        match op {
            Operation::Add => { ret.push(Operation::Mult); switched = true },
            Operation::Mult => ret.push(Operation::Add),
        }
    }
    ret
}

fn read_input(input: &str) -> Vec<Equation> {
    let mut ret = vec![];
    for line in input.lines() {
        let colon : Vec<&str> = line.split(":").collect();
        let test = colon[0].parse().unwrap();
        let nums = colon[1].split_whitespace().map(|x|{ x.parse().unwrap()}).collect();
        ret.push(Equation {test: test, nums: nums});
    }
    // println!("{ret:?}");
    ret
}

fn check_op(eq: &Equation, ops: &Vec<Operation>) -> bool {
    assert!(eq.nums.len() == ops.len() + 1);
    let mut out: i64 = eq.nums[0];
    // println!("Checking {eq:?} and {ops:?}");
    for i in 0..ops.len() {
        match ops[i] {
            Operation::Add => out += eq.nums[i+1],
            Operation::Mult => out *= eq.nums[i+1],
        }
    }
    out == eq.test
}

fn part1(input: &str) -> i64 {
    let eqs = read_input(input);
    let mut ret = 0;
    for eq in eqs {
        let mut ops = vec![Operation::Add; eq.nums.len() - 1];
        let end = vec![Operation::Mult; eq.nums.len() - 1];
        let mut sat = false;
        if check_op(&eq, &ops) { sat = true; }
        while !sat {
            ops = next_ops(ops);
            if check_op(&eq, &ops) { sat = true; break }
            if ops == end { break }
        }
        if sat { ret += eq.test }
    }
    ret
}

fn next_ops2(ops: Vec<Operation2>) -> Vec<Operation2> {
    let mut switched = false;
    let mut ret = vec![];
    for op in ops {
        if switched { ret.push(op); continue; }
        match op {
            Operation2::Add => { ret.push(Operation2::Mult); switched = true },
            Operation2::Mult => { ret.push(Concat); switched = true },
            Concat => ret.push(Operation2::Add),
        }
    }
    ret
}

fn check_op2(eq: &Equation, ops: &Vec<Operation2>) -> bool {
    assert!(eq.nums.len() == ops.len() + 1);
    // oof, read the question wrong...
    // println!("Checking {eq:?} and {ops:?}");
    // let mut num2 = vec![];
    // num2.push(eq.nums[0]);
    // for i in 0..ops.len() {
    //     if ops[i] == Concat {
    //         let prev = num2.pop().unwrap();
    //         let next = eq.nums[i+1];
    //         let Some(shifted) = prev.checked_mul(10i64.pow(next.ilog10() + 1)) else { return false };
    //         num2.push(shifted + next);
    //     } else {
    //         num2.push(eq.nums[i+1]);
    //     }
    // }

    let mut out: i64 = eq.nums[0];
    for i in 0..ops.len() {
        let next = eq.nums[i+1];
        match ops[i] {
            Operation2::Add => {
                let Some(sum) = out.checked_add(next) else {return false}; out = sum },
            Operation2::Mult => {
                let Some(prod) = out.checked_mul(next) else {return false}; out = prod; },
            Concat => {
                let Some(shifted) = out.checked_mul(10i64.pow(next.ilog10() + 1)) else { return false };
                out = shifted + next;
            },
        }
    }
    // println!("Checking {eq:?} and {ops:?}");
    out == eq.test
}

fn part2(input: &str) -> i64 {
    let eqs = read_input(input);
    let mut ret = 0;
    for eq in eqs {
        let mut ops = vec![Operation2::Add; eq.nums.len() - 1];
        let end = vec![Concat; eq.nums.len() - 1];
        let mut sat = false;
        if check_op2(&eq, &ops) { sat = true; }
        while !sat {
            ops = next_ops2(ops);
            if check_op2(&eq, &ops) { sat = true; break }
            if ops == end { break }
        }
        if sat { ret += eq.test }
    }
    ret
}
