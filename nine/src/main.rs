use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::RangeInclusive;

const PREAMBLE_SIZE: usize = 25;

/// Loops through all combinations of map to find the matching sum
fn find_sum(num: usize, map: &Vec<(usize, usize)>) -> Option<(usize,usize)> {
    for (i, p) in map.clone() {
        // This uses unsigned ints, so numbers greater than the product will never match
        if p >= num {
            continue;
        }
        for (j, pk) in map.clone() {
            // Indexes shouldn't match we don't want to sum both together
            if i == j {
               continue;
            }
            if p + pk == num {
                return Some((i,j));
            } 
        }
    }
    None
}

fn find_contiguous(num: usize, map: &Vec<(usize, usize)>) -> Option<usize> {
    // ignore vals that are too large
    let map: Vec<&(usize, usize)> = map.iter().filter(|(_i, n)| n<=&num).collect();
    for win_size in 2..=PREAMBLE_SIZE {
        for window in map.windows(win_size) {
            let window_sum = window.iter().fold(0, |sum, (_i,n)| sum+n);
            if window_sum == num {
                let first = window.first().expect("should have first");
                let last = window.last().expect("should have last");
                return Some(first.1 + last.1);
            }
        }
    }
    None
}

fn process(mut map: Vec<Option<usize>>) {
    let mut index = PREAMBLE_SIZE + 1;
    while let Some(n) = map.get_mut(index) {
        let num = n.expect("Should be number");
       
        let range = RangeInclusive::new(index-1-PREAMBLE_SIZE, index-1);
        let prev = map[range].iter().filter(|v| v.is_some()).map(|v| v.unwrap()).enumerate().collect();
        let res = find_sum(num, &prev);
        if res == None {
            let prev_list = map[1..index].iter().filter(|v| v.is_some()).map(|v| v.unwrap()).enumerate().collect();
            let c = find_contiguous(num, &prev_list);
            println!("No res for index: {:?} number: {:?} contig sum: {:?}", index, num, c);
            return;
        }

        index += 1;
    }
}

fn main() {
    let input = get_input().expect("Error getting input");
    let map: Vec<Option<usize>> = input.iter().map(|r| Some(r.clone().parse().unwrap())).collect();
    process(map);
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
