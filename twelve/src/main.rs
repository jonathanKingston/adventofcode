use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::{RangeInclusive, RangeFrom};
use std::convert::TryInto;

fn main() {
    let mut input = get_input().expect("Error getting input");
    let mut map: Vec<String> = input.iter().map(|r| r.clone()).collect();
    println!("Map: {:?}", map);
    process(map);
}

#[derive(Debug)]
struct Manhattan {
  x: isize,
  y: isize,
  dir: char
}
impl Manhattan {
    fn forward(&mut self, n: isize) {
        match &self.dir {
            'N' => self.y += n,
            'S' => self.y -= n,
            'E' => self.x += n,
            'W' => self.x -= n,
            _ => panic!("Unknown direction"),
        }
    }
    fn change_direction(&mut self, dir: char) {
        self.dir = dir;
    }
    fn go_direction(&mut self, dir: char, distance: isize) {
        let last_dir = self.dir;
        self.change_direction(dir);
        self.forward(distance);
        self.change_direction(last_dir);
    }
    fn rotate(&mut self, dir: char, deg: isize) {
        if deg % 90 != 0 {
            unimplemented!("Unexpected degrees input");
        }
        let mut dirs = vec!['N', 'E', 'S', 'W'];
        if dir == 'L' {
            dirs = dirs.iter().rev().copied().collect();
        }
        let cur = self.dir.clone();
        let mut c = dirs.iter().cycle().skip_while(|l| **l != cur);
        let mut rotations = deg / 90 + 1; // add extra rotation as not skipped enough
        loop {
            if rotations == 0 {
                break;
            }
            self.dir = *c.next().expect("always exists dir");
            rotations -= 1;
        }
    }
    fn get_man(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

fn process(map: Vec<String>) {
    let mut man = Manhattan { x: 0, y: 0, dir: 'E' };
    for mut direction in map {
        println!("dir: {:?}", man);
        let distance: isize = direction.split_off(1).parse().expect("num");
        let dir: char = direction.chars().next().expect("should have char");
        match dir {
            'L' | 'R' => {
                 man.rotate(dir, distance);
            },
            'E' | 'W' | 'S' | 'N' => {
                 man.go_direction(dir, distance);
            },
            'F' => man.forward(distance),
            _ => { unimplemented!("Have not implemented dir {:?}", dir); },
        }
    }
    println!("dir: {:?}", man);
    println!("man: {}", man.get_man());
}

fn get_file() -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open("input.txt")?))
}

fn get_input() -> io::Result<Vec<String>> {
    let file = get_file()?;
    file
    .lines()
    .map(|r| r?.parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e)))
    .collect()
}
