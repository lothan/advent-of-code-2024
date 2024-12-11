use std::fs::read_to_string;
use std::collections::HashMap;

pub fn solve() {
    let input = read_to_string("input/11.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn read_input(input: &str) -> Vec<u64> {
    input.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn part1_iter(stones: Vec<u64>) -> Vec<u64> {
    let mut ret = vec![];
    for stone in stones {
        if stone == 0 { ret.push(1); continue }
        if stone.ilog10() % 2 == 1 {
            let b = 10u64.pow((stone.ilog10() + 1) / 2);
            ret.push(stone / b);
            ret.push(stone % b);
            continue;
        }
        ret.push(stone*2024);
    }
    ret
}

fn part1(input: &str) -> i32 {
    let mut stones = read_input(input);
    for i in 0..25 {
        // println!("{i}: {stones:?}");
        stones = part1_iter(stones);
    }
    stones.len() as i32
}


fn part2_iter(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut ret = HashMap::new();
    for (stone, count) in stones.iter() {
        if *count == 0 { continue } 
        if *stone == 0 {
            *ret.entry(1).or_insert(0) += *count;
            continue
        }
        if stone.ilog10() % 2 == 1 {
            let b = 10u64.pow((stone.ilog10() + 1) / 2);
            *ret.entry(stone / b).or_insert(0) += *count;
            *ret.entry(stone % b).or_insert(0) += *count;
            continue;
        }
        *ret.entry(stone * 2024).or_insert(0) += *count;
    }
    ret
}

fn part2(input: &str) -> u64 {
    let svec = read_input(input);
    let mut stones = HashMap::new();
    for stone in svec {
        *stones.entry(stone).or_insert(0) += 1;
    }
    // println!("{stones:?}");
    
    for _ in 0..75 {
        let next = part2_iter(stones);
        stones = next;
        // println!("iter {i}");
    }

    let mut ret = 0;
    for (_, count) in stones {
        ret += count;
    }
    ret
}
