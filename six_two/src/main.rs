use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};
use std::collections::HashSet;


fn split_once(expr: String, split_char: char) -> Option<(String, String)> {
    let input: Vec<&str> = expr.split(split_char).collect();
    Some((input.get(0)?.to_string(), input.get(1)?.to_string()))
}

fn main() {
    let input = get_input().expect("Error getting input");
    let mut iter = input.iter().peekable();
    let mut group_keys: Option<HashSet<char>> = None;
    let mut groups: Vec<HashSet<char>> = vec![];

    while let Some(res) = iter.next() {
        if res != "" {
            let mut member_keys: HashSet<char> = HashSet::new();
            for letter in res.chars() {
                if let 'a'..='z' = letter {
                    member_keys.insert(letter);
                } else {
                    panic!("Invalid question.");
                }
            }
            match group_keys {
                None => { group_keys = Some(member_keys); },
                Some(g_k) => {
                    let intersection: HashSet<char> = g_k.intersection(&member_keys).map(|a| a.clone()).collect();
                    if g_k.len() < intersection.len() || member_keys.len() < intersection.len() {
                        panic!("Group/Member should never be smaller than intersection");
                    }
                    //println!("Member        {:?}\nGroup         {:?}\nIntersection: {:?}\n\n", member_keys, group_keys, intersection);
                    group_keys = Some(intersection);
                }
            }
        }
        if res == "" || None == iter.peek() {
            groups.push(group_keys.expect("need keys").clone());
            group_keys = None;
        }
    }
    if let Some(v) = group_keys {
        panic!("Should not happen");
    }
    //println!("groups: {:?} len: {}", groups, groups.len());
    let count: usize = groups.iter().fold(0, |c, g| {
        //println!("Count {}, Len: {:?} = {:?}", c, g.len(), g);
        c + g.len()
    });
    // Not 3513
    println!("count: {:?}", count);
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
