use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug)]
struct Topo {
    map: Vec<Vec<usize>>,
    heads: Vec<(usize, usize)>,
    h: usize,
    w: usize,
}

pub fn solve() {
    let input = read_to_string("input/10.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn read_input(input: &str) -> Topo {
    let mut map = vec![];
    let mut heads = vec![];
    for (i, line) in input.lines().enumerate() {
        map.push(vec![]);
        for (j, c) in line.chars().enumerate() {
            map[i].push((c as usize) - 48usize);
            if c == '0' { heads.push((j, i)); }
        }
    }
    return Topo { map: map.clone(), heads, h: map.len(), w: map[0].len() }
}

fn part1(input: &str) -> i32 {
    let t = read_input(input);
    // println!("{t:?}");
    let mut paths : Vec<Vec<(usize, usize)>> = t.heads.clone()
        .into_iter().map(|x| vec![x,]).collect();
    // println!("paths: {} {paths:?}", paths.len());
    for i in 1..10 {
        // println!("Finding next {i}'s");
        let mut nexts : Vec<Vec<(usize, usize)>> = vec![];
        for p in paths {
            let (x, y) = p[i-1];
            // let x = cord.0 as usize;
            // let y = cord.1 as usize;
            if x < t.w - 1 && t.map[y][x+1] == i {
                let mut new = p.clone();
                new.push((x+1, y));
                nexts.push(new);
            }
            if x > 0 && t.map[y][x-1] == i {
                let mut new = p.clone();
                new.push((x-1, y));
                nexts.push(new);
            }
            if y < t.h - 1 && t.map[y+1][x] == i {
                let mut new = p.clone();
                new.push((x, y+1));
                nexts.push(new);
            }
            if y > 0 && t.map[y-1][x] == i {
                let mut new = p.clone();
                new.push((x, y-1));
                nexts.push(new);
            }
        }
        // for n in &nexts {
        //     println!("{n:?}");
        // }
        paths = nexts;
    }

    let mut startend = HashMap::new();
    for p in paths {
        *startend.entry((p[0], p[9])).or_insert(0) += 1;
    }
    // println!("startend: {startend:?}");
    let mut ret = 0;
    for (_, num) in startend {
        ret += 1;
    }
    
    ret
}

fn part2(input: &str) -> i32 {
    let t = read_input(input);
    // println!("{t:?}");
    let mut paths : Vec<Vec<(usize, usize)>> = t.heads.clone()
        .into_iter().map(|x| vec![x,]).collect();
    // println!("paths: {} {paths:?}", paths.len());
    for i in 1..10 {
        // println!("Finding next {i}'s");
        let mut nexts : Vec<Vec<(usize, usize)>> = vec![];
        for p in paths {
            let (x, y) = p[i-1];
            // let x = cord.0 as usize;
            // let y = cord.1 as usize;
            if x < t.w - 1 && t.map[y][x+1] == i {
                let mut new = p.clone();
                new.push((x+1, y));
                nexts.push(new);
            }
            if x > 0 && t.map[y][x-1] == i {
                let mut new = p.clone();
                new.push((x-1, y));
                nexts.push(new);
            }
            if y < t.h - 1 && t.map[y+1][x] == i {
                let mut new = p.clone();
                new.push((x, y+1));
                nexts.push(new);
            }
            if y > 0 && t.map[y-1][x] == i {
                let mut new = p.clone();
                new.push((x, y-1));
                nexts.push(new);
            }
        }
        paths = nexts;
    }
    paths.len() as i32
}
