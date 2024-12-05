use std::fs::read_to_string;
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let input = read_to_string("input/5.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn check_rule(rule: (i32, i32), update: &Vec<i32>) -> bool {
    let (x,y) = rule;
    let (mut seenx, mut seeny) = (false, false);

    for page in update {
        if *page == x {
            seenx = true;
            if seeny { return false; }
        }
        if *page == y {
            seeny = true;
            if seenx { return true; }
        }
    }

    // return (! seenx) && (! seeny);
    true
}

fn part1(input: &str) -> i32 {
    let mut rules : Vec<(i32, i32)> = vec![];
    let mut pages : Vec<Vec<i32>> = vec![];
    let mut middle : bool = false;
    for line in input.lines() {
        if line == "" { middle = true; continue }
        if middle {
            pages.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        } else {
            rules.push(line.split("|")
                       .map(|x| x.parse::<i32>().unwrap())
                       .collect_tuple()
                       .unwrap());
        }
    }
    // println!("{:?}", pages);

    let mut count = 0;
    for update in pages {
        let mut correct = true;
        for rule in &rules {
            if ! check_rule(*rule, &update) { correct = false; break; }
        }
        if correct {
            let mid = update[update.len() / 2];
            count += mid;
        }
    }
    count
}

// assumes every single rule exists, so you can count the rules
// and infer what index it is based on the number of rules
fn correct_pages(update: &Vec<i32>, rules: &Vec<(i32,i32)>) -> Vec<i32> {
    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
    // println!("\nCalling correct_pages with: {:?}", update);
    for (x, y) in rules {
        if update.contains(y) && update.contains(x) {
            before.entry(*y)
                .or_insert_with(Vec::new)
                .push(*x);
        }
    }
    let mut ret : Vec<i32> = vec![0; update.len()];
    for (y, xs) in before.into_iter() {
        ret[xs.len()] = y;
        if xs.len() == 1 { ret[0] = xs[0]; }
    }
    // println!("Correct order: {:?}", ret);
    return ret;
}

fn part2(input: &str) -> i32 {
    let mut rules : Vec<(i32, i32)> = vec![];
    let mut pages : Vec<Vec<i32>> = vec![];
    let mut middle : bool = false;
    for line in input.lines() {
        if line == "" { middle = true; continue }
        if middle {
            pages.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        } else {
            rules.push(line.split("|")
                       .map(|x| x.parse::<i32>().unwrap())
                       .collect_tuple()
                       .unwrap());
        }
    }

    let mut count = 0;
    for update in pages {
        let mut correct = true;
        for rule in &rules {
            if ! check_rule(*rule, &update) { correct = false; break; }
        }
        if ! correct {
            let corrected = correct_pages(&update, &rules);
            count += corrected[corrected.len() / 2];
        }
    }
    count
}
