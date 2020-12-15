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
  waypoint_x: isize,
  waypoint_y: isize,
}
impl Manhattan {
    fn forward(&mut self, n: isize) {
        self.y += self.waypoint_y * n;
        self.x += self.waypoint_x * n;
    }
    fn move_waypoint(&mut self, dir: char, n: isize) {
        match dir {
            'N' => self.waypoint_y += n,
            'S' => self.waypoint_y -= n,
            'E' => self.waypoint_x += n,
            'W' => self.waypoint_x -= n,
            _ => panic!("Unknown direction"),
        }
    }
    fn rotate(&mut self, dir: char, deg: isize) {
        if deg % 90 != 0 {
            unimplemented!("Unexpected degrees input");
        }
        let mut rotations = deg / 90;
        if dir == 'R' {
            for _ in 1..=rotations {
                let fx = self.waypoint_x;
                let fy = self.waypoint_y;
                self.waypoint_x = fy;
                self.waypoint_y = -fx;
            }
        } else if dir == 'L' {
            for _ in 1..=rotations {
                let fx = self.waypoint_x;
                let fy = self.waypoint_y;
                self.waypoint_x = -fy;
                self.waypoint_y = fx;
            }
        }
//          172685
//          77089
//          113449
// too low  26629
// too high 270013
// wrong 46753
// 47883
    }
    fn get_man(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

fn process(map: Vec<String>) {
    let mut man = Manhattan { x: 0, y: 0, waypoint_x: 10, waypoint_y: 1 };
    for mut direction in map {
        println!("dir: {:?}", man);
        let distance: isize = direction.split_off(1).parse().expect("num");
        let dir: char = direction.chars().next().expect("should have char");
        match dir {
            'L' | 'R' => {
                 man.rotate(dir, distance);
            },
            'E' | 'W' | 'S' | 'N' => {
                 man.move_waypoint(dir, distance);
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
