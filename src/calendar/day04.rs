use std::fs::read_to_string;

pub fn solve() {
    let input = read_to_string("input/4.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut search : Vec<Vec<_>> = vec![];
    for line in input.lines() {
        let mut row : Vec<_> = line.split("").skip(1).collect();
        row.pop();
        search.push(row);
    }
    // println!("{:?}", search);
    let mut count = 0;
    for (i, row) in search.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == "X" {
                if check_n(&search, i, j) { count += 1; }
                if check_e(&search, i, j) { count += 1; }
                if check_s(&search, i, j) { count += 1; }
                if check_w(&search, i, j) { count += 1; }
                if check_ne(&search, i, j) { count += 1; }
                if check_se(&search, i, j) { count += 1; }
                if check_sw(&search, i, j) { count += 1; }
                if check_nw(&search, i, j) { count += 1; }
            }
        }
    }
    count
}

fn check_n(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    if i < 3 { return false; }
    return search[i-1][j] == "M" && search[i-2][j] == "A" && search[i-3][j] == "S"
}

fn check_s(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    if i > search.len() - 4 { return false; }
    return search[i+1][j] == "M" && search[i+2][j] == "A" && search[i+3][j] == "S"
}

fn check_e(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    if j > search[0].len() - 4 { return false; }
    return search[i][j+1] == "M" && search[i][j+2] == "A" && search[i][j+3] == "S"
}

fn check_w(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    if j < 3 { return false; }
    return search[i][j-1] == "M" && search[i][j-2] == "A" && search[i][j-3] == "S"
}

fn check_nw(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    if j < 3 || i < 3 { return false; }
    return search[i-1][j-1] == "M" && search[i-2][j-2] == "A" && search[i-3][j-3] == "S"
}

fn check_ne(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    if j > search[0].len() - 4 || i < 3 { return false; }
    return search[i-1][j+1] == "M" && search[i-2][j+2] == "A" && search[i-3][j+3] == "S"
}

fn check_se(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    if j > search[0].len() - 4 || i > search.len() - 4 { return false; }
    return search[i+1][j+1] == "M" && search[i+2][j+2] == "A" && search[i+3][j+3] == "S"
}

fn check_sw(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    if j < 3 || i > search.len() - 4 { return false; }
    return search[i+1][j-1] == "M" && search[i+2][j-2] == "A" && search[i+3][j-3] == "S"
}

fn check_xmas(search: &Vec<Vec<&str>>, i: usize, j: usize) -> bool {
    // println!("check_xmas: {} {}", j, i);
    if search[i+1][j+1] != "A" { return false; }
    if !((search[i][j] == "M" && search[i+2][j+2] == "S") ||
        (search[i][j] == "S" && search[i+2][j+2] == "M")) { return false; }
    return (search[i][j] == search[i][j+2] &&
            search[i+2][j] == search[i+2][j+2]) ||
        (search[i][j] == search[i+2][j] &&
         search[i][j+2] == search[i+2][j+2])
}

fn part2(input: &str) -> i64 {
    let mut search : Vec<Vec<_>> = vec![];
    for line in input.lines() {
        let mut row : Vec<_> = line.split("").skip(1).collect();
        row.pop();
        search.push(row);
    }
    // println!("{:?}", search);
    let mut count = 0;
    for i in 0..search.len() - 2 {
        for j in 0..search[0].len() - 2 {
            if check_xmas(&search, i, j) { count += 1; } 
        }
    }
    count
}
