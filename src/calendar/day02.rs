use std::fs::read_to_string;

pub fn solve() {
    let input = read_to_string("input/2.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut reports: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let report : Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(); 
        reports.push(report);
    }

    let mut count : i32 = 0;
    for report in reports {
        let mut prev : i32 = report[0];
        let decreasing : bool = report[1] < report[0];
        let mut safe : bool = true;
        for level in &report[1..] {
            if (decreasing &&
                ((*level >= prev) ||  (*level < prev - 3))) ||
                (! decreasing &&
                 ((*level <= prev) || (*level > prev + 3))) {
                    // println!("level {} prev {} decreasing {}", level, prev, decreasing);
                    safe = false;
                    break;
                }
            prev = *level;
        }
        if safe {
            count += 1;
        }
    }
    return count;
}

// return index of first problem
fn check_safe(report: &Vec<i32>) -> Option<usize> {
    let mut prev : i32 = report[0];
    let decreasing : bool = report[report.len() - 1] < report[0];
    let mut safe : bool = true;
    let mut badidx : usize = 0;
    for (idx, level) in report[1..].iter().enumerate() {
        if (decreasing &&
            ((*level >= prev) ||  (*level < prev - 3))) ||
            (! decreasing &&
             ((*level <= prev) || (*level > prev + 3))) {
                // println!("level {} prev {} decreasing {}", level, prev, decreasing);
                safe = false;
                badidx = idx;
                break;
            }
        prev = *level;
    }
    
    if safe {
        return None;
    } else {
        return Some(badidx);
    }
                    
}

fn part2(input: &str) -> i32 {
    let mut reports: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let report : Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(); 
        reports.push(report);
    }

    let mut count : i32 = 0;
    for report in reports {
        let badidx = check_safe(&report);
        match badidx {
            None => count += 1,
            Some(idx) => { let mut first = report.clone();
                           let mut second = report.clone();
                           first.remove(idx);
                           second.remove(idx+1);
                           if check_safe(&first).is_none() ||
                           check_safe(&second).is_none() {
                               count += 1;
                           }
            }
        }
    }
    // println!("{:?}", reports);
    return count;
}
