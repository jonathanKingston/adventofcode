use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
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

fn run(mut map: Vec<Option<Op>>) -> (usize, isize, Vec<Option<Op>>) {
    let mut index: usize = 0;
    let mut acc = 0;
    let len = map.len();
    let mut res = map.get_mut(index).expect("should have val");
    while let Some(r) = res {
        //println!("v {} {}", op, pos);
        match r {
            Op::jmp(pos) => {
                //println!("in: {}, jmp: {}", index, pos);
                index = jmp(index, *pos);
            },
            Op::acc(pos) => { acc += *pos; index += 1; },
            Op::nop(_) => { index += 1; },
        }
        *res = None;
        if index == len {
            return (index, acc, map);
        }
        res = map.get_mut(index).expect("Should have val");
    }
    (index, acc, map)
}

fn make_combinations(items: Vec<Option<Op>>) -> Vec<Vec<Option<Op>>> {
    let mut combinations = vec![items.clone()];
    for (i, item) in items.iter().enumerate() {
        let item = item.as_ref().expect("Should exist");
        if let Op::nop(n) = item {
            let mut new_combination = items.clone();
            let v = new_combination.get_mut(i).expect("should exist");
            *v = Some(Op::jmp(*n));
            combinations.push(new_combination);
        }
        if let Op::jmp(n) = item {
            let mut new_combination = items.clone();
            let v = new_combination.get_mut(i).expect("should exist");
            *v = Some(Op::nop(*n));
            combinations.push(new_combination);
        }
    }
    return combinations;
}

#[derive(Debug,Clone)]
enum Op {
  jmp(isize),
  nop(isize),
  acc(isize),
}

fn main() {
    let input = get_input().expect("Error getting input");
    let mut map: Vec<Option<Op>> = input.iter().map(|r| {
        let (op, pos) = split_once(r.to_string(), ' ').expect("invalid op");
        let pos: isize = pos.parse().expect("invalid pos");
        let op = match op.as_str() {
            "jmp" => Op::jmp(pos),
            "acc" => Op::acc(pos),
            "nop" => Op::nop(pos), // needed to make combinations
            _ => panic!("unknown op"),
        };
        Some(op)
    }).collect();
    let c = make_combinations(map);
    println!("{:?}", c.len());
    //println!("{:?}", map);
    for c_map in c {
        let count = c_map.len();
        let (index, acc, map) = run(c_map.clone());
        // 1930
        if index == count {
            println!("Len {:?}", count);
            println!("index: {} acc: {}", index, acc);
            println!("res: {:?}", c_map);
        }
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
