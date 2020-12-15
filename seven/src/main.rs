use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::collections::{HashSet,HashMap};

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
    let mut leaves = find_leaves(&bags, "shiny gold".to_string());
    let mut total_leaves: HashSet<String> = HashSet::new();
    while !leaves.is_empty() {
        let mut leaves_res = HashSet::new();
        for leaf in leaves {
            total_leaves.insert(leaf.clone());
            let leaves_sub = find_leaves(&bags, leaf.to_string());
            leaves_res = leaves_res.union(&leaves_sub).copied().collect();
        } 
        leaves = leaves_res;
    }
    println!("s: {:?}", total_leaves.len());
}

fn find_leaves(bags: &Bags, key: String) -> HashSet<&String> {
    let mut leaves = HashSet::new();
    for (bag_name, contains) in bags {
        if contains.contains_key(&key) {
            leaves.insert(bag_name);
        }
    }
    leaves
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
