use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::{RangeInclusive, RangeFrom};
use std::convert::TryInto;

fn main() {
    let mut input = get_input().expect("Error getting input");
    let mut map: Vec<Vec<char>> = input.iter().map(|r| r.chars().collect()).collect();
    print_grid(&map);
    while map != process(map.clone()) {
        println!("Next:");
        map = process(map);
        print_grid(&map);
    }
    let count = map.iter().fold(0, |mut acc, row| {
        acc += row.iter().filter(|char| **char == '#').count();
        acc
    });
    println!("count: {}", count);
}

fn process(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    //println!("out: {:?} ", map);
    let check_map = map.clone();
    let mut check = check_map.iter().enumerate().peekable();
    while let Some((y, row)) = check.next() {
        for (x,chair) in row.iter().enumerate() {
            if 'L' == *chair && adjacent_chairs_vacant(&check_map, y, x) {
                map[y][x] = '#';
            }
            if '#' == *chair && adjacent_chairs_occupied(&check_map, y, x) {
                map[y][x] = 'L';
            }
        }
    } 
    map
}

fn adjacent_chairs_occupied(check: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    adjacent_squares(check, i, j).iter().filter(|c| **c == '#').count() >= 5
}

fn get_col(iter: &mut dyn Iterator<Item = &Vec<char>>, i: usize) -> Vec<char> {
    iter.map(|item| item.iter().nth(i).expect("should have row").clone()).collect()
}

fn get_range(start: isize, iter: &Vec<char>) -> Option<Vec<char>> {
   let mut vec: Vec<char> = vec![];
   if start >= 0 {
       let mut ustart: usize = start.try_into().unwrap();
       while let Some(r) = iter.get(ustart) {
           if *r != '.' {
               vec.push(r.clone());
           }
           if vec.len() == 4 {
               return Some(vec);
           }
           ustart += 1
       }
   }
   None
}

fn adjacent_squares(check: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<char> {
    let mut res = vec![];
    let row = check.iter().nth(i).expect("row should exist");
    let mut distance = 0;
    loop {
        distance += 1;
        match row.get(j+distance) {
            Some(&'.') => continue,
            None => break,
            Some(right) => {
                res.push(right.clone());
                break;
            }
        }
    }
    
    let mut distance = 0;
    loop {
        distance += 1;
        if distance > j {
            break;
        }
        match row.get(j-distance) {
            Some(&'.') => continue,
            None => break,
            Some(left) => {
                res.push(left.clone());
                break;
            }
        }
    }
    
    let mut distance = 0;
    let mut left_found = false;
    let mut down_found = false;
    let mut right_found = false;
    loop {
        distance += 1;
        if left_found && down_found && right_found {
            break;
        }
        let next_row = match check.get(i+distance) {
            None => break,
            Some(next_row) => {
                next_row
            }
        };

        if !left_found {
            if j >= distance {
                match next_row.get(j-distance) {
                    Some(&'.') => {},
                    None => left_found = true,
                    Some(left) => {
                        left_found = true;
                        res.push(left.clone());
                    }
                }
            } else {
                left_found = true;
            }
        }
        if !down_found {
            match next_row.get(j) {
                Some(&'.') => {},
                None => down_found = true,
                Some(down) => {
                    down_found = true;
                    res.push(down.clone());
                }
            }
        }

        if !right_found {
            match next_row.get(j + distance) {
                Some(&'.') => {},
                None => right_found = true,
                Some(down) => {
                    right_found = true;
                    res.push(down.clone());
                }
            }
        }
    }


    let mut distance = 0;
    let mut left_found = false;
    let mut up_found = false;
    let mut right_found = false;
    loop {
        distance += 1;
        if left_found && up_found && right_found {
            break;
        }
        if i < distance {
            break;
        }
        let next_row = match check.get(i-distance) {
            None => break,
            Some(next_row) => {
                next_row
            }
        };

        if !left_found {
            if j >= distance {
                match next_row.get(j-distance) {
                    Some(&'.') => {},
                    None => left_found = true,
                    Some(left) => {
                        left_found = true;
                        res.push(left.clone());
                    }
                }
            } else {
                left_found = true;
            }
        }
        if !up_found {
            match next_row.get(j) {
                Some(&'.') => {},
                None => up_found = true,
                Some(up) => {
                    up_found = true;
                    res.push(up.clone());
                }
            }
        }

        if !right_found {
            match next_row.get(j + distance) {
                Some(&'.') => {},
                None => right_found = true,
                Some(down) => {
                    right_found = true;
                    res.push(down.clone());
                }
            }
        }
    }
    res
}

fn adjacent_chairs_vacant(check: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    !adjacent_squares(check, i, j).iter().any(|c| *c == '#')
}

fn print_grid(map: &Vec<Vec<char>>) {
  for row in map {
    println!("{:?}", row);
  }
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
