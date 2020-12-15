use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::collections::HashMap;

const YEAR: usize = 2020;

#[derive(Debug)]
struct Passport {
    byr: String, // (Birth Year)
    iyr: String, // (Issue Year)
    eyr: String, // (Expiration Year)
    hgt: String, // (Height)
    hcl: String, // (Hair Color)
    ecl: String, // (Eye Color)
    pid: String, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

fn split_once(expr: String, split_char: char) -> Option<(String, String)> {
    let input: Vec<&str> = expr.split(split_char).collect();
    Some((input.get(0)?.to_string(), input.get(1)?.to_string()))
}

fn is_passport(keys: HashMap<String, String>) -> Option<Passport> {
   let cid = keys.get("cid").map(|v| v.into());
   Some(Passport {
       byr: keys.get("byr")?.to_string(),
       iyr: keys.get("iyr")?.to_string(),
       eyr: keys.get("eyr")?.to_string(),
       hgt: keys.get("hgt")?.to_string(),
       hcl: keys.get("hcl")?.to_string(),
       ecl: keys.get("ecl")?.to_string(),
       pid: keys.get("pid")?.to_string(),
       cid,
   })
}

fn main() {
    let mut items: Vec<Option<Passport>> = vec![];
    let input = get_input().expect("Error getting input");
    let mut iter = input.iter().peekable();
    let mut keys: HashMap<String, String> = HashMap::new();

    while let Some(res) = iter.next() {
        if res != "" {
            for part in res.split(' ') {
                let (key, val) = split_once(part.into(), ':').expect("should have key val format");
                keys.insert(key, val);
            }
        }
        if res == "" || None == iter.peek() {
            items.push(is_passport(
                keys.clone(),
            ));
            keys = HashMap::new();
        }
    }
    println!("Out: {:?}", items);
    println!("Count: {}", items.iter().filter(|i| i.is_some()).count());
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
