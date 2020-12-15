use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};

const ALONG: usize = 3;

fn main() {
    let input = get_input().expect("Error getting input");
    println!("Trees: {}", get_tree_count(input.clone(), ALONG, 1));

    let items = [
      get_tree_count(input.clone(), 1, 1),
      get_tree_count(input.clone(), 3, 1),
      get_tree_count(input.clone(), 5, 1),
      get_tree_count(input.clone(), 7, 1),
      get_tree_count(input.clone(), 1, 2),
    ];
    println!("Trees: {:?}", items);
    println!("Trees sum: {:?}", items.iter().fold(1, |a, b| a*b));
}

fn get_tree_count(input: Vec<String>, along: usize, down: usize) -> usize {
    let mut tree_count = 0;
    let mut line_i = 0;
    for (i, line) in input.iter().enumerate() {
        if i % down != 0 {
            continue;
        }
        let n = line_i * along;
        let line_count = line.chars().count();
        let items: Vec<char> = line.chars().collect();
        if Some(&'#') == items.get(n % line_count) {
            tree_count = tree_count + 1;
        }
        line_i = line_i + 1;
    }
    tree_count
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
