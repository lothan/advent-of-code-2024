use std::{backtrace, collections::VecDeque, fs::read_to_string};
// use std::collections::HashMap;
use itertools::Itertools;

struct Garden {
    map: Vec<Vec<char>>,
    // id => (char, cords)
    // regions: HashMap<usize, (char, Vec<(usize, usize)>)>,
    h: usize,
    w: usize,
}

struct Region {
    c: char,
    cords: Vec<(usize, usize)>,
    border: Vec<(usize, usize)>,
}

fn read_input(input: &str) -> Garden {
    let mut map : Vec<Vec<char>> = vec![];
    // let mut regions = HashMap::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }
    let h = map.len();
    let w = map[0].len();

    Garden { map, h, w }
}

// id => (char, cords)
// regions: HashMap<usize, (char, Vec<(usize, usize)>)>,

// walk(map, seen, x, y) -> area * permi

// while queue is not empty 
fn walk(new: (usize, usize), seen: &mut Vec<Vec<bool>>, g: &Garden) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(new);
    let c = g.map[new.1][new.0];
    let mut area = 0;
    let mut perm = 0;
    while ! queue.is_empty() {
        let (x,y) = queue.pop_front().unwrap();
        if seen[y][x] { continue }
        area += 1;
        perm += 4;
        
        seen[y][x] = true;
        if x > 0 && g.map[y][x-1] == c {
            if !seen[y][x-1] {
                queue.push_back((x-1, y));
            }
            perm -= 1;
        }
        if y > 0 && g.map[y-1][x] == c {
            if !seen[y-1][x] {
                queue.push_back((x, y-1));
            }
            perm -= 1;
        }
        if x < g.w - 1 && g.map[y][x+1] == c {
            if !seen[y][x+1] {
                queue.push_back((x+1, y));
            }
            perm -= 1;
        }
        if y < g.h - 1 && g.map[y+1][x] == c {
            if !seen[y+1][x] {
                queue.push_back((x, y+1));
            }
            perm -= 1;
        }
        // println!("Walking {x},{y} perm={perm} area={area}");
    }
    perm * area
}

pub fn solve() {
    let input = read_to_string("input/12.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let g = read_input(input);
    let mut seen = vec![];
    for _ in 0..g.h {
        seen.push(vec![false; g.w]);
    }
    
    let mut ret = 0;
    for y in 0..g.h {
        for x in 0..g.w {
            if seen[y][x] { continue }
            let price = walk((x,y), &mut seen, &g);
            // println!("Region {}, price {}", g.map[y][x], price);
            ret += price;
        }
    }
    ret
}

// return border map
fn walk2(new: (usize, usize), seen: &mut Vec<Vec<bool>>, g: &Garden) -> usize {
    // border map with different sides on east and west
    // not "running" east west
    let mut ewbord = vec![];
    let mut nsbord = vec![];
    for _ in 0..(g.h+2) {
        ewbord.push(vec!['.'; g.w+2]);
        nsbord.push(vec!['.'; g.w+2]);
    }
    // for _ in 0..(g.h+1) {
    // }

    let mut area = 0;
    let mut queue = VecDeque::new();
    queue.push_back(new);
    let c = g.map[new.1][new.0];
    while ! queue.is_empty() {
        let (x,y) = queue.pop_front().unwrap();
        if seen[y][x] { continue }
        
        seen[y][x] = true;
        area += 1;
        if x > 0 && g.map[y][x-1] == c {
            queue.push_back((x-1, y));
        } else {
            ewbord[y][x] = '<';
        }
        if y > 0 && g.map[y-1][x] == c {
            queue.push_back((x, y-1));
        } else {
            nsbord[y][x] = '^';
        }
        if x < g.w - 1 && g.map[y][x+1] == c {
            queue.push_back((x+1, y));
        } else {
            ewbord[y][x+1] = '>';
        }
        if y < g.h - 1 && g.map[y+1][x] == c {
            queue.push_back((x, y+1));
        } else {
            nsbord[y+1][x] = 'v';
        }
        // println!("Walking {x},{y} perm={perm} area={area}");
    }

    /*
    println!("EW Border:");
    for line in &ewbord {
        for b in line {
            print!("{}", *b);
        }
        println!("");
    }
    println!("NS Border:");
    for line in &nsbord {
        for b in line {
            print!("{}", *b);
        }
        println!("");
    }
    */

    let mut sides = 0;
    // count horizonal / ns borders
    let mut edging = '.';
    for y in 0..(g.h+2) {
        for x in 0..(g.w+2) {
            if edging != nsbord[y][x] && nsbord[y][x] != '.' { sides += 1 }
            edging = nsbord[y][x];
        }
    }
    // println!("horz sides = {sides}");

    // count vertical / ew
    let mut edging = '.';
    for y in 0..(g.h+2) {
        for x in 0..(g.w+2) {
            if (edging != nsbord[y][x]) && nsbord[y][x] != '.' {
                sides += 1;
            }
            edging = nsbord[y][x];
        }
    }
    // println!("vert sides = {sides}");
    
    area * sides
}
fn part2(input: &str) -> usize {
    let g = read_input(input);
    let mut seen = vec![];
    for _ in 0..g.h {
        seen.push(vec![false; g.w]);
    }
    
    let mut ret = 0;
    for y in 0..g.h {
        for x in 0..g.w {
            if seen[y][x] { continue }
            let price = walk2((x,y), &mut seen, &g);
            // println!("Region {}, price {}", g.map[y][x], price);
            ret += price;
        }
    }
    ret
}
