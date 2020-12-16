use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    let input = [0,3,1,6,7,5];
    let start = Instant::now();
    println!("answer: {} elapsed: {:?}", calc(&input, 2020), start.elapsed());


    let n = 30000000;
    let start = Instant::now();
    println!("answer: {}, time: {:?}", calc(&input, n), start.elapsed());
}

#[derive(Debug)]
struct ItemData {
    last: Option<usize>,
    before: Option<usize>,
//    count: usize,
}
impl ItemData {
    fn new(index: usize) -> ItemData {
        ItemData {
            last: Some(index),
            before: None,
       //     count: 1
        }
    }

    fn get_num(&self) -> usize {
        match self.before {
            Some(before) => { self.last.expect("should always be unwrappable") - before },
            None => 0
        }
    }

    fn set_index(&mut self, index: usize) {
        self.before = self.last;
        //self.count += 1;
        self.last = Some(index);
    }
}

fn calc(input: &[usize], limit: usize) -> usize {
    let mut hash: HashMap<usize, ItemData> = HashMap::new();
    let mut last_num = 0;
    // input num start at 1 so make this non inclusive
    for index in 0..limit {
        let num = if index < input.len() {
            input[index]
        } else {
            hash.get(&last_num).expect("last num should exist").get_num()
        };
        match hash.get_mut(&num) {
            Some(ref mut hash_items) => {
                hash_items.set_index(index);
            },
            None => {
                hash.insert(num, ItemData::new(index));
            },
        }
        last_num = num;
    }
    last_num
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(calc(&[0,3,6], 2020), 436);
        assert_eq!(calc(&[1,3,2], 2020), 1);
        assert_eq!(calc(&[2,1,3], 2020), 10);
        assert_eq!(calc(&[1,2,3], 2020), 27);
        assert_eq!(calc(&[2,3,1], 2020), 78);
        assert_eq!(calc(&[3,2,1], 2020), 438);
        assert_eq!(calc(&[3,1,2], 2020), 1836);
    }

    #[test]
    fn test_big() {
        let n = 30000000;
        assert_eq!(calc(&[0,3,6], n), 175594);
        assert_eq!(calc(&[1,3,2], n), 2578);
        assert_eq!(calc(&[2,1,3], n), 3544142);
        assert_eq!(calc(&[1,2,3], n), 261214);
        assert_eq!(calc(&[2,3,1], n), 6895259);
        assert_eq!(calc(&[3,2,1], n), 18);
        assert_eq!(calc(&[3,1,2], n), 362);
    }
}
