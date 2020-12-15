use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::collections::{HashSet,HashMap};
use std::convert::TryFrom;

fn split_once(expr: String, split_char: char) -> Option<(String, String)> {
    let input: Vec<&str> = expr.split(split_char).collect();
    Some((input.get(0)?.to_string(), input.get(1)?.to_string()))
}

fn jmp(mut index: usize, jmp: isize) -> usize {
   if jmp < 0 {
       index -= jmp.abs() as usize;
   } else {
       index += jmp as usize;
   }
   index
}

fn run(mut map: Vec<Option<String>>) -> (usize, isize, Vec<Option<String>>) {
    let mut index: usize = 0;
    let mut acc = 0;
    let mut res = map.get_mut(index).expect("should have val");
/*
    while let Some(r) = res {
        println!("r {:?}", r);
        match r {
            Some(v) => {
                let (op, pos) = split_once(v.to_string(), ' ').expect("invalid op");
                let pos: isize = pos.parse().expect("invalid pos");
                println!("v {} {}", op, pos);
                match op.as_str() {
                    "jmp" => {
                        println!("in: {}, jmp: {}", index, pos);
                        index = jmp(index, pos);
                    },
                    "acc" => { acc += pos; index += 1; },
                    "nop" => { index += 1; },
                    _ => panic!("unknown op"),
                }
            },
            None => { res = None; continue;},
        }
        *r = None;
        res = map.get_mut(index);
    }
*/
    while let Some(r) = res {
        //println!("r {:?}", r);
        let (op, pos) = split_once(r.to_string(), ' ').expect("invalid op");
        let pos: isize = pos.parse().expect("invalid pos");
        //println!("v {} {}", op, pos);
        match op.as_str() {
            "jmp" => {
                //println!("in: {}, jmp: {}", index, pos);
                index = jmp(index, pos);
            },
            "acc" => { acc += pos; index += 1; },
            "nop" => { index += 1; },
            _ => panic!("unknown op"),
        }
        *res = None;
        res = map.get_mut(index).expect("Should have val");
    }
    (index, acc, map)
}

fn main() {
    let input = get_input().expect("Error getting input");
    let mut map: Vec<Option<String>> = input.iter().map(|r| Some(r.clone())).collect();
    //println!("{:?}", map);
    let count = map.len();
    println!("Len {:?}", count);
    let (index, acc, map) = run(map);
    // 1930
    println!("index: {} acc: {}", index, acc);
    println!("res: {:?}", map);
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
