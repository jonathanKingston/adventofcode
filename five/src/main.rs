use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::RangeInclusive;
use std::convert::TryFrom;
use std::collections::HashMap;

const ROW_COUNT: RangeInclusive<isize> = 0..=127;
const COL_COUNT: RangeInclusive<isize> = 0..=7;

fn main() {
    let input = get_input().expect("Error getting input");
    println!("Out: {:?}", input);
    let mut seats: HashMap<isize, (isize, isize)> = HashMap::new();
    for seat in input {
        let (rows, cols) = seat.split_at(7);
        println!("seat: {:?} {:?}", rows, cols);
        let row = find_binary(ROW_COUNT, rows.into()).expect("should have row");
        let col = find_binary(COL_COUNT, cols.into()).expect("should have col");
        let seat_id = row*8 + col;
        seats.insert(seat_id, (row, col));
    }
    println!("seats: {:?}", seats);
    let max = seats.iter().map(|(k,v)| k).max().expect("Must have max seat");
    println!("seat max: {:?}", max);
    for seat_id in RangeInclusive::new(0, *max) {
        if None == seats.get(&seat_id) {
            println!("Missing: {}", seat_id);
        }
    }
}

fn find_binary(mut range: RangeInclusive<isize>, bin_letters: String) -> Option<isize> {
    for letter in bin_letters.chars() {
        println!("L {:?}", letter);
        let range_length = range.end() - range.start();
        let mut diff: isize = range_length / 2;
        if range_length % 2 > 0 {
            diff = diff + 1;
        }
        match letter {
            'B' | 'R' => {
                range = RangeInclusive::new(range.start() + diff, *range.end());
            },
            'F' | 'L' => {
                range = RangeInclusive::new(*range.start(), range.end() - diff);
            },
            _ => return None,
        }
    }
    if range.start() == range.end() {
        Some(*range.start())
    } else {
        None
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
