use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::collections::HashSet;


fn split_once(expr: String, split_char: char) -> Option<(String, String)> {
    let input: Vec<&str> = expr.split(split_char).collect();
    Some((input.get(0)?.to_string(), input.get(1)?.to_string()))
}

fn main() {
    let input = get_input().expect("Error getting input");
    let mut iter = input.iter().peekable();
    let mut group_keys: HashSet<char> = HashSet::new();
    let mut groups: Vec<HashSet<char>> = vec![];

    while let Some(res) = iter.next() {
        if res != "" {
            for letter in res.chars() {
                group_keys.insert(letter);
            }
        }
        if res == "" || None == iter.peek() {
            groups.push(group_keys.clone());
            group_keys = HashSet::new();
        }
    }
    println!("groups: {:?}", groups);
    let count: Vec<usize> = groups.iter().map(|g| g.len()).collect();
    println!("count: {:?}", count.iter().fold(0, |r,x| r+x));
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
