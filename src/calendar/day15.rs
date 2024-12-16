use std::fs::read_to_string;
use std::collections::{VecDeque,HashSet};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point (i32, i32);

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<char>>,
    walls: HashSet<Point>,
    boxes: HashSet<Point>,
    robot: Point,
    moves: VecDeque<char>,
    h: i32, w: i32,
}

impl Point {
    fn add(&self, x: &Point) -> Point {
        Point(self.0 + x.0, self.1 + x.1)
    }
}

impl Warehouse {
    fn go(&mut self) {
        let dir = self.moves.pop_front().unwrap();
        let mut diff = Point (0, 0);
        match dir {
            '^' => diff = Point(0, -1),
            '>' => diff = Point(1,  0),
            'v' => diff = Point(0,  1),
            '<' => diff = Point(-1, 0),
            _   => (),
        }
        let new = self.robot.add(&diff);
        if self.walls.contains(&new) { return }
        if ! (self.walls.contains(&new) || self.boxes.contains(&new)) {
            self.robot = new;
            return
        }
        let mut empty = new.clone();
        while self.boxes.contains(&empty) {
            empty = empty.add(&diff);
        }
        if ! self.walls.contains(&empty) {
            self.boxes.remove(&new);
            self.boxes.insert(empty);
            self.robot = new;
        }
    }

    fn go2(&mut self) {
        let dir = self.moves.pop_front().unwrap();
        let mut diff = Point (0, 0);
        match dir {
            '^' => diff = Point(0, -1),
            '>' => diff = Point(1,  0),
            'v' => diff = Point(0,  1),
            '<' => diff = Point(-1, 0),
            _   => (),
        }
        let new = self.robot.add(&diff);
        if self.walls.contains(&new) { return }
        if self.loc_empty(&new) {
            self.robot = new;
            return
        }
        // so new must have a box
        let boxcord = self.getboxcord(&new);
        if self.can_move_box(&boxcord, &diff) {
            self.move_box(&boxcord, &diff);
            self.robot = new.clone();
        }
    }

    fn loc_empty(&self, cord: &Point) -> bool {
        let boxdiff = Point(-1, 0);
        ! (self.walls.contains(&cord) ||
           self.boxes.contains(&cord) ||
           self.boxes.contains(&cord.add(&boxdiff)))
    }

    fn getboxcord(&self, cord: &Point) -> Point {
        let boxdiff = Point(-1, 0);
        if self.boxes.contains(&cord) { cord.clone() } else { cord.add(&boxdiff) }
    }
    
    fn isboxcord(&self, cord: &Point) -> bool {
        let boxdiff = Point(-1, 0);
        self.boxes.contains(&cord) || self.boxes.contains(&cord.add(&boxdiff))
    }
    
    fn can_move_box(&self, boxcord: &Point, diff: &Point) -> bool {
        let mut new = boxcord.add(diff);
        if *diff == Point(-1,0) || *diff == Point(1,0) {
            if *diff == Point(1,0) { new = new.add(diff); }
            if self.loc_empty(&new) { return true }
            if self.walls.contains(&new) { return false }
            return self.can_move_box(&self.getboxcord(&new), diff)
        }
        if *diff == Point(0,-1) || *diff == Point(0,1) {
            let new2 = new.add(&Point(1, 0));

            if self.loc_empty(&new) && self.loc_empty(&new2) { return true }
            if self.walls.contains(&new) || self.walls.contains(&new2) { return false }
            if self.isboxcord(&new) && self.isboxcord(&new2) {
                if self.getboxcord(&new) == self.getboxcord(&new2) {
                    return self.can_move_box(&new, diff);
                }

                return self.can_move_box(&self.getboxcord(&new), diff) && 
                    self.can_move_box(&self.getboxcord(&new2), diff)
            }

            if self.isboxcord(&new) {
                return self.can_move_box(&self.getboxcord(&new), diff);
            }
            if self.isboxcord(&new2) {
                return self.can_move_box(&self.getboxcord(&new2), diff);
            }
            panic!("can_move_box impossible 2");
        }
        panic!("can_move_box impossible!");
    }

    fn move_box(&mut self, boxcord: &Point, diff: &Point) {
        let new = boxcord.add(diff);
        if *diff == Point(-1,0) || *diff == Point(1,0) {
            // jank
            let empty = if (*diff == Point(-1,0)) { new.clone() } else {new.add(diff)};
            if self.loc_empty(&empty) {
                self.boxes.remove(&boxcord);
                self.boxes.insert(new.clone());
                return
            }
            if self.walls.contains(&empty) { return }
            self.move_box(&self.getboxcord(&empty), diff);
            self.move_box(boxcord, diff);
            return
        }
        if *diff == Point(0,-1) || *diff == Point(0,1) {
            let new2 = new.add(&Point(1, 0));

            if self.loc_empty(&new) && self.loc_empty(&new2) {
                self.boxes.remove(&boxcord);
                self.boxes.insert(new.clone());
                return
            }
            if self.walls.contains(&new) || self.walls.contains(&new2) { return }

            if self.isboxcord(&new) && self.isboxcord(&new2) {
                if self.getboxcord(&new) == self.getboxcord(&new2) {
                    // println!("Just moving the one!");
                    self.move_box(&new, diff);
                    self.move_box(boxcord, diff);
                    return;
                }
                self.move_box(&self.getboxcord(&new), diff);
                self.move_box(&self.getboxcord(&new2), diff);
                self.move_box(boxcord, diff);
                return
            }

            if self.isboxcord(&new) {
                self.move_box(&self.getboxcord(&new), diff);
                self.move_box(boxcord, diff);
                return
            }
            if self.isboxcord(&new2) {
                self.move_box(&self.getboxcord(&new2), diff);
                self.move_box(boxcord, diff);
                return
            }
        }
        panic!("move_box impossible!");
    }
    
    fn sum_box_gps(&self) -> i32 {
        let mut ret = 0;
        for Point(x,y) in self.boxes.iter() {
            ret += y * 100 + x;
        }
        ret
    }

    fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.walls.contains(&Point(x,y)) { print!("#"); }
                else if self.boxes.contains(&Point(x,y)) { print!("O"); }
                else if self.robot == Point(x,y) { print!("@"); }
                else { print!("."); }
            }
            println!();
        }
    }

    fn print2(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.walls.contains(&Point(x,y)) { print!("#"); }
                else if self.boxes.contains(&Point(x,y)) { print!("["); }
                else if self.robot == Point(x,y) { print!("@"); }
                else if self.boxes.contains(&Point(x-1,y)) { print!("]"); }
                else { print!("."); }
            }
            println!();
        }
    }
}

fn read_input(input: &str) -> Warehouse {
    let mut map : Vec<Vec<char>> = vec![];
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = Point (0, 0);
    let mut moves = VecDeque::new();

    let mut mapping = true;
    for (y, line) in input.lines().enumerate() {
        if line == "" { mapping = false; continue }
        if mapping {
            map.push(line.chars().collect());
            for (x, c) in line.chars().enumerate() {
                let p = Point(x as i32, y as i32);
                match c {
                    '#' => {walls.insert(p);},
                    '@' => robot = p,
                    'O' => {boxes.insert(p);},
                    _   => (),
                }
            }
        } else {
            for c in line.chars() {
                moves.push_back(c);
            }
        }
    }

    let h = map.len() as i32;
    let w = map[0].len() as i32;
    Warehouse { map, walls, boxes, robot, moves, h, w}
}

fn read_input2(input: &str) -> Warehouse {
    let mut map : Vec<Vec<char>> = vec![];
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = Point (0, 0);
    let mut moves = VecDeque::new();

    let mut mapping = true;
    let mut h = 0;
    for (y, line) in input.lines().enumerate() {

        if line == "" { mapping = false; continue }

        if mapping {
            h += 1;
            for (x, c) in line.chars().enumerate() {
                let p1 = Point(2*x as i32, y as i32);
                let p2 = Point((2*x + 1) as i32, y as i32);
                match c {
                    '#' => {walls.insert(p1); walls.insert(p2);},
                    '@' => robot = p1,
                    'O' => {boxes.insert(p1);},
                    _   => (),
                }
            }
        } else {
            for c in line.chars() {
                moves.push_back(c);
            }
        }
    }
    let line = input.lines().next().unwrap();
    let w = (2*line.len()) as i32;
    Warehouse { map, walls, boxes, robot, moves, h, w}
}

pub fn solve() {
    let input = read_to_string("input/15.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut wh = read_input(input);
    while ! wh.moves.is_empty() {
        wh.go();
    }
    wh.sum_box_gps()
}

fn part2(input: &str) -> i32 {
    let mut wh = read_input2(input);
    while ! wh.moves.is_empty() {
        wh.go2();
    }
    wh.sum_box_gps()
}
