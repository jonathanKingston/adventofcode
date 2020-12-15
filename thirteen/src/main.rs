use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::{RangeInclusive, RangeFrom};
use std::convert::TryInto;

fn main() {
    let mut input = get_input().expect("Error getting input");
    let mut map: Vec<String> = input.iter().map(|r| r.clone()).collect();
    println!("Map: {:?}", map);
    let depart: usize = map.get(0).unwrap().parse().unwrap();
    let busses: Vec<usize> = map.get(1).unwrap().split(',')
                .filter(|bus| *bus != "x")
                .map(|x| x.parse().unwrap()).collect();
    println!("busses: {:?}", busses);
    let mut bus_times = busses.clone();
    let mut bus_times_closest = vec![None; bus_times.len()];
    loop {
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
    }
    println!("times: {:?}", bus_times_closest);
    let min = bus_times_closest.iter().enumerate().min_by(|(i,x),(j,y)| x.cmp(y)).expect("must have min");
    let bus_id = busses.get(min.0).expect("should have bus");
    let wait = min.1.expect("Res") - depart;
    println!("min: {:?} - {:?}", min, bus_id);
    println!("v {}", bus_id * wait);
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
