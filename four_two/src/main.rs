use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::collections::HashMap;

const YEAR: usize = 2020;

#[derive(Debug)]
struct Passport {
    byr: usize, // (Birth Year)
    iyr: usize, // (Issue Year)
    eyr: usize, // (Expiration Year)
    hgt: (usize, String), // (Height)
    hcl: String, // (Hair Color)
    ecl: String, // (Eye Color)
    pid: String, // (Passport ID)
    cid: Option<usize>, // (Country ID)
}

fn split_once(expr: String, split_char: char) -> Option<(String, String)> {
    let input: Vec<&str> = expr.split(split_char).collect();
    Some((input.get(0)?.to_string(), input.get(1)?.to_string()))
}

fn is_passport(keys: HashMap<String, String>) -> Option<Passport> {
   let cid = keys.get("cid").map(|v| v.parse().unwrap());
   let pid: String = keys.get("pid")?.parse().ok()?;
   let byr = keys.get("byr")?.parse().ok()?;
   let iyr = keys.get("iyr")?.parse().ok()?;
   let eyr = keys.get("eyr")?.parse().ok()?;
   if byr < 1920 || byr > 2002 {
       return None;
   }
   if eyr < 2020 || eyr > 2030 {
       return None;
   }
   if iyr < 2010 || iyr > 2020 {
       return None;
   }
   let ecl = match keys.get("ecl")?.as_str() {
       "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => keys.get("ecl")?.to_string(),
       _ => return None,
   };
   if pid.len() != 9 {
       return None;
   }
   let hgt = keys.get("hgt")?.to_string();
   let hgt = if let Some(hgt) = hgt.strip_suffix("cm") {
       let unit = hgt.parse().ok()?;
       if unit < 150 || unit > 193 {
           return None;
       }
       (unit, "cm".into())
   } else if let Some(hgt) = hgt.strip_suffix("in") {
       let unit = hgt.parse().ok()?;
       if unit < 59 || unit > 76 {
           return None;
       }
       (unit, "in".into())
   } else {
       return None;
   };
   let hcl = keys.get("hcl")?.to_string();
   if hcl.len() != 7 {
       return None;
   }
   let mut hcl_chars = hcl.chars();
   hcl_chars.next();
   for c in hcl_chars {
       match c {
           '0'..='9' | 'a'..='f'=> {},
           _ => return None,
       }
   }
   Some(Passport {
       byr,
       iyr,
       eyr,
       hgt,
       hcl,
       ecl,
       pid,
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
