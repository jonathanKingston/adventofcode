use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::collections::HashMap;


type Contains = HashMap<String, isize>;
type Bags = HashMap<String, Contains>;

fn main() {
    let input = get_input().expect("Error getting input");
    let iter = input.iter().peekable();
    let mut bags: HashMap<String, HashMap<String, isize>> = HashMap::new(); 

    for res in iter {
        let mut words: Vec<&str> = res.split(' ').collect();
        let bag_name: String = words.drain(0..=1).collect::<Vec<&str>>().join(" ");
        words.remove(0);
        words.remove(0);
        let mut contains: HashMap<String, isize> = HashMap::new();
        for parts in words.chunks(4) {
            let mut parts: Vec<&str> = parts.iter().copied().collect();
            let count = parts.remove(0).parse();
            // Ignore 'no' bags
            if let Ok(count) = count {
                contains.insert(parts.drain(0..=1).collect::<Vec<&str>>().join(" "), count);
            }
        }
        bags.insert(bag_name, contains);
    }
    println!("s: {:?}", bags);
    println!("Children: {}", find_children(&bags, "shiny gold".to_string()) - 1);
}

fn find_children(bags: &Bags, key: String) -> isize {
    let item = bags.get(&key);
    let mut total = 1;
    if let Some(children) = item {
        for (bag, n) in children {
            total += n * find_children(bags, bag.to_string());
        }
        return total;
    }
    total
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
