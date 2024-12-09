use std::fs::read_to_string;
use Direction::*;
// use std::collections::HashSet;

pub fn solve() {
    let input = read_to_string("input/6.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Clone, PartialEq)]
struct Guard {
    pos: (usize, usize), 
    dir: Direction,
}

fn part1(input: &str) -> i32 {
    let mut map : Vec<Vec<char>> = vec![];
    let mut g = Guard {
        pos: (0, 0),
        dir: North,
    };

    for (i, line) in input.lines().enumerate() {
        let mut row : Vec<&str> = line.split("").skip(1).collect();
        row.pop();
        let row = row.iter().enumerate()
            .map(|(j, x)|
                 if *x == "^" { g.pos = (j, i); 'X' } else {
                     x.chars().nth(0).unwrap()})
            .collect();
        map.push(row);
    }
    // println!("{map:?}");

    while move_guard(&mut map, &mut g) {
        // println!("Iteration {i}");
        // print_map(&map);
    }

    let mut count = 0;
    for row in map {
        for cell in row {
            if cell == 'X' {
                count += 1;
            }
        }
    }
    count
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        let l: String = line.iter().collect();
        println!("{}", l);
    }
}

fn move_guard(map: &mut Vec<Vec<char>>, g: &mut Guard) -> bool {
    let prevpos = g.pos;
    let mut nextpos = (0,0);
    match (&g.dir, prevpos) {
        (North, (x, y)) => nextpos = (x, y - 1),
        (South, (x, y)) => nextpos = (x, y + 1),
        (East, (x, y)) => nextpos = (x + 1, y),
        (West, (x, y)) => nextpos = (x - 1, y),
    }

    if nextpos.0 >= map[0].len() || nextpos.1 >= map[1].len() {
        // because we are using usize, this will underflow and
        // work for when it falls off the West or North edge too
        return false;
        }

    if map[nextpos.1][nextpos.0] == '#' {
        g.dir = match g.dir {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        // println!("Turned {:?}", g.dir);
        // print_map(&map);
        move_guard(map, g);
    } else {
        map[nextpos.1][nextpos.0] = 'X';
        g.pos = nextpos;
    }
    true
}

// painfully slow... could you find loops just by looking at the map?
// and not even needing to run the guard walks?
// or even just walk the path and save it. At every step, put a obstacle 
// in front and clone the guard, see if that one ends up at a previous position. Should be faster. 
fn part2(input: &str) -> i32 {
    let mut map : Vec<Vec<char>> = vec![];
    let mut g = Guard {
        pos: (0, 0),
        dir: North,
    };

    for (i, line) in input.lines().enumerate() {
        let mut row : Vec<&str> = line.split("").skip(1).collect();
        row.pop();
        let row = row.iter().enumerate()
            .map(|(j, x)|
                 if *x == "^" { g.pos = (j, i); '.' } else {
                     x.chars().nth(0).unwrap()})
            .collect();
        map.push(row);
    }
    println!("{map:?}");

    // let mut obs : HashSet<(usize, usize)> = HashSet::new();
    let mut initmap = map.clone();
    let mut initg = g.clone();
    while move_guard(&mut initmap, &mut initg) {
        // println!("Iteration {i}");
        // print_map(&map);
    }

    let mut initpath : Vec<(usize, usize)> = vec![];
    for (j, row) in initmap.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            if *cell == 'X' {
                initpath.push((i,j));
            }
        }
    }

    // print_map(&initmap);
    // kinda dumb, iterate over all map locations
    let mut count = 0;
    for (i,j) in initpath {
        if map[j][i] == '#' || g.pos == (i, j) { continue; }
        println!("Trying with new obstacle at location ({i},{j})");
        let mut newmap = map.clone();
        newmap[j][i] = '#';
        // print_map(&newmap);
        if move_guard2(&mut newmap, &mut g) {
            count += 1;
        }
    }

    count
}

fn move_guard2(map: &Vec<Vec<char>>, start: &Guard) -> bool { 
    let mut path : Vec<Guard> = vec![];
    let mut g = start.clone();
    path.push(start.clone());

    loop {
        let prevpos = g.pos;
        let mut nextpos = (0,0);
        match (&g.dir, prevpos) {
            (North, (x, y)) => {if y == 0 { return false } ; nextpos = (x, y - 1)},
            (South, (x, y)) => nextpos = (x, y + 1),
            (East, (x, y)) => nextpos = (x + 1, y),
            (West, (x, y)) => {if x == 0 { return false }; nextpos = (x - 1, y)},
        }

        if nextpos.0 >= map[0].len() || nextpos.1 >= map[1].len() {
            return false;
        }

      

        if map[nextpos.1][nextpos.0] == '#' {
            g.dir = match g.dir {
                North => East,
                East => South,
                South => West,
                West => North,
            };
            // println!("Turned: {g:?}");
            // println!("Path: {path:?}");
                
        } else {
            g.pos = nextpos;
            for loc in &path {
                if g == *loc {
                    println!("Found loop!");
                    return true;
                }
            }
            path.push(g.clone());
            // map[nextpos.1][nextpos.0] = 'X';
        }
    }
}
