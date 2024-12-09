use std::fs::read_to_string;
use std::collections::HashMap;
use itertools::Itertools;
use gcd::Gcd;

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    locs: HashMap<char, Vec<(i32, i32)>>,
    w: i32,
    h: i32,
}

fn read_input(input: &str) -> Map {
    let mut m = Map { grid: vec![], locs: HashMap::new(), w: 0, h:0 };
    for (y, line) in input.lines().enumerate() {
        m.grid.push(line.chars().collect());
        for (x, c) in line.chars().enumerate() {
            if c == '.' { continue }
            m.locs.entry(c)
                .or_insert(vec![])
                .push((x as i32, y as i32));
        }
    }
    
    m.h = m.grid.len() as i32;
    m.w = m.grid[0].len() as i32;
    println!("Map: {m:?}");
    m
}

pub fn solve() {
    let input = read_to_string("input/8.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}



fn part1(input: &str) -> i32 {
    let m = read_input(input);
    let mut alocs = HashMap::new();
    for (c, cords) in m.locs.iter() {
        for pair in (*cords).iter().combinations(2) {
            
            println!("{c} pair: {pair:?}");
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            
            let x3 = (x1 - x2) + x1;
            let y3 = (y1 - y2) + y1;

            let x4 = (x2 - x1) + x2;
            let y4 = (y2 - y1) + y2;

            if x3 < m.w && x3 >= 0 && y3 < m.h && y3 >= 0 {
                alocs.entry((x3,y3)).or_insert(vec![]).push(c);
            }

            if x4 < m.w && x4 >= 0 && y4 < m.h && y4 >= 0 {
                alocs.entry((x4,y4)).or_insert(vec![]).push(c);
            }
        }
    }

    alocs.len() as i32
}

fn part2(input: &str) -> i32 {
    let m = read_input(input);
    let mut alocs = HashMap::new();
    for (c, cords) in m.locs.iter() {
        for pair in (*cords).iter().combinations(2) {
            
            // println!("{c} pair: {pair:?}");
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            
            let g = ((x1 - x2).abs() as u32).gcd((y1 - y2).abs() as u32) as i32;
            // println!("{x1},{y1} {x2},{y2} gcd: {g}");
            let xstep = if g == 1 { x1 - x2 } else {(x1 - x2) / g};
            let ystep = if g == 1 { y1 - y2 } else {(y1 - y2) / g};
                
            // println!("xstep: {xstep} ystep: {ystep}");
            let mut x3 = *x1;
            let mut y3 = *y1;

            while x3 < m.w && x3 >= 0 && y3 < m.h && y3 >= 0 {
                alocs.entry((x3,y3)).or_insert(vec![]).push(c);
                x3 += xstep;
                y3 += ystep;
            }

            let mut x3 = *x1;
            let mut y3 = *y1;

            while x3 < m.w && x3 >= 0 && y3 < m.h && y3 >= 0 {
                alocs.entry((x3,y3)).or_insert(vec![]).push(c);
                x3 -= xstep;
                y3 -= ystep;
            }

        }
    }

    println!("alocs: {alocs:?}");
    alocs.len() as i32
}
