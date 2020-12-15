use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::RangeInclusive;

fn main() {
    let mut input = get_input().expect("Error getting input");
    let mut map: Vec<usize> = input.iter().map(|r| r.clone().parse().unwrap()).collect();
    map.push(0); // charger port
    map.sort();
    let mut wins = map.iter().enumerate().peekable();
    println!("{:?}", wins);
    // initialise vec with 3 placement as the device is always 3 bigger than max
    let mut acc = 0;
    while let Some((i, b)) = wins.next() {
        println!("{:?}", b);
        //acc[b-1] += 1;
        if let Some((j, next)) = wins.peek() {
          if *next - b == 1 {
              //let me = wins.peek();
              acc += 2;
              //println!("s {:?}", me);
          }
        }
        acc += 1;
    }

    println!("out: {:?} ", acc);//, res[0] * res[2]);
}

fn get_file() -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open("data.txt")?))
}

fn get_input() -> io::Result<Vec<String>> {
    let file = get_file()?;
    file
    .lines()
    .map(|r| r?.parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e)))
    .collect()
}
