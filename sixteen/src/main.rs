use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::ops::RangeInclusive;

type ParseResult = (Vec<(String, Field)>, Vec<isize>, Vec<Vec<isize>>);
type Field = Vec<RangeInclusive<isize>>;

fn main() {
    let input = get_input().expect("Error getting input");
    let map: Vec<String> = input.iter().map(|r| r.clone()).collect();
    let (fields, my_ticket, other_tickets) = parse_data(map.join("\n"));

    let d = error_rate((fields, my_ticket, other_tickets));
    println!("error rate: {:?}", d);

    // Part 2
    let (fields, my_ticket, other_tickets) = parse_data(map.join("\n"));

    let d = get_fields((fields, my_ticket.clone(), other_tickets));

    let product: isize = my_ticket
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let key = d.get(&i).unwrap();
            (key, v)
        })
        .filter(|(k, _v)| k.starts_with("departure"))
        .map(|(_k, v)| v)
        .product();
    println!("product: {:?}", product);
}

fn parse_data(input_string: String) -> ParseResult {
    let r: Vec<Vec<String>> = input_string
        .trim()
        .split('\n')
        .map(|a| a)
        .collect::<Vec<&str>>()
        // Split out section data
        .split(|a| *a == "")
        .map(|a| a.iter().map(|b| b.to_string()).collect())
        .collect();

    let fields = &r[0];
    let my_ticket = r[1][1]
        .split(',')
        .map(|a| a.parse().expect("should be valid num"))
        .collect();
    let other_tickets = r[2][1..]
        .iter()
        .map(|a| {
            a.split(',')
                .map(|b| b.parse().expect("Should be valid num"))
                .collect()
        })
        .collect();
    let out: Vec<(String, Vec<RangeInclusive<isize>>)> = fields
        .iter()
        .map(|f| {
            let (field, val) = f.split_at(f.find(':').expect("fields should have : char") + 1);
            let vals = parse_field_val(val);
            (field.trim_end_matches(':').to_string(), vals)
        })
        .collect();

    (out, my_ticket, other_tickets)
}

fn parse_field_val(val: &str) -> Field {
    val.trim()
        .split(" or ")
        .map(|part| {
            let split: Vec<isize> = part
                .split("-")
                .map(|v| v.parse().expect("should be valid number"))
                .collect();
            RangeInclusive::new(split[0], split[1])
        })
        .collect()
}

fn potentially_valid_fields(
    ticket_field: &isize,
    fields: &Vec<(String, Field)>,
) -> HashSet<String> {
    let mut out: HashSet<String> = HashSet::new();
    for (field, ranges) in fields.iter() {
        for range in ranges {
            if range.contains(&ticket_field) {
                out.insert(field.to_string());
            }
        }
    }
    out
}

fn is_potentially_valid(ticket: &Vec<isize>, fields: &Vec<(String, Field)>) -> Option<Vec<isize>> {
    let mut invalid: Vec<isize> = vec![];
    for ticket_field in ticket {
        if potentially_valid_fields(ticket_field, fields).len() == 0 {
            invalid.push(*ticket_field);
        }
    }
    if invalid.len() == 0 {
        None
    } else {
        Some(invalid)
    }
}

fn get_fields(parse_res: ParseResult) -> HashMap<usize, String> {
    let (fields, my_ticket, other_tickets) = parse_res;

    let mut tickets = other_tickets.clone();
    tickets.push(my_ticket);

    let valids: Vec<HashMap<usize, HashSet<String>>> = tickets
        .iter()
        .filter(|other| is_potentially_valid(other, &fields).is_none())
        .map(|ticket| {
            ticket
                .iter()
                .enumerate()
                .map(|(i, v)| (i, potentially_valid_fields(v, &fields)))
                .collect()
        })
        .collect();

    // need fold first from nightly!
    let mut valids = valids.iter();
    let first = valids.next().expect("has first");

    let mut vec: HashMap<usize, HashSet<String>> = valids.fold(first.clone(), |a, b| {
        a.iter()
            .map(|(index, vals)| {
                let set: HashSet<String> = vals
                    .intersection(b.get(index).expect("val there"))
                    .cloned()
                    .collect();
                (*index, set)
            })
            .collect()
    });

    let mut singles = vec![];
    let mut has_non_singles = true;

    // Keep looping until each field is unique
    loop {
        if !has_non_singles {
            break;
        }
        has_non_singles = false;

        for (_i, set) in &mut vec {
            if set.len() == 1 {
                let single = set.iter().next().unwrap();
                singles.push(single.clone());
            } else {
                has_non_singles = true;
                for single in &singles {
                    set.remove(single);
                }
            }
        }
    }

    // Remove the hashset from the vec as we have one now
    vec.into_iter()
        .map(|(i, v)| (i, v.iter().next().expect("Must have one field").to_string()))
        .collect()
}

fn error_rate(parse_res: ParseResult) -> isize {
    let (fields, _my_ticket, other_tickets) = parse_res;
    let invalid_fields: isize = other_tickets
        .iter()
        .map(|other| is_potentially_valid(other, &fields))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .flatten()
        .sum();
    invalid_fields
}

fn get_file() -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open("input.txt")?))
}

fn get_input() -> io::Result<Vec<String>> {
    let file = get_file()?;
    file.lines()
        .map(|r| {
            r?.parse()
                .map_err(|e| Error::new(ErrorKind::InvalidInput, e))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        let test_input = r#"
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"#;
        assert_eq!(error_rate(parse_data(test_input.to_string())), 71);
    }

    #[test]
    fn test_fields() {
        let test_input = r#"
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
"#;
        let d = get_fields(parse_data(test_input.to_string()));
        println!("fields: {:?}", d);
    }
}
