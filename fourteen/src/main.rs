use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::{RangeInclusive, RangeFrom};
use std::convert::TryInto;
use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Mask(String),
    Mem(isize, isize)
}

struct Store {
    current_mask: String, // must only be mask
    mem: HashMap<isize,isize>,
}

impl Store {
    fn new() -> Store {
        Store {
            current_mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
            mem: HashMap::new(),
        }
    }

    fn set_mask(&mut self, mask_string: String) {
        self.current_mask = mask_string;
    }

    fn set_val(&mut self, mem: isize, val: isize) {
        let comp = self.mem.get(&mem).or(Some(&0)).expect("val should exist");
        let bin_string = format!("{:036b}", val);//self.mem.get(&mem).or(Some(&val)).expect("val should exist"));
        let bitmask_string: String = self.current_mask.clone().chars().zip(bin_string.chars()).map(|(mask,d)| {
            match mask {
                'X' => d,
                '1' => '1',
                '0' => '0',
                _ => unimplemented!("unknown mask char"),
            }
        }).collect();

        let outmask = isize::from_str_radix(&bitmask_string, 2).unwrap();
        println!("Val       {:036b} (decimal {})", comp, comp);
        println!("Mask:     {}", self.current_mask);
        println!("Mask out: {}", bitmask_string);
        let outval = outmask;// | comp;
        println!("Result:   {:036b} (decimal {})", outval, outval);
        println!("");
        //println!("Expect:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        //println!("memid: {:?} mask: {:038b} val {:038b} outval: {:038b} - {}", mem, outmask, val, outval, outval);

        self.mem.insert(mem, outmask);
    }

    fn get_sum(&self) -> isize {
        self.mem.iter().map(|(addr, v)| {
            println!("{:?}", (addr, v));
            v
        }).sum()
    }
}

fn main() {
    let mut input = get_input().expect("Error getting input");
    let mut map: Vec<String> = input.iter().map(|r| r.clone()).collect();
    let out: Vec<Op> = map.iter().map(|row| {
        let split: Vec<&str> = row.split("=").collect();
        let (left, right) = (split[0].trim(), split[1].trim());
        let op_split: Vec<&str> = left.split('[').collect();
        match op_split[0] {
            "mem" => {
                let mem = op_split[1].trim_end_matches(']').parse().expect("should be isizable");
                Op::Mem(mem, right.parse().expect("should be isizable"))
            },
            "mask" => {
                Op::Mask(right.to_string())
            },
            _ => unimplemented!("Not implemented"),
        }
    }).collect();
    
// ---

    let mut store = Store::new();

    for op in &out {
        match op {
            Op::Mem(mem, val) => {
                store.set_val(*mem, *val);
            },
            Op::Mask(mask_string) => {
                store.set_mask(mask_string.to_string());
            },
        }
    }

// too low     12740684552
// too high 10265163451010
//          10035335144067
    println!("out: {:?}", store.get_sum())
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
