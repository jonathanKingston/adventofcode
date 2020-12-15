use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::{RangeInclusive, RangeFrom};
use std::convert::TryInto;

use num::traits::{Zero, One};
use num::bigint::{BigInt, ToBigInt};

fn main() {
    let mut input = get_input().expect("Error getting input");
    let mut map: Vec<String> = input.iter().map(|r| r.clone()).collect();
    println!("Map: {:?}", map);
    let busses: Vec<Option<u128>> = map.get(1).unwrap().split(',')
                .map(|x| x.parse().ok()).collect();
    println!("busses: {:?}", busses);
    let mut bus_times = busses.clone();
    // loop through
    // peek. minus offset
    // see if it matches the num
    let (max_i, max) = busses.iter().enumerate().max_by(|(i, a), (j, b)| a.cmp(b)).unwrap();
    let max = max.expect("max expected");
    let (first_i, first) = busses.iter().enumerate().next().unwrap();
    let first = first.expect("first expected");
    println!("max {:?} first {:?}", max, first);
    let mut n: BigInt = Zero::zero();
    
//    n = ((target / max) * max) - max;
    loop {
        n += max;

println!("---");
        let matched = busses.iter().enumerate().all(|(i, bus)| {
            if let Some(bus) = bus {
                let x = if max_i > i {
                    let diff = max_i - i;
                    // Where the bus lies in the match sequence is i.
                    n.clone() - diff
                } else if max_i < i {
                    let diff = i - max_i;
                    // Where the bus lies in the match sequence is i.
                    n.clone() + diff
                } else {
                    n.clone()
                };
println!("i: {} n: {} x: {} rem: {} bus: {}", i, n, x, x.clone()%bus, bus);
                // If it's divisible by the bus number it's arrived
                x % bus == Zero::zero()
            } else {
               true
            }
        });
        if matched {
        println!("match {:?}  max N: {} n: {}", matched, n, n.clone() - (max_i-first_i));
            break;
        }
/*
        for (i, bus) in busses.iter().rev().enumerate() {
            if let Some(bus) = bus {
let x = n - i;
if x % bus == 0 {
// Skip out of this loop as it's invalid.
  break;
}
                let diff = max - bus;
                let bus_time = n - diff;
                println!("bus: {} {:?} diff {} bus time: {}", i, bus, diff, bus_time);
            }
        }
*/
/*
        if n > 100000000000000_000000 {
            break;
        }
*/
/* data.txt debug:
        if n == 1068785 {
        println!("match {:?}  max N: {} n: {}", matched, n, n - (max_i-first_i));
            break;
        }
*/
/*
        for (i, bus) in busses.iter().enumerate() {
            if let Some(bus_time) = bus_times.get_mut(i) {
                if *bus_time < depart {
            println!("bus {} {}", bus, bus_time);
                    *bus_time = *bus_time + bus;
                } else {
                    bus_times_closest[i] = Some(bus_time.clone());
                }
            }
        }
        if bus_times_closest.iter().all(|x| x.is_some()) {
            break;
        }
*/
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
