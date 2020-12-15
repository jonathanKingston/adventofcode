use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::ops::RangeInclusive;
use std::convert::TryInto;

use num::traits::{Zero, One};
use num::bigint::{BigInt, ToBigInt};

fn main() {
    let mut input = get_input().expect("Error getting input");
    let mut map: Vec<usize> = input.iter().map(|r| r.clone().parse().unwrap()).collect();
    map.push(0); // charger port
    map.sort();
    let wins = map.windows(2);
    println!("{:?}", wins);
    // initialise vec with 3 placement as the device is always 3 bigger than max
    let res: Vec<isize> = wins.map(|a| a[1]-a[0]).fold(vec![Zero::zero(),Zero::zero(),One::one()], |mut acc, b| {
        println!("{}", b);
        acc[b-1] += 1;
        acc
    });


    //let sums: Vec<usize> = wins.map(|a| a[1]-a[0]).collect();
    println!("{:?} - {:?}", res, res[0] * res[2]);



    println!("{:?} - {:?}", res, factorial(res[0]));// + One::one());
    let r = res[0]; 
    let n = 3;
    let combinations = factorial(r) / (factorial(n) * factorial(r - n));
    // Not 2276
    println!("com: {:?}", combinations.clone()-2);
/*
    for win in wins {
        println!("s {:?}", win);
    }
*/
}
fn factorial(x: isize) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..(x+1) {
            factorial = factorial * i;
        }
        factorial
    }
    else {
        panic!("Failed to calculate factorial!");
    }
}
/*
fn factorial(j: BigUint) -> BigUint {
   let mut acc: BigUint = One::one();
   let j: BigUint = One::one();
   let mut i: BigUint = j+j;
   while i <= j {
       acc = acc * i;
       i += j.clone();
   }
   j
}
*/

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
