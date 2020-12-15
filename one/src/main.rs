use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};

const SUM: i32 = 2020;

fn main() {
    let input = get_input().expect("Error getting input");
    println!("Res: {:?}", find(input.clone()));
    println!("Res 2: {:?}", find_three(input));
}

fn get_file() -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open("input.txt")?))
}

fn get_input() -> io::Result<Vec<i32>> {
    let file = get_file()?;
    file
    .lines()
    .map(|r| r?.parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e)))
    .collect()
}


fn find(mut input: Vec<i32>) -> Option<i32> {
    while let Some(item) = input.pop() {
        let lookup = SUM - item;
        if input.contains(&lookup) {
            return Some(item * lookup)
        }
    }
    None
}

fn find_three(mut input: Vec<i32>) -> Option<i32> {
    for (i, item_i) in input.iter().enumerate() {
        for (j, item_j) in input.iter().enumerate() {
            if i == j {
                continue;
            }
            for (k, item_k) in input.iter().enumerate() {
                if k == j {
                    continue;
                }
                if item_i + item_j + item_k == SUM {
                    return Some(item_i * item_j * item_k);
                }
            }
        }
    }
    None
}
