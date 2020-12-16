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
        //let comp = self.mem.get(&mem).or(Some(&0)).expect("val should exist");
        let bin_string = format!("{:036b}", mem);
println!("Address:  {}", bin_string);
        let bitmask_string: String = self.current_mask.clone().chars().zip(bin_string.chars()).map(|(mask,d)| {
            match mask {
                'X' => 'X',
                '1' => '1',
                '0' => d,
                _ => unimplemented!("unknown mask char"),
            }
        }).collect();
println!("mask:     {}", bitmask_string);

        let nums: Vec<Vec<(usize,char)>> = bitmask_string.chars().enumerate().filter(|(i, v)| *v == 'X').map(|(i, c)| vec![(i, '0'),(i, '1')]).collect();
        let mut expand: Vec<String> = vec![bitmask_string.clone()];
        for num_com in &nums {
            let mut expander: Vec<String> = vec![]; 
            for expand_string in &expand {
                for (index, v) in num_com {
                    println!("s {:?}", (index,v));
                    let mut string_vec: Vec<char> = expand_string.chars().collect();
                    if let Some(val) = string_vec.get_mut(*index) {
                        *val = *v;
                    }
                    
                    expander.push(string_vec.iter().collect())
                }
            }
            expand = expander;
        }

        let mut mem = mem;
        for mem_addr in expand {
            let outmask = isize::from_str_radix(&mem_addr, 2).unwrap();

            println!("Result:   {:036b} (decimal {}) (addr {})", outmask, outmask, mem);
            self.mem.insert(outmask, val);
            mem += 1;
        }
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
