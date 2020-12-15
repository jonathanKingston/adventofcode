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
    adjacent_squares(check, i, j).iter().filter(|c| **c == '#').count() >= 4
}

/*
fn adjacent_chairs_occupied(check: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let ix: isize = x.try_into().unwrap();
    let iy: isize = y.try_into().unwrap();
    let row = check.iter().nth(x).expect("row should exist");

if y > 1 {
    let range: RangeInclusive<usize> = 0..=y-1;
    let before = get_range(0, &row.get(range).unwrap().iter().rev().copied().collect());
    if let Some(before_range) = before {
        if before_range.iter().all(|a| is_occupied(Some(a))) {
            return true;
        }
    }
}
    let range: RangeFrom<usize> = y+1..;
    let after = get_range(0, &row.get(range).unwrap().iter().copied().collect());
    if let Some(after_range) = after {
        if after_range.iter().all(|a| is_occupied(Some(a))) {
            return true;
        }
    }

/*
    row.iter().skip(x+1).take_while(|c| {
      if *c == &'#' || *c == &'L' {
        return false;
      }
      true
    });
*/
    let col = get_col(&mut check.iter(), y);
println!("col y {} x {} {:?}", iy,ix, col);
    //let after = get_range(0, &col);
    let range: RangeFrom<usize> = x+1..;
    let after = get_range(0, &col.get(range).unwrap().iter().copied().collect());
    if after.is_some() && after.expect("should be some").iter().all(|a| is_occupied(Some(a))) {
        return true;
    }
    let range: RangeInclusive<usize> = 0..=x-1;
    let before = get_range(0, &col.get(range).unwrap().iter().rev().copied().collect());
    if before.is_some() && before.expect("should be some").iter().all(|a| is_occupied(Some(a))) {
        return true;
    }
    false
}
*/

fn get_col(iter: &mut dyn Iterator<Item = &Vec<char>>, i: usize) -> Vec<char> {
    iter.map(|item| item.iter().nth(i).expect("should have row").clone()).collect()
}

fn get_range(start: isize, iter: &Vec<char>) -> Option<Vec<char>> {
   let mut vec: Vec<char> = vec![];
   if start >= 0 {
       let mut ustart: usize = start.try_into().unwrap();
       while let Some(r) = iter.get(ustart) {
//println!("s {} r {:?} vec: {:?}", start, r, vec);
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
    if let Some(right) = row.iter().nth(j+1) {
        res.push(right.clone());
    }
    
    if j >= 1 {
        if let Some(left) = row.iter().nth(j-1) {
            res.push(left.clone());
        }
    }
    
    if let Some(next_row) = check.iter().nth(i+1) {
        if j >= 1 {
            if let Some(left) = next_row.iter().nth(j-1) {
                res.push(left.clone());
            }
        }
        if let Some(down) = next_row.iter().nth(j) {
            res.push(down.clone());
        }
        if let Some(right) = next_row.iter().nth(j+1) {
            res.push(right.clone());
        }
    }
    if i >= 1 {
        if let Some(prev_row) = check.iter().nth(i-1) {
            if j >= 1 {
                if let Some(left) = prev_row.iter().nth(j-1) {
                    res.push(left.clone());
                }
            }
            if let Some(up) = prev_row.iter().nth(j) {
                res.push(up.clone());
            }
            if let Some(right) = prev_row.iter().nth(j+1) {
                res.push(right.clone());
            }
        }
    }
    res
}

fn adjacent_chairs_vacant(check: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    !adjacent_squares(check, i, j).iter().any(|c| *c == '#')
}
/*
fn adjacent_chairs_vacant(check: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let row = check.iter().nth(i).expect("row should exist");
    let next_chair = row.iter().nth(j+1);
    if !is_vacant(next_chair) {
        return false;
    }
    
    let mut prev_chair = None;
    if j >= 1 {
        prev_chair = row.iter().nth(j-1);
    }
    if !is_vacant(prev_chair) {
        return false;
    }
    
    if let Some(next_row) = check.iter().nth(i+1) {
        if !is_vacant(next_row.iter().nth(j)) {
            return false;
        }
        if !is_vacant(next_row.iter().nth(j)) {
            return false;
        }
    }
    let mut prev_row = None;
    if i >= 1 {
        prev_row = check.iter().nth(i-1);
    }
    if let Some(prev_row) = prev_row {
        if !is_vacant(prev_row.iter().nth(j)) {
            return false;
        }
    }
    true
}
*/

fn is_occupied(chair: Option<&char>) -> bool {
    !is_vacant(chair)
/*
    match chair {
        None | Some(&'#') | Some(&'.') => true,
        _ => false
    }
*/
}

fn is_vacant(chair: Option<&char>) -> bool {
    match chair {
        None | Some(&'L') | Some(&'.') => true,
        _ => false
    }
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
