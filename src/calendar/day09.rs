use std::fs::read_to_string;

#[derive(Debug,PartialEq)]
enum Block {
    Free,
    File(i64),
}

pub fn solve() {
    let input = read_to_string("input/9.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn read_input(input: &str) -> Vec<Block> {
    let mut free = false;
    let mut id = 0;
    let mut ret = vec![];
    for c in input.chars() {
        if ! c.is_numeric() { continue; }
        let n : u8 = c.to_string().parse().unwrap();
        for _ in 0..n {
            if free {
                ret.push(Block::Free);
            } else {
                ret.push(Block::File(id));
            }
        }
        free = ! free;
        if free { id += 1 }
    }
    ret
}

fn part1(input: &str) -> i64 {
    let mut disk = read_input(input);
    let mut fwd = 0;
    let mut rev = disk.len() - 1;

    loop {
        while let Block::File(_) = disk[fwd] { fwd += 1 }
        while Block::Free == disk[rev] { rev -= 1 }
        if fwd >= rev { break }
        disk.swap(fwd,rev);
        // println!("fwd: {fwd} rev: {rev}");
        // println!("{disk:?}");
    }
    checksum(disk)
}

fn checksum(disk: Vec<Block>) -> i64 {
    let mut ret = 0;
    for (i, b) in disk.iter().enumerate() {
        match b {
            Block::Free => continue,
            Block::File(j) => ret += (i as i64) * *j,
        }
    }
    ret
}

fn part2(input: &str) -> i64 {
    let mut disk = read_input(input);
    let mut begin = 0;
    let mut backend = disk.len() - 1;
    
    loop {
        while let Block::File(_) = disk[begin] { begin += 1 }
        if backend <= begin { break }
        while Block::Free == disk[backend] { backend -= 1 }
        let mut backstart = backend;
        let mut fid : i64 = -1;
        while let Block::File(i) = disk[backstart] {
            if fid == -1 { fid = i }
            if fid != i { break; }
            backstart -= 1;
        }
        backstart += 1;
        let flen = backend - backstart + 1;

        let mut contig = 0;
        let mut fwd = begin;
        while fwd < backstart {
            match disk[fwd] {
                Block::Free => contig += 1,
                Block::File(_) => contig = 0,
            }
            fwd += 1;
            if contig >= flen { break }
        }

        // println!("{begin} {backstart} {backend} {contig}"); 
        // println!("{fwd} {fid} {flen}");
        // println!("{disk:?}");

        if contig >= flen {
            println!("Swapping! {fid}");
            for i in 0..flen {
                disk.swap(fwd - contig + i, backstart + i); 
            }
        } else {
            println!("Cannot find full space! {fid}");
            backend = backstart - 1;
            continue
        }
        
    }
    checksum(disk)
}
