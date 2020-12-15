use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};



use num::traits::{Zero};


use modinverse::modinverse;

fn main() {
    let input = get_input().expect("Error getting input");
    let map: Vec<String> = input.iter().map(|r| r.clone()).collect();
    println!("Map: {:?}", map);
    let busses: Vec<Option<i128>> = map.get(1).unwrap().split(',')
                .map(|x| x.parse().ok()).collect();
    println!("busses: {:?}", busses);
    let _bus_times = busses.clone();
    // loop through
    // peek. minus offset
    // see if it matches the num
    let (max_i, max) = busses.iter().enumerate().max_by(|(_i, a), (_j, b)| a.cmp(b)).unwrap();
    let _max_i = max_i as i128;
    let max = max.expect("max expected");
    let (first_i, first) = busses.iter().enumerate().next().unwrap();
    let _first_i = first_i as i128;
    let first = first.expect("first expected");
    println!("max {:?} first {:?}", max, first);
    let mut n: i128 = Zero::zero();
    let enumerate = busses.iter().enumerate().filter(|(_i,bus)| bus.is_some()).map(|(i,bus)| (i as i128, bus.unwrap()));
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    // Multiply entire set together
    let prod: i128 = enumerate.clone().map(|(_i,bus)| bus).product();
    let out = enumerate.clone().map(|(i, bus)| (bus, bus-i));
    println!("prod: {} enum: {:?}", prod, out.clone().collect::<Vec<(i128, i128)>>());
    let sum: Option<i128> = out.map(|(bus, res)| {
        // P is the number of bus's that go into the prod.
        let p = prod / bus;
        modinverse(p, bus).map(|a| {
            res * a * p
        })
    }).sum();
    println!("sum: {:?}", sum);
    let sum = sum.map(|s| s%prod);
    println!("sum: {:?}", sum);
    // ^ the answer
n = sum.unwrap();
    
println!("--- verify:");
        let matched = busses.iter().enumerate().all(|(i, bus)| {
            let i = i as i128;
            if let Some(bus) = bus {
                let x = n + i;
println!("i: {} n: {} x: {} rem: {} bus: {}", i, n, x, x.clone()%bus, bus);
                // If it's divisible by the bus number it's arrived
                x % bus == Zero::zero()
            } else {
               true
            }
        });
        if matched {
        println!("match {:?}  n: {}", matched, n);
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
