use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};

fn main() {
    let input = get_input().expect("Error getting input");
    let items = input.iter().filter(|(range, char, password)| {
      let count = password.chars().filter(|c| c == char).count();
      range.contains(&count.into())
    });
    println!("One: {:?}", items.count());

    let items = input.iter().filter(|(range, char, password)| {
      let chars: Vec<char> = password.chars().collect();
      let a = chars.get(range.start-1);
      let b = chars.get(range.end-2);
      (a == Some(char) || b == Some(char)) && a != b
    });
    println!("Two: {:?}", items.count());
}

fn get_file() -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open("input.txt")?))
}

fn get_input() -> io::Result<Vec<(core::ops::Range<usize>, char, String)>> {
    let file = get_file()?;
    let output = file
    .lines()
    .map(|r| { 
      // Ugly but I'm being lazy
      let item = r.unwrap();
      let row: Vec<&str> = item.split(' ').collect();
      let char = row.get(1).unwrap().chars().next().unwrap();
      let range: Vec<usize> = row.get(0).unwrap().split('-').map(|i| i.parse().unwrap()).collect();
      let range = std::ops::Range {
          start: range.get(0).unwrap().clone(),
          end: range.get(1).unwrap().clone() + 1
      };
      (range, char, String::from(row.get(2).unwrap().clone()))
    })
    .collect();
    Ok(output)
}

