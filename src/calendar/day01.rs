use std::{collections::HashMap, fs::read_to_string};

pub fn solve() {
    let input = read_to_string("input/1.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    for line in input.lines() {
        // println!("line {}", line);
        
        let split : Vec<&str> = line.split_whitespace().collect(); 
        list1.push(split[0].parse().unwrap());
        list2.push(split[1].parse().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut total = 0;
    for (i, num) in list1.iter().enumerate() {
        if list2[i] < *num {
            total += *num - list2[i];
        } else {
            total += list2[i] - *num;
        }
        // println!("{} {} {}", *num, list2[i], total);
    }
    
    return total;
}

fn part2(input: &str) -> i32 {
    let mut list1: Vec<i32> = vec![];
    let mut list2: HashMap<i32,i32> = HashMap::new();
    for line in input.lines() {
        // println!("line {}", line);
        
        let split : Vec<&str> = line.split_whitespace().collect(); 
        list1.push(split[0].parse().unwrap());
        let num2 : i32 = split[1].parse().unwrap();
        let count = list2.entry(num2).or_insert(0); 
        *count += 1;
        // list2.insert(num2, *count+1);
    }
    // list1.sort();
    // list2.sort();
    let mut total = 0;
    for num in list1 {
        let freq = list2.get(&num).copied().unwrap_or(0);
        total += num * freq;
        // println!("{} {} {}", num, freq, total);
    }
    
    return total;
}
